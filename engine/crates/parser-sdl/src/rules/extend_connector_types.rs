use std::collections::btree_map::Entry;

use engine::registry::{
    self,
    resolvers::{custom::CustomResolver, Resolver},
    MetaField, MetaType,
};
use engine_parser::types::TypeKind;
use registry_v2::{resolvers::transformer::Transformer, FederationProperties};

use super::{
    deprecated_directive::DeprecatedDirective,
    federation::{
        ExternalDirective, InaccessibleDirective, OverrideDirective, ProvidesDirective, ShareableDirective,
        TagDirective,
    },
    join_directive::JoinDirective,
    requires_directive::RequiresDirective,
    visitor::{Visitor, VisitorContext},
};
use crate::{
    parser_extensions::FieldExtension, rules::resolver_directive::ResolverDirective, schema_coord::SchemaCoord,
};

pub struct ExtendConnectorTypes;

impl<'a> Visitor<'a> for ExtendConnectorTypes {
    fn enter_type_definition(
        &mut self,
        ctx: &mut VisitorContext<'a>,
        type_definition: &'a engine::Positioned<engine_parser::types::TypeDefinition>,
    ) {
        let type_name = type_definition.node.name.as_str();
        let TypeKind::Object(object) = &type_definition.node.kind else {
            return;
        };

        if !type_definition.node.extend || matches!(type_name, "Query" | "Mutation") {
            // Query & Mutation extensions are handled in ExtendQueryAndMutationTypes
            return;
        }

        super::basic_type::handle_key_directives(&type_definition.directives, type_name, ctx);

        let extended_fields = object
            .fields
            .iter()
            .filter_map(|field| {
                let name = field.node.name.node.to_string();

                let join_directive = JoinDirective::from_directives(&field.node.directives, ctx);
                let resolver_name = ResolverDirective::resolver_name(&field.node);

                let mut requires =
                    RequiresDirective::from_directives(&field.directives, ctx).map(RequiresDirective::into_fields);

                let external = ExternalDirective::from_directives(&field.directives, ctx).is_some();
                let shareable = ShareableDirective::from_directives(&field.directives, ctx).is_some();
                let r#override =
                    OverrideDirective::from_directives(&field.directives, ctx).map(|directive| directive.from);
                let provides =
                    ProvidesDirective::from_directives(&field.directives, ctx).map(|directive| directive.fields);
                let deprecation = DeprecatedDirective::from_directives(&field.directives, ctx);
                let inaccessible = InaccessibleDirective::from_directives(&field.directives, ctx);
                let tags = TagDirective::from_directives(&field.directives, ctx);

                let resolver = match (join_directive, resolver_name) {
                    (None, None) if is_key_field(ctx, type_name, &name) => {
                        Resolver::Transformer(Transformer::Select { key: name.to_string() })
                    }
                    (None, None) => {
                        ctx.report_error(
                            vec![field.pos],
                            format!("Field '{name}' of extended '{type_name}' must have a custom resolver or a join"),
                        );
                        return None;
                    }
                    (None, Some(resolver_name)) => Resolver::CustomResolver(CustomResolver {
                        resolver_name: resolver_name.to_owned(),
                    }),
                    (Some(join_directive), None) => {
                        if requires.is_some() {
                            // We could support this by merging the requires, but I don't want to implement it right now.
                            // If someone asks we could do it
                            ctx.report_error(vec![field.pos], "A field can't have a join and a requires on it");
                        }
                        requires = join_directive.select.required_fieldset(&field.arguments);

                        ctx.warnings.extend(
                            join_directive
                                .validate_arguments(&field.arguments, SchemaCoord::Field(type_name, field.name())),
                        );

                        Resolver::Join(join_directive.select.to_join_resolver())
                    }
                    (Some(_), Some(_)) => {
                        ctx.report_error(vec![field.pos], "A field can't have a join and a custom resolver on it");
                        return None;
                    }
                };

                let field = &field.node;

                let mut federation = None;
                if external
                    || shareable
                    || r#override.is_some()
                    || provides.is_some()
                    || inaccessible
                    || !tags.is_empty()
                {
                    federation = Some(Box::new(FederationProperties {
                        provides,
                        tags,
                        r#override,
                        external,
                        shareable,
                        inaccessible,
                    }));
                }

                Some(MetaField {
                    name,
                    description: field.description.clone().map(|x| x.node),
                    args: field.converted_arguments(),
                    ty: field.ty.clone().node.to_string().into(),
                    requires,
                    resolver,
                    federation,
                    deprecation,
                    ..MetaField::default()
                })
            })
            .collect::<Vec<_>>();

        let is_external = ExternalDirective::from_directives(&type_definition.directives, ctx).is_some();
        let is_shareable = ShareableDirective::from_directives(&type_definition.directives, ctx).is_some();

        let mut registry = ctx.registry.borrow_mut();

        match registry.types.entry(type_name.to_string()) {
            Entry::Vacant(entry) => {
                entry.insert(
                    registry::ObjectType::new(type_name.to_string(), extended_fields)
                        .with_external(is_external)
                        .with_shareable(is_shareable)
                        .into(),
                );
            }
            Entry::Occupied(mut entry) => {
                let MetaType::Object(registry::ObjectType {
                    fields,
                    shareable,
                    external,
                    ..
                }) = entry.get_mut()
                else {
                    drop(registry);
                    ctx.report_error(
                        vec![type_definition.pos],
                        format!("Tried to extend '{type_name}' as an object but this type is not an object"),
                    );
                    return;
                };

                *shareable |= is_shareable;
                *external |= is_external;

                fields.extend(extended_fields.into_iter().map(|field| (field.name.to_owned(), field)));
            }
        }
    }
}

