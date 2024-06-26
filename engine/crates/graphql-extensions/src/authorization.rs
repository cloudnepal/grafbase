//! ----------------------------------------------------------------------------
//! The Auth is going to be injected inside engine instead of just living as an
//! Extension as it's adding complexity without much gain.
//! ----------------------------------------------------------------------------
use std::sync::Arc;

use common_types::auth::{ExecutionAuth, Operations};
use engine::{
    extensions::{Extension, ExtensionContext, ExtensionFactory, NextResolve, ResolveInfo},
    graph_entities::ResponseNodeId,
    registry::NamedType,
    AuthConfig, ServerError, ServerResult,
};
use engine_value::ConstValue;

const INPUT_ARG: &str = "input";
const MUTATION_TYPE: &str = "Mutation";

/// Authorization extension
///
/// This extension will check that the user is authorized to execute the GraphQL operation.
pub struct AuthExtension {
    trace_id: String,
}

impl ExtensionFactory for AuthExtension {
    fn create(&self) -> Arc<dyn Extension> {
        Arc::new(AuthExtension::new(self.trace_id.clone()))
    }
}

// Use ExecutionAuth from ctx and AuthConfig from ResolveInfo (global) or MetaField  to allow/deny the request.
#[async_trait::async_trait]
impl Extension for AuthExtension {
    /// Called at prepare request.
    async fn prepare_request(
        &self,
        ctx: &ExtensionContext<'_>,
        request: engine::Request,
        next: engine::extensions::NextPrepareRequest<'_>,
    ) -> ServerResult<engine::Request> {
        let auth_context = ctx
            .data::<ExecutionAuth>()
            .expect("auth must be injected into the context");
        let request = if auth_context.is_introspection_allowed()
            || request.introspection_state() == engine::IntrospectionState::ForceEnabled
        {
            request
        } else {
            request.set_introspection_state(engine::IntrospectionState::ForceDisabled)
        };
        next.run(ctx, request).await
    }

    async fn resolve(
        &self,
        ctx: &ExtensionContext<'_>,
        info: ResolveInfo<'_>,
        next: NextResolve<'_>,
    ) -> ServerResult<Option<ResponseNodeId>> {
        if info.parent_type.starts_with("__") || info.parent_type.starts_with("[__") || info.name.starts_with("__") {
            return next.run(ctx, info).await;
        }

        let execution_auth = ctx
            .data::<ExecutionAuth>()
            .expect("auth must be injected into the context");
        let auth_fn = |auth: Option<&AuthConfig>, default_ops: Operations| {
            auth.map(|auth| match execution_auth {
                ExecutionAuth::ApiKey => common_types::auth::API_KEY_OPS,
                ExecutionAuth::Token(token) => auth.private_public_and_group_based_ops(token.groups_from_token()),
                ExecutionAuth::Public { .. } => auth.allowed_public_ops,
            })
            .unwrap_or(default_ops)
        };
        // Get the allowed operation from the parsed schema.
        let model_allowed_ops = auth_fn(info.auth, execution_auth.global_ops()); // Fall back to global auth if model auth is not configured
        tracing::trace!(
            "Resolving {parent_type}.{name}, auth: {auth:?} allowed ops as {model_allowed_ops:?}, required {required_op:?}",
            parent_type = info.parent_type,
            name = info.name,
            auth = info.auth,
            required_op = info.required_operation
        );

        // Required operation is inferred from the current request.
        if let Some(required_op) = info.required_operation {
            if !model_allowed_ops.contains(required_op) {
                let msg = format!(
                    "Unauthorized to access {parent_type}.{name} (missing {required_op} operation)",
                    parent_type = info.parent_type,
                    name = info.name
                );
                tracing::warn!("{msg} auth={auth:?}", auth = info.auth);
                return Err(ServerError::new(msg, None));
            }

            match (info.parent_type, required_op) {
                (MUTATION_TYPE, Operations::CREATE | Operations::UPDATE) => {
                    let input = info
                        .input_values
                        .iter()
                        .find_map(|(name, val)| val.as_ref().filter(|_| name.as_str() == INPUT_ARG))
                        .unwrap_or(&ConstValue::Null);

                    if let Some(type_name) = guess_batch_operation_type_name(&info, required_op) {
                        let inputs = match input {
                            obj @ ConstValue::Object(_) => vec![obj],
                            ConstValue::List(objs) => objs.iter().collect(),
                            _ => vec![],
                        };
                        for input in inputs {
                            self.check_input(CheckInputOptions {
                                input: match input {
                                    ConstValue::Object(args) => args.get(INPUT_ARG),
                                    _ => None,
                                }
                                .unwrap_or(&ConstValue::Null),
                                type_name: type_name.clone(),
                                mutation_name: info.name,
                                registry: &ctx.schema_env.registry,
                                required_op,
                                model_allowed_ops,
                                auth_fn: &auth_fn,
                            })?;
                        }
                    } else {
                        self.check_input(CheckInputOptions {
                            input,
                            type_name: guess_type_name(&info, required_op),
                            mutation_name: info.name,
                            registry: &ctx.schema_env.registry,
                            required_op,
                            model_allowed_ops,
                            auth_fn: &auth_fn,
                        })?;
                    }
                }
                (MUTATION_TYPE, Operations::DELETE) => {
                    self.check_delete(
                        guess_batch_operation_type_name(&info, required_op)
                            .unwrap_or_else(|| guess_type_name(&info, required_op)),
                        info.name,
                        &ctx.schema_env.registry,
                        model_allowed_ops,
                        &auth_fn,
                    )?;
                }
                _ => {}
            }
        // Assume we're resolving a field to be returned by a query or
        // mutation when required_op is None (objects are agnostic to
        // operations) and auth is set.
        } else if let Some(auth) = info.auth {
            let field_ops = auth_fn(Some(auth), Operations::empty());
            tracing::trace!("Field level auth. field_ops:{field_ops}");
            if !field_ops.intersects(Operations::READ) {
                // FIXME: Field rule should not have operations configurable.
                let msg = format!(
                    "Unauthorized to access {type_name}.{field_name}",
                    type_name = info.parent_type,
                    field_name = info.name,
                );
                tracing::warn!("{msg} field_ops={field_ops:?}");
                return Err(ServerError::new(msg, None));
            }
        }

        next.run(ctx, info).await
    }
}

