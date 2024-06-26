use common_types::auth::Operations;
use engine::registry::{
    resolvers::{custom::CustomResolver, Resolver},
    MetaField,
};
use engine_parser::types::{ObjectType, TypeKind};
use registry_v2::FederationProperties;

use super::{
    deprecated_directive::DeprecatedDirective,
    federation::{InaccessibleDirective, TagDirective},
    join_directive::JoinDirective,
    requires_directive::RequiresDirective,
    visitor::{Visitor, VisitorContext, MUTATION_TYPE, QUERY_TYPE},
};
use crate::{
    parser_extensions::FieldExtension,
    rules::{cache_directive::CacheDirective, resolver_directive::ResolverDirective},
};

pub struct ExtendQueryAndMutationTypes;

enum EntryPoint {
    Query,
    Mutation,
}

fn find_entry_point(
    type_definition: &engine::Positioned<engine_parser::types::TypeDefinition>,
) -> Option<(EntryPoint, &ObjectType)> {
    match &type_definition.node.kind {
        TypeKind::Object(object) if type_definition.node.name.node == QUERY_TYPE => Some((EntryPoint::Query, object)),
        TypeKind::Object(object) if type_definition.node.name.node == MUTATION_TYPE => {
            Some((EntryPoint::Mutation, object))
        }
        _ => None,
    }
}

impl<'a> Visitor<'a> for ExtendQueryAndMutationTypes {
    fn enter_type_definition(
        &mut self,
        ctx: &mut VisitorContext<'a>,
        type_definition: &'a engine::Positioned<engine_parser::types::TypeDefinition>,
    ) {
        if let Some((entry_point, object)) = find_entry_point(type_definition) {
            let type_name = type_definition.node.name.node.to_string();
            let required_operation = match entry_point {
                EntryPoint::Query => Some(Operations::READ),
                EntryPoint::Mutation => Some(Operations::WRITE),
            };
            for field in &object.fields {
                let name = field.node.name.node.to_string();
                let deprecation = DeprecatedDirective::from_directives(&field.directives, ctx);
                let inaccessible = InaccessibleDirective::from_directives(&field.directives, ctx);
                let tags = TagDirective::from_directives(&field.directives, ctx);
                let mut requires =
                    RequiresDirective::from_directives(&field.directives, ctx).map(|requires| requires.into_fields());

                let resolver = match (
                    ResolverDirective::resolver_name(&field.node),
                    JoinDirective::from_directives(&field.directives, ctx),
                ) {
                    (Some(name), None) => Resolver::CustomResolver(CustomResolver {
                        resolver_name: name.to_owned(),
                    }),
                    (None, Some(_)) if requires.is_some() => {
                        ctx.report_error(
                            vec![field.pos],
                            format!("{type_name}.{name} field can't have a join and a requires on it"),
                        );
                        return;
                    }
                    (None, Some(join_directive)) => {
                        requires = join_directive.select.required_fieldset(&field.arguments);
                        Resolver::Join(join_directive.select.to_join_resolver())
                    }
                    (None, None) => {
                        ctx.report_error(
                            vec![field.pos],
                            format!("{type_name}.{name} must have a join or a resolver defined"),
                        );
                        return;
                    }
                    (Some(_), Some(_)) => {
                        ctx.report_error(
                            vec![field.pos],
                            format!("{type_name}.{name} can't have both a join and a resolver defined"),
                        );
                        return;
                    }
                };

                let (field_collection, cache_control) = match entry_point {
                    EntryPoint::Query => (&mut ctx.queries, CacheDirective::parse(&field.node.directives)),
                    EntryPoint::Mutation => (&mut ctx.mutations, Default::default()),
                };

                let mut federation = None;
                if inaccessible || !tags.is_empty() {
                    federation = Some(Box::new(FederationProperties {
                        inaccessible,
                        tags,
                        ..Default::default()
                    }));
                }

                field_collection.push(MetaField {
                    name: name.clone(),
                    mapped_name: None,
                    description: field.node.description.clone().map(|x| x.node),
                    args: field.converted_arguments(),
                    ty: field.node.ty.clone().node.to_string().into(),
                    deprecation,
                    cache_control,
                    requires,
                    resolver,
                    required_operation,
                    auth: None,
                    federation,
                });
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use engine::CacheControl;
    use engine_parser::parse_schema;
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::rules::visitor::visit;

    #[rstest::rstest]
    #[case(r#"
        type Query {
            foo: String! @resolver(name: "blah")
        }
    "#, &[])]
    #[case(r#"
        type Query {
            foo: String! @resolver(name: "blah")
            bar: String! @join(select: "foo")
        }
    "#, &[])]
    #[case(r"
        extend type Query {
            foo: String!
        }
    ", &[
        "Query.foo must have a join or a resolver defined"
    ])]
    #[case(r#"
        extend type Query {
            foo: String! @resolver(name: "return-foo")
        }
    "#, &[])]
    #[case(r#"
        type Mutation {
            foo: String! @resolver(name: "blah")
        }
    "#, &[])]
    #[case(r"
        extend type Mutation {
            foo: String!
        }
    ", &[
        "Mutation.foo must have a join or a resolver defined"
    ])]
    #[case(r#"
        extend type Mutation {
            foo: String! @resolver(name: "return-foo")
        }
    "#, &[])]
    #[case(r#"
        extend type Query {
            foo: String! @resolver(name: "return-foo") @join(select: "whatever")
        }
    "#, &["Query.foo can't have both a join and a resolver defined"])]
    fn test_parse_result(#[case] schema: &str, #[case] expected_messages: &[&str]) {
        let schema = parse_schema(schema).unwrap();
        let mut ctx = VisitorContext::new_for_tests(&schema);
        visit(&mut ExtendQueryAndMutationTypes, &mut ctx, &schema);

        let actual_messages: Vec<_> = ctx.errors.iter().map(|error| error.message.as_str()).collect();
        assert_eq!(actual_messages.as_slice(), expected_messages);
    }

    #[test]
    fn test_parse_result_with_cache() {
        // prepare
        let schema = r#"
            extend type Query {
                foo: String! @resolver(name: "foo") @cache(maxAge: 60)
            }

            extend type Mutation {
                foo: String! @resolver(name: "foo") @cache(maxAge: 60)
            }
        "#;

        let schema = parse_schema(schema).unwrap();
        let mut ctx = VisitorContext::new_for_tests(&schema);

        // act
        visit(&mut ExtendQueryAndMutationTypes, &mut ctx, &schema);

        // assert
        assert!(ctx.errors.is_empty());

        let foo_query = ctx
            .queries
            .iter()
            .find(|query| query.name == "foo")
            .expect("Should find foo query");
        let foo_mutation = ctx
            .mutations
            .iter()
            .find(|mutation| mutation.name == "foo")
            .expect("Should find foo mutation");

        assert_eq!(
            foo_query.cache_control,
            Some(Box::new(CacheControl {
                max_age: 60,
                ..Default::default()
            }))
        );

        assert_eq!(foo_mutation.cache_control, Default::default());
    }
}
