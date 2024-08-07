use std::sync::Arc;

use runtime::{
    error::{PartialErrorCode, PartialGraphqlError},
    hooks::{Anything, AuthorizationVerdict, AuthorizationVerdicts, AuthorizedHooks, EdgeDefinition, NodeDefinition},
};
use tracing::instrument;

use super::{guest_error_as_gql, Context, HooksWasi};

macro_rules! prepare_authorized {
    ($self:ident named $func_name:literal at $definition:expr; [$(($name:literal, $input:expr),)+]) => {{
        let Some(ref inner) = $self.0 else {
            return Err(PartialGraphqlError::new(
                "@authorized directive cannot be used, so access was denied",
                PartialErrorCode::Unauthorized,
            ));
        };
        let instance = inner.authorization.get().await;
        let inputs = [$(
            encode($func_name, $definition, $name, $input)?,
        )+];
        (instance, inputs)
    }};
}

fn encode<'a>(
    func_name: &str,
    definition: impl std::fmt::Display,
    name: &str,
    values: impl IntoIterator<Item: Anything<'a>>,
) -> Result<Vec<String>, PartialGraphqlError> {
    values
        .into_iter()
        .map(|value| {
            serde_json::to_string(&value).map_err(|_| {
                tracing::error!("{func_name} error at {definition}: failed to serialize {name}");
                PartialGraphqlError::internal_server_error()
            })
        })
        .collect()
}

