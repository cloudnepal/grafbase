use {
    crate::visitor::{Visitor, VisitorContext},
    engine_parser::{
        types::{FragmentDefinition, InlineFragment, TypeCondition, VariableDefinition},
        Pos, Positioned,
    },
    engine_value::Name,
    meta_type_name::MetaTypeName,
};

#[derive(Default)]
pub struct KnownTypeNames;

impl<'a> Visitor<'a, registry_v2::Registry> for KnownTypeNames {
    fn enter_fragment_definition(
        &mut self,
        ctx: &mut VisitorContext<'a, registry_v2::Registry>,
        _name: &'a Name,
        fragment_definition: &'a Positioned<FragmentDefinition>,
    ) {
        let TypeCondition { on: name } = &fragment_definition.node.type_condition.node;
        validate_type(ctx, &name.node, fragment_definition.pos);
    }

    fn enter_variable_definition(
        &mut self,
        ctx: &mut VisitorContext<'a, registry_v2::Registry>,
        variable_definition: &'a Positioned<VariableDefinition>,
    ) {
        validate_type(
            ctx,
            MetaTypeName::concrete_typename(&variable_definition.node.var_type.to_string()),
            variable_definition.pos,
        );
    }

    fn enter_inline_fragment(
        &mut self,
        ctx: &mut VisitorContext<'a, registry_v2::Registry>,
        inline_fragment: &'a Positioned<InlineFragment>,
    ) {
        if let Some(TypeCondition { on: name }) = inline_fragment.node.type_condition.as_ref().map(|c| &c.node) {
            validate_type(ctx, &name.node, inline_fragment.pos);
        }
    }
}

fn validate_type(ctx: &mut VisitorContext<'_, registry_v2::Registry>, type_name: &str, pos: Pos) {
    if ctx.registry.lookup_type(type_name).is_none() {
        ctx.report_error(vec![pos], format!(r#"Unknown type "{type_name}""#));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    pub fn factory() -> KnownTypeNames {
        KnownTypeNames
    }

    #[test]
    fn known_type_names_are_valid() {
        expect_passes_rule!(
            factory,
            r"
          query Foo($var: String, $required: [String!]!) {
            user(id: 4) {
              pets { ... on Pet { name }, ...PetFields, ... { name } }
            }
          }
          fragment PetFields on Pet {
            name
          }
        ",
        );
    }

    #[test]
    fn unknown_type_names_are_invalid() {
        expect_fails_rule!(
            factory,
            r"
          query Foo($var: JumbledUpLetters) {
            user(id: 4) {
              name
              pets { ... on Badger { name }, ...PetFields }
            }
          }
          fragment PetFields on Peettt {
            name
          }
        ",
        );
    }
}