fn is_key_field(ctx: &VisitorContext<'_>, type_name: &str, field_name: &str) -> bool {
    ctx.registry
        .borrow()
        .federation_entities
        .get(type_name)
        .map(|entity| entity.keys.iter().any(|key| key.includes_field(field_name)))
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use engine::registry::{self, MetaField, Registry};
    use serde_json as _;

    use crate::{rules::postgres_directive::PostgresDirective, ConnectorParsers, GraphqlDirective, OpenApiDirective};

    #[test]
    fn test_connector_models_can_be_extended() {
        let output = futures::executor::block_on(crate::parse(
            r#"
        extend schema @openapi(name: "Stripe", namespace: true, schema: "http://example.com")

        extend type StripeCustomer {
            email: String @resolver(name: "email")
        }
        "#,
            &HashMap::new(),
            &FakeConnectorParser,
        ));

        output
            .unwrap()
            .registry
            .types
            .get("StripeCustomer")
            .unwrap()
            .field_by_name("email")
            .expect("StripeCustomer to have an email field after parsing");
    }

    #[test]
    fn test_federation_key_fields_dont_need_join_or_resolver() {
        let output = futures::executor::block_on(crate::parse(
            r#"
        extend schema @federation(version: "2.3")

        extend type Whatever @key(fields: "id") {
            id: String!
            someNewField: String! @resolver(name: "whatever")
        }
        "#,
            &HashMap::new(),
            &FakeConnectorParser,
        ))
        .unwrap();

        output
            .registry
            .types
            .get("Whatever")
            .unwrap()
            .field_by_name("id")
            .expect("StripeCustomer to have an id field after parsing");

        let number_keys = output.registry.federation_entities.get("Whatever").unwrap().keys.len();

        assert_eq!(number_keys, 1);
    }

    #[test]
    fn types_from_resolvers_of_extended_connector_types_can_be_extended() {
        let output = futures::executor::block_on(crate::parse(
            r#"
        extend schema @openapi(name: "Stripe", namespace: true, schema: "http://example.com")

        extend type StripeCustomer {
            email: String @resolver(name: "email")
            location: Place @resolver(name: "customer/location")
        }

        extend type Place {
            annualPrecipitations: Int @resolver(name: "place/annualPrecipitations")
        }

        type Place {
            id: ID!
            name: String
        }

        extend type Place {
            squareMeterPrice: Int @resolver(name: "place/squareMeterPrice")
        }
        "#,
            &HashMap::new(),
            &FakeConnectorParser,
        ));

        let registry = output.unwrap().registry;

        registry
            .types
            .get("StripeCustomer")
            .unwrap()
            .field_by_name("location")
            .expect("StripeCustomer to have a location field after parsing");

        let place = registry.types.get("Place").unwrap();

        place.field_by_name("annualPrecipitations").unwrap();
        place.field_by_name("squareMeterPrice").unwrap();
        place.field_by_name("name").unwrap();
    }

    #[test]
    fn types_from_resolvers_of_extended_connector_types_can_be_extended_with_directives() {
        let output = futures::executor::block_on(crate::parse(
            r#"
        extend schema @openapi(name: "Stripe", namespace: true, schema: "http://example.com")

        extend type StripeCustomer {
            email: String @resolver(name: "email")
            location: Place @resolver(name: "customer/location") @shareable
        }

        extend type Place @shareable {
            annualPrecipitations: Int @resolver(name: "place/annualPrecipitations") @external
        }

        type Place @external {
            id: ID!
            name: String @shareable
        }

        extend type Place {
            squareMeterPrice: Int @resolver(name: "place/squareMeterPrice") @shareable
        }
        "#,
            &HashMap::new(),
            &FakeConnectorParser,
        ));

        let registry = output.unwrap().registry;

        let location = registry
            .types
            .get("StripeCustomer")
            .unwrap()
            .field_by_name("location")
            .expect("StripeCustomer to have a location field after parsing");
        let federation = location.federation.as_ref().unwrap();
        assert!(federation.shareable);

        let place = registry.types.get("Place").unwrap();
        let place_object = place.object().unwrap();
        assert!(place_object.shareable);
        assert!(place_object.external);

        let annual_precipitations = place.field_by_name("annualPrecipitations").unwrap();
        let federation = annual_precipitations.federation.as_ref().unwrap();
        assert!(federation.external);

        let square_meter_price = place.field_by_name("squareMeterPrice").unwrap();
        let federation = square_meter_price.federation.as_ref().unwrap();
        assert!(federation.shareable);

        let name = place.field_by_name("name").unwrap();
        let federation = name.federation.as_ref().unwrap();
        assert!(federation.shareable);
    }

    #[rstest::rstest]
    #[case::extend_without_resolver(r#"
        extend schema @openapi(name: "Stripe", namespace: true, schema: "http://example.com")

        extend type StripeCustomer {
            foo: String!
        }
    "#, &["Field 'foo' of extended 'StripeCustomer' must have a custom resolver or a join"])]
    fn test_parse_result(#[case] schema: &str, #[case] expected_messages: &[&str]) {
        let output = futures::executor::block_on(crate::parse(schema, &HashMap::new(), &FakeConnectorParser));

        let validation_errors = output.unwrap_err().validation_errors().unwrap_or_default();

        let actual_messages = validation_errors
            .iter()
            .map(|error| error.message.as_str())
            .collect::<Vec<_>>();

        assert_eq!(actual_messages.as_slice(), expected_messages);
    }

    struct FakeConnectorParser;

    #[async_trait::async_trait]
    impl ConnectorParsers for FakeConnectorParser {
        async fn fetch_and_parse_openapi(&self, _directive: OpenApiDirective) -> Result<Registry, Vec<String>> {
            let mut registry = Registry::new();
            registry.types.insert(
                "StripeCustomer".into(),
                registry::ObjectType::new(
                    "StripeCustomer",
                    [MetaField {
                        name: "id".into(),
                        ty: "String".into(),
                        ..MetaField::default()
                    }],
                )
                .into(),
            );
            registry.query_root_mut().fields_mut().unwrap().insert(
                "customer".into(),
                MetaField {
                    name: "customer".into(),
                    ty: "StripeCustomer".into(),
                    ..MetaField::default()
                },
            );
            Ok(registry)
        }

        async fn fetch_and_parse_graphql(&self, _directive: GraphqlDirective) -> Result<Registry, Vec<String>> {
            Err(Vec::new())
        }

        async fn fetch_and_parse_postgres(&self, _: &PostgresDirective) -> Result<Registry, Vec<String>> {
            Err(Vec::new())
        }
    }
}