impl AuthorizedHooks<Context> for HooksWasi {
    #[instrument(skip_all)]
    async fn authorize_edge_pre_execution<'a>(
        &self,
        context: &Context,
        definition: EdgeDefinition<'a>,
        arguments: impl Anything<'a>,
        metadata: Option<impl Anything<'a>>,
    ) -> AuthorizationVerdict {
        let (mut instance, [arguments, metadata]) = prepare_authorized!(
            self named "authorize_edge_pre_execution" at &definition;
            [("arguments", [arguments]), ("metadata", metadata),]
        );
        let arguments = arguments.into_iter().next().unwrap();
        let metadata = metadata.into_iter().next().unwrap_or_default();
        let definition = wasi_component_loader::EdgeDefinition {
            parent_type_name: definition.parent_type_name.to_string(),
            field_name: definition.field_name.to_string(),
        };

        instance
            .authorize_edge_pre_execution(Arc::clone(context), definition, arguments, metadata)
            .await
            .map_err(|err| match err {
                wasi_component_loader::Error::Internal(error) => {
                    tracing::error!("authorize_edge_pre_execution error at: {error}");
                    PartialGraphqlError::internal_hook_error()
                }
                wasi_component_loader::Error::Guest(error) => guest_error_as_gql(error, PartialErrorCode::Unauthorized),
            })?;

        Ok(())
    }

    #[instrument(skip_all)]
    async fn authorize_node_pre_execution<'a>(
        &self,
        context: &Context,
        definition: NodeDefinition<'a>,
        metadata: Option<impl Anything<'a>>,
    ) -> AuthorizationVerdict {
        let (mut instance, [metadata]) = prepare_authorized!(
            self named "authorize_node_pre_execution" at &definition;
            [ ("metadata", metadata),]
        );
        let metadata = metadata.into_iter().next().unwrap_or_default();
        let definition = wasi_component_loader::NodeDefinition {
            type_name: definition.type_name.to_string(),
        };

        instance
            .authorize_node_pre_execution(Arc::clone(context), definition, metadata)
            .await
            .map_err(|err| match err {
                wasi_component_loader::Error::Internal(error) => {
                    tracing::error!("authorize_node_pre_execution error at: {error}");
                    PartialGraphqlError::internal_hook_error()
                }
                wasi_component_loader::Error::Guest(error) => guest_error_as_gql(error, PartialErrorCode::Unauthorized),
            })?;

        Ok(())
    }

    #[instrument(skip_all)]
    async fn authorize_node_post_execution<'a>(
        &self,
        _context: &Context,
        definition: NodeDefinition<'a>,
        nodes: impl IntoIterator<Item: Anything<'a>> + Send,
        metadata: Option<impl Anything<'a>>,
    ) -> AuthorizationVerdicts {
        let (mut _instance, [_nodes, metadata]) = prepare_authorized!(
            self named "authorize_node_post_execution" at &definition;
            [("nodes", nodes), ("metadata", metadata),]
        );
        let _metadata = metadata.into_iter().next().unwrap_or_default();
        let _definition = wasi_component_loader::NodeDefinition {
            type_name: definition.type_name.to_string(),
        };

        todo!()
    }

    #[instrument(skip_all)]
    async fn authorize_parent_edge_post_execution<'a>(
        &self,
        context: &Context,
        definition: EdgeDefinition<'a>,
        parents: impl IntoIterator<Item: Anything<'a>> + Send,
        metadata: Option<impl Anything<'a>>,
    ) -> AuthorizationVerdicts {
        let (mut instance, [parents, metadata]) = prepare_authorized!(
            self named "authorize_parent_edge_post_execution" at &definition;
            [("parents", parents), ("metadata", metadata),]
        );
        let metadata = metadata.into_iter().next().unwrap_or_default();
        let definition = wasi_component_loader::EdgeDefinition {
            parent_type_name: definition.parent_type_name.to_string(),
            field_name: definition.field_name.to_string(),
        };

        let results = instance
            .authorize_parent_edge_post_execution(Arc::clone(context), definition, parents, metadata)
            .await
            .map_err(|err| match err {
                wasi_component_loader::Error::Internal(error) => {
                    tracing::error!("authorize_parent_edge_post_execution error at: {error}");
                    PartialGraphqlError::internal_server_error()
                }
                wasi_component_loader::Error::Guest(error) => guest_error_as_gql(error, PartialErrorCode::Unauthorized),
            })?
            .into_iter()
            .map(|result| match result {
                Ok(()) => Ok(()),
                Err(error) => Err(guest_error_as_gql(error, PartialErrorCode::Unauthorized)),
            })
            .collect();

        Ok(results)
    }

    #[instrument(skip_all)]
    async fn authorize_edge_node_post_execution<'a>(
        &self,
        context: &Context,
        definition: EdgeDefinition<'a>,
        nodes: impl IntoIterator<Item: Anything<'a>> + Send,
        metadata: Option<impl Anything<'a>>,
    ) -> AuthorizationVerdicts {
        let (mut instance, [nodes, metadata]) = prepare_authorized!(
            self named "authorize_edge_node_post_execution" at &definition;
            [("nodes", nodes), ("metadata", metadata),]
        );
        let metadata = metadata.into_iter().next().unwrap_or_default();
        let definition = wasi_component_loader::EdgeDefinition {
            parent_type_name: definition.parent_type_name.to_string(),
            field_name: definition.field_name.to_string(),
        };

        let result = instance
            .authorize_edge_node_post_execution(Arc::clone(context), definition, nodes, metadata)
            .await
            .map_err(|err| match err {
                wasi_component_loader::Error::Internal(error) => {
                    tracing::error!("authorize_edge_node_post_execution error at: {error}");
                    PartialGraphqlError::internal_server_error()
                }
                wasi_component_loader::Error::Guest(error) => guest_error_as_gql(error, PartialErrorCode::Unauthorized),
            })?
            .into_iter()
            .map(|result| match result {
                Ok(()) => Ok(()),
                Err(error) => Err(guest_error_as_gql(error, PartialErrorCode::Unauthorized)),
            })
            .collect();

        Ok(result)
    }

    #[instrument(skip_all)]
    async fn authorize_edge_post_execution<'a, Parent, Nodes>(
        &self,
        context: &Context,
        definition: EdgeDefinition<'a>,
        edges: impl IntoIterator<Item = (Parent, Nodes)> + Send,
        metadata: Option<impl Anything<'a>>,
    ) -> AuthorizationVerdicts
    where
        Parent: Anything<'a>,
        Nodes: IntoIterator<Item: Anything<'a>> + Send,
    {
        let (mut instance, [metadata]) = prepare_authorized!(
            self named "authorize_edge_post_execution" at &definition;
            [("metadata", metadata),]
        );
        let metadata: String = metadata.into_iter().next().unwrap_or_default();
        let edges: Vec<(String, Vec<String>)> = edges
            .into_iter()
            .map(|(parent, nodes): (Parent, Nodes)| {
                let parent = serde_json::to_string(&parent).map_err(|_| {
                    tracing::error!("authorize_edge_post_execution error at {definition}: failed to serialize edge");
                    PartialGraphqlError::internal_server_error()
                })?;
                let nodes = nodes
                    .into_iter()
                    .map(|node| {
                        serde_json::to_string(&node).map_err(|_| {
                            tracing::error!(
                                "authorize_edge_post_execution error at {definition}: failed to serialize edge"
                            );
                            PartialGraphqlError::internal_server_error()
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()?;

                Ok((parent, nodes))
            })
            .collect::<Result<Vec<_>, _>>()?;
        let definition = wasi_component_loader::EdgeDefinition {
            parent_type_name: definition.parent_type_name.to_string(),
            field_name: definition.field_name.to_string(),
        };

        let result = instance
            .authorize_edge_post_execution(Arc::clone(context), definition, edges, metadata)
            .await
            .map_err(|err| match err {
                wasi_component_loader::Error::Internal(error) => {
                    tracing::error!("authorize_edge_post_execution error at: {error}");
                    PartialGraphqlError::internal_server_error()
                }
                wasi_component_loader::Error::Guest(error) => guest_error_as_gql(error, PartialErrorCode::Unauthorized),
            })?
            .into_iter()
            .map(|result| match result {
                Ok(()) => Ok(()),
                Err(error) => Err(guest_error_as_gql(error, PartialErrorCode::Unauthorized)),
            })
            .collect();

        Ok(result)
    }
}
