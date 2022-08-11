//! ### What it does
//!
//! The user defined scalars can be hydrated to the generated API only if those scalars belongs to
//! the list of PossibleScalar from dynaql for now.
//!
use dynaql::registry::scalars::PossibleScalar;
use dynaql::{Positioned, Value};
use dynaql_parser::types::TypeDefinition;

use super::visitor::{Visitor, VisitorContext};

pub struct ScalarHydratation;

const SPECIFIED_BY_DIRECTIVE: &str = "specifiedBy";
const SPECIFIED_BY_ARGUMENT_URL: &str = "url";

impl<'a> Visitor<'a> for ScalarHydratation {
    fn enter_scalar_definition(
        &mut self,
        ctx: &mut VisitorContext<'a>,
        type_definition: &'a Positioned<TypeDefinition>,
    ) {
        let name = type_definition.node.name.node.as_str().to_string();

        match PossibleScalar::try_from(name.as_str()) {
            Ok(_) => {
                ctx.registry.get_mut().create_type(
                    &mut |_| {
                        let specified_by_url = type_definition
                            .node
                            .directives
                            .iter()
                            .find(|directive| directive.node.name.node.as_str() == SPECIFIED_BY_DIRECTIVE)
                            .and_then(|directive| directive.node.get_argument(SPECIFIED_BY_ARGUMENT_URL))
                            .and_then(|x| match &x.node {
                                Value::String(s) => Some(s.clone()),
                                _ => None,
                            });

                        dynaql::registry::MetaType::Scalar {
                            name: name.clone(),
                            description: type_definition
                                .node
                                .description
                                .clone()
                                .map(|x| x.node.as_str().to_string()),
                            is_valid: None,
                            visible: None,
                            specified_by_url,
                        }
                    },
                    name.as_str(),
                    name.as_str(),
                );
            }
            Err(err) => {
                ctx.report_error(vec![type_definition.pos], err.to_string());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ScalarHydratation;
    use crate::rules::visitor::{visit, VisitorContext};
    use dynaql_parser::parse_schema;
    use serde_json as _;

    #[test]
    fn should_error_when_defining_a_invalid_scalar() {
        let schema = r#"
            scalar Date

            type Product @model {
                id: ID!
                test: Date!
            }
            "#;

        let schema = parse_schema(schema).expect("");

        let mut ctx = VisitorContext::new(&schema);
        visit(&mut ScalarHydratation, &mut ctx, &schema);

        assert!(!ctx.errors.is_empty(), "shouldn't be empty");
        assert_eq!(ctx.errors.len(), 1, "should have one error");
        assert_eq!(
            ctx.errors.get(0).unwrap().message,
            "\"Date\" is not a proper scalar",
            "should match"
        );
    }

    #[test]
    fn should_work_with_a_valid_scalar() {
        let schema = r#"
            scalar DateTime

            type Product @model {
                id: ID!
                test: DateTime!
            }
            "#;

        let schema = parse_schema(schema).expect("");

        let mut ctx = VisitorContext::new(&schema);
        visit(&mut ScalarHydratation, &mut ctx, &schema);

        let scalar_ty = ctx.registry.get_mut().types.get("DateTime");

        assert!(ctx.errors.is_empty(), "should be empty");
        assert!(scalar_ty.is_some(), "should have the scalar definition");
        insta::assert_debug_snapshot!(scalar_ty.unwrap());
    }
}