struct CheckInputOptions<'a, F: Fn(Option<&AuthConfig>, Operations) -> Operations> {
    input: &'a ConstValue,
    type_name: NamedType<'a>,
    mutation_name: &'a str,
    registry: &'a registry_v2::Registry,
    required_op: Operations,
    model_allowed_ops: Operations,
    auth_fn: &'a F,
}

impl AuthExtension {
    pub fn new(trace_id: String) -> Self {
        Self { trace_id }
    }

    // Only allow create/update when the user is authorized to access ALL fields passed as input
    fn check_input<F: Fn(Option<&AuthConfig>, Operations) -> Operations>(
        &self,
        opts: CheckInputOptions<'_, F>,
    ) -> Result<(), ServerError> {
        let ConstValue::Object(input_fields) = opts.input else {
            return Ok(());
        };

        tracing::info!("{:?}", opts.mutation_name);
        tracing::info!("{:?}", opts.type_name);
        tracing::info!("{:?}", input_fields);
        let type_fields = opts
            .registry
            .lookup_type(opts.type_name.as_str())
            .expect("type must exist")
            .fields()
            .expect("type must have fields")
            .collect::<Vec<_>>();

        for field_name in input_fields.keys() {
            let field = type_fields
                .iter()
                .find(|field| field.name() == field_name.as_str())
                .expect("field must exist");

            let field_ops = (opts.auth_fn)(field.auth(), opts.model_allowed_ops);

            tracing::trace!("check_input.{field_name} ${field_ops}");

            if !field_ops.contains(opts.required_op) {
                let msg = format!(
                    "Unauthorized to access {MUTATION_TYPE}.{mutation_name} (missing {required_op} operation on {type_name}.{field_name})",
                    mutation_name = opts.mutation_name,
                    required_op = opts.required_op,
                    type_name = opts.type_name,
                );

                tracing::warn!("{msg} auth={auth:?}", auth = field.auth());
                return Err(ServerError::new(msg, None));
            }
        }

        Ok(())
    }

    // Only allow delete when the user is authorized to delete ALL fields of the type
    // TODO: Check fields of nested types once we support cascading deletes
    fn check_delete<F: Fn(Option<&AuthConfig>, Operations) -> Operations>(
        &self,
        type_name: NamedType<'_>,
        mutation_name: &str,
        registry: &registry_v2::Registry,
        model_ops: Operations,
        auth_fn: &F,
    ) -> Result<(), ServerError> {
        let type_fields = registry
            .lookup_type(type_name.as_str())
            .expect("type must exist")
            .fields()
            .expect("type must have fields");

        for field in type_fields {
            let field_ops = auth_fn(field.auth(), model_ops);

            if !field_ops.contains(Operations::DELETE) {
                let msg = format!(
                    "Unauthorized to access {MUTATION_TYPE}.{mutation_name} (missing delete operation on {type_name}.{})", field.name()
                );
                tracing::warn!("{msg} auth={auth:?}", auth = field.auth());
                return Err(ServerError::new(msg, None));
            }
        }

        Ok(())
    }
}

// HACK to get underlying type, which is not available in ResolveInfo
#[allow(clippy::panic)]
fn guess_type_name(info: &ResolveInfo<'_>, required_op: Operations) -> NamedType<'static> {
    let suffix = match required_op {
        Operations::CREATE => "CreatePayload",
        Operations::UPDATE => "UpdatePayload",
        Operations::DELETE => "DeletePayload",
        _ => panic!("unexpected operation"),
    };

    named_type_from_type_str(info.return_type)
        .strip_suffix(suffix)
        .expect("must be the expected Payload type")
        .to_owned()
        .into()
}

// HACK: we're deprecating the database, so continuing the previous hack...
#[allow(clippy::panic)]
fn guess_batch_operation_type_name(info: &ResolveInfo<'_>, required_op: Operations) -> Option<NamedType<'static>> {
    let suffix = match required_op {
        Operations::CREATE => "CreateManyPayload",
        Operations::UPDATE => "UpdateManyPayload",
        Operations::DELETE => "DeleteManyPayload",
        _ => panic!("unexpected operation"),
    };

    named_type_from_type_str(info.return_type)
        .strip_suffix(suffix)
        .map(|name| name.to_owned().into())
}

fn named_type_from_type_str(meta: &str) -> &str {
    let mut nested = Some(meta);

    if meta.starts_with('[') && meta.ends_with(']') {
        nested = nested.and_then(|x| x.strip_prefix('['));
        nested = nested.and_then(|x| x.strip_suffix(']'));
        return named_type_from_type_str(nested.expect("Can't fail"));
    }

    if meta.ends_with('!') {
        nested = nested.and_then(|x| x.strip_suffix('!'));
        return named_type_from_type_str(nested.expect("Can't fail"));
    }

    nested.expect("Can't fail")
}
