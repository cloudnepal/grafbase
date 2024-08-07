use std::{
    collections::HashMap,
    fmt::{self, Display, Formatter},
};

use engine_value::{Name, Value, Variables};
use meta_type_name::MetaTypeName;

use crate::registries::{AnyDirective, AnyField, AnyInputValue, AnyMetaType, AnyRegistry};

use engine_parser::{
    types::{
        Directive, ExecutableDocument, Field, FragmentDefinition, FragmentSpread, InlineFragment, OperationDefinition,
        OperationType, Selection, SelectionSet, TypeCondition, VariableDefinition,
    },
    Pos, Positioned,
};

#[doc(hidden)]
pub struct VisitorContext<'a, Registry>
where
    Registry: AnyRegistry,
{
    pub(crate) registry: &'a Registry,
    pub(crate) variables: Option<&'a Variables>,
    pub(crate) errors: Vec<RuleError>,
    type_stack: Vec<Option<Registry::MetaType<'a>>>,
    input_type: Vec<Option<Registry::MetaInputValue<'a>>>,
    fragments: &'a HashMap<Name, Positioned<FragmentDefinition>>,
}

impl<'a, Registry> VisitorContext<'a, Registry>
where
    Registry: AnyRegistry,
{
    pub(super) fn new(registry: &'a Registry, doc: &'a ExecutableDocument, variables: Option<&'a Variables>) -> Self {
        Self {
            registry,
            variables,
            errors: Default::default(),
            type_stack: Default::default(),
            input_type: Default::default(),
            fragments: &doc.fragments,
        }
    }

    pub(crate) fn report_error<T: Into<String>>(&mut self, locations: Vec<Pos>, msg: T) {
        self.errors.push(RuleError::new(locations, msg));
    }

    pub(crate) fn append_errors(&mut self, errors: Vec<RuleError>) {
        self.errors.extend(errors);
    }

    pub(crate) fn with_type<F: FnMut(&mut Self)>(&mut self, ty: Option<Registry::MetaType<'a>>, mut f: F) {
        self.type_stack.push(ty);
        f(self);
        self.type_stack.pop();
    }

    pub(crate) fn with_input_type<F: FnMut(&mut Self)>(&mut self, ty: Option<Registry::MetaInputValue<'a>>, mut f: F) {
        self.input_type.push(ty);
        f(self);
        self.input_type.pop();
    }

    pub(crate) fn parent_type(&self) -> Option<Registry::MetaType<'a>> {
        if self.type_stack.len() >= 2 {
            self.type_stack.get(self.type_stack.len() - 2).copied().flatten()
        } else {
            None
        }
    }

    pub(crate) fn current_type(&self) -> Option<Registry::MetaType<'a>> {
        self.type_stack.last().copied().flatten()
    }

    pub(crate) fn is_known_fragment(&self, name: &str) -> bool {
        self.fragments.contains_key(name)
    }

    pub(crate) fn fragment(&self, name: &str) -> Option<&'a Positioned<FragmentDefinition>> {
        self.fragments.get(name)
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub(crate) enum VisitMode {
    Normal,
    Inline,
}

pub(crate) trait Visitor<'a, Registry>
where
    Registry: AnyRegistry,
{
    fn mode(&self) -> VisitMode {
        VisitMode::Normal
    }

    fn enter_document(&mut self, _ctx: &mut VisitorContext<'a, Registry>, _doc: &'a ExecutableDocument) {}
    fn exit_document(&mut self, _ctx: &mut VisitorContext<'a, Registry>, _doc: &'a ExecutableDocument) {}

    fn enter_operation_definition(
        &mut self,
        _ctx: &mut VisitorContext<'a, Registry>,
        _name: Option<&'a Name>,
        _operation_definition: &'a Positioned<OperationDefinition>,
    ) {
    }
    fn exit_operation_definition(
        &mut self,
        _ctx: &mut VisitorContext<'a, Registry>,
        _name: Option<&'a Name>,
        _operation_definition: &'a Positioned<OperationDefinition>,
    ) {
    }

    fn enter_fragment_definition(
        &mut self,
        _ctx: &mut VisitorContext<'a, Registry>,
        _name: &'a Name,
        _fragment_definition: &'a Positioned<FragmentDefinition>,
    ) {
    }
    fn exit_fragment_definition(
        &mut self,
        _ctx: &mut VisitorContext<'a, Registry>,
        _name: &'a Name,
        _fragment_definition: &'a Positioned<FragmentDefinition>,
    ) {
    }

    fn enter_variable_definition(
        &mut self,
        _ctx: &mut VisitorContext<'a, Registry>,
        _variable_definition: &'a Positioned<VariableDefinition>,
    ) {
    }
    fn exit_variable_definition(
        &mut self,
        _ctx: &mut VisitorContext<'a, Registry>,
        _variable_definition: &'a Positioned<VariableDefinition>,
    ) {
    }

    fn enter_directive(&mut self, _ctx: &mut VisitorContext<'a, Registry>, _directive: &'a Positioned<Directive>) {}
    fn exit_directive(&mut self, _ctx: &mut VisitorContext<'a, Registry>, _directive: &'a Positioned<Directive>) {}

    fn enter_argument(
        &mut self,
        _ctx: &mut VisitorContext<'a, Registry>,
        _name: &'a Positioned<Name>,
        _value: &'a Positioned<Value>,
    ) {
    }
    fn exit_argument(
        &mut self,
        _ctx: &mut VisitorContext<'a, Registry>,
        _name: &'a Positioned<Name>,
        _value: &'a Positioned<Value>,
    ) {
    }

    fn enter_selection_set(
        &mut self,
        _ctx: &mut VisitorContext<'a, Registry>,
        _selection_set: &'a Positioned<SelectionSet>,
    ) {
    }
    fn exit_selection_set(
        &mut self,
        _ctx: &mut VisitorContext<'a, Registry>,
        _selection_set: &'a Positioned<SelectionSet>,
    ) {
    }

    fn enter_selection(&mut self, _ctx: &mut VisitorContext<'a, Registry>, _selection: &'a Positioned<Selection>) {}
    fn exit_selection(&mut self, _ctx: &mut VisitorContext<'a, Registry>, _selection: &'a Positioned<Selection>) {}

    fn enter_field(&mut self, _ctx: &mut VisitorContext<'a, Registry>, _field: &'a Positioned<Field>) {}
    fn exit_field(&mut self, _ctx: &mut VisitorContext<'a, Registry>, _field: &'a Positioned<Field>) {}

    fn enter_fragment_spread(
        &mut self,
        _ctx: &mut VisitorContext<'a, Registry>,
        _fragment_spread: &'a Positioned<FragmentSpread>,
    ) {
    }
    fn exit_fragment_spread(
        &mut self,
        _ctx: &mut VisitorContext<'a, Registry>,
        _fragment_spread: &'a Positioned<FragmentSpread>,
    ) {
    }

    fn enter_inline_fragment(
        &mut self,
        _ctx: &mut VisitorContext<'a, Registry>,
        _inline_fragment: &'a Positioned<InlineFragment>,
    ) {
    }
    fn exit_inline_fragment(
        &mut self,
        _ctx: &mut VisitorContext<'a, Registry>,
        _inline_fragment: &'a Positioned<InlineFragment>,
    ) {
    }

    fn enter_input_value(
        &mut self,
        _ctx: &mut VisitorContext<'a, Registry>,
        _pos: Pos,
        _expected_type: &Option<MetaTypeName<'_>>,
        _value: &'a Value,
        _meta: Option<Registry::MetaInputValue<'a>>,
    ) {
    }
    fn exit_input_value(
        &mut self,
        _ctx: &mut VisitorContext<'a, Registry>,
        _pos: Pos,
        _expected_type: &Option<MetaTypeName<'_>>,
        _value: &Value,
        _meta: Option<Registry::MetaInputValue<'a>>,
    ) {
    }
}

pub(crate) struct VisitorNil;

impl VisitorNil {
    pub(crate) fn with<V>(self, visitor: V) -> VisitorCons<V, Self> {
        VisitorCons(visitor, self)
    }
}

pub(crate) struct VisitorCons<A, B>(A, B);

impl<A, B> VisitorCons<A, B> {
    pub(crate) fn with<V>(self, visitor: V) -> VisitorCons<V, Self> {
        VisitorCons(visitor, self)
    }
}

impl<'a, Registry: AnyRegistry> Visitor<'a, Registry> for VisitorNil {}

impl<'ctx, 'a, 'b, A, B, Registry> Visitor<'ctx, Registry> for VisitorCons<A, B>
where
    Registry: AnyRegistry,
    A: Visitor<'ctx, Registry> + 'a,
    B: Visitor<'ctx, Registry> + 'b,
    'ctx: 'a,
    'ctx: 'b,
{
    fn enter_document(&mut self, ctx: &mut VisitorContext<'ctx, Registry>, doc: &'ctx ExecutableDocument) {
        self.0.enter_document(ctx, doc);
        self.1.enter_document(ctx, doc);
    }

    fn exit_document(&mut self, ctx: &mut VisitorContext<'ctx, Registry>, doc: &'ctx ExecutableDocument) {
        self.0.exit_document(ctx, doc);
        self.1.exit_document(ctx, doc);
    }

    fn enter_operation_definition(
        &mut self,
        ctx: &mut VisitorContext<'ctx, Registry>,
        name: Option<&'ctx Name>,
        operation_definition: &'ctx Positioned<OperationDefinition>,
    ) {
        self.0.enter_operation_definition(ctx, name, operation_definition);
        self.1.enter_operation_definition(ctx, name, operation_definition);
    }

    fn exit_operation_definition(
        &mut self,
        ctx: &mut VisitorContext<'ctx, Registry>,
        name: Option<&'ctx Name>,
        operation_definition: &'ctx Positioned<OperationDefinition>,
    ) {
        self.0.exit_operation_definition(ctx, name, operation_definition);
        self.1.exit_operation_definition(ctx, name, operation_definition);
    }

    fn enter_fragment_definition(
        &mut self,
        ctx: &mut VisitorContext<'ctx, Registry>,
        name: &'ctx Name,
        fragment_definition: &'ctx Positioned<FragmentDefinition>,
    ) {
        self.0.enter_fragment_definition(ctx, name, fragment_definition);
        self.1.enter_fragment_definition(ctx, name, fragment_definition);
    }

    fn exit_fragment_definition(
        &mut self,
        ctx: &mut VisitorContext<'ctx, Registry>,
        name: &'ctx Name,
        fragment_definition: &'ctx Positioned<FragmentDefinition>,
    ) {
        self.0.exit_fragment_definition(ctx, name, fragment_definition);
        self.1.exit_fragment_definition(ctx, name, fragment_definition);
    }

    fn enter_variable_definition(
        &mut self,
        ctx: &mut VisitorContext<'ctx, Registry>,
        variable_definition: &'ctx Positioned<VariableDefinition>,
    ) {
        self.0.enter_variable_definition(ctx, variable_definition);
        self.1.enter_variable_definition(ctx, variable_definition);
    }

    fn exit_variable_definition(
        &mut self,
        ctx: &mut VisitorContext<'ctx, Registry>,
        variable_definition: &'ctx Positioned<VariableDefinition>,
    ) {
        self.0.exit_variable_definition(ctx, variable_definition);
        self.1.exit_variable_definition(ctx, variable_definition);
    }

    fn enter_directive(&mut self, ctx: &mut VisitorContext<'ctx, Registry>, directive: &'ctx Positioned<Directive>) {
        self.0.enter_directive(ctx, directive);
        self.1.enter_directive(ctx, directive);
    }

    fn exit_directive(&mut self, ctx: &mut VisitorContext<'ctx, Registry>, directive: &'ctx Positioned<Directive>) {
        self.0.exit_directive(ctx, directive);
        self.1.exit_directive(ctx, directive);
    }

    fn enter_argument(
        &mut self,
        ctx: &mut VisitorContext<'ctx, Registry>,
        name: &'ctx Positioned<Name>,
        value: &'ctx Positioned<Value>,
    ) {
        self.0.enter_argument(ctx, name, value);
        self.1.enter_argument(ctx, name, value);
    }

    fn exit_argument(
        &mut self,
        ctx: &mut VisitorContext<'ctx, Registry>,
        name: &'ctx Positioned<Name>,
        value: &'ctx Positioned<Value>,
    ) {
        self.0.exit_argument(ctx, name, value);
        self.1.exit_argument(ctx, name, value);
    }

    fn enter_selection_set(
        &mut self,
        ctx: &mut VisitorContext<'ctx, Registry>,
        selection_set: &'ctx Positioned<SelectionSet>,
    ) {
        self.0.enter_selection_set(ctx, selection_set);
        self.1.enter_selection_set(ctx, selection_set);
    }

    fn exit_selection_set(
        &mut self,
        ctx: &mut VisitorContext<'ctx, Registry>,
        selection_set: &'ctx Positioned<SelectionSet>,
    ) {
        self.0.exit_selection_set(ctx, selection_set);
        self.1.exit_selection_set(ctx, selection_set);
    }

    fn enter_selection(&mut self, ctx: &mut VisitorContext<'ctx, Registry>, selection: &'ctx Positioned<Selection>) {
        self.0.enter_selection(ctx, selection);
        self.1.enter_selection(ctx, selection);
    }

    fn exit_selection(&mut self, ctx: &mut VisitorContext<'ctx, Registry>, selection: &'ctx Positioned<Selection>) {
        self.0.exit_selection(ctx, selection);
        self.1.exit_selection(ctx, selection);
    }

    fn enter_field(&mut self, ctx: &mut VisitorContext<'ctx, Registry>, field: &'ctx Positioned<Field>) {
        self.0.enter_field(ctx, field);
        self.1.enter_field(ctx, field);
    }

    fn exit_field(&mut self, ctx: &mut VisitorContext<'ctx, Registry>, field: &'ctx Positioned<Field>) {
        self.0.exit_field(ctx, field);
        self.1.exit_field(ctx, field);
    }

    fn enter_fragment_spread(
        &mut self,
        ctx: &mut VisitorContext<'ctx, Registry>,
        fragment_spread: &'ctx Positioned<FragmentSpread>,
    ) {
        self.0.enter_fragment_spread(ctx, fragment_spread);
        self.1.enter_fragment_spread(ctx, fragment_spread);
    }

    fn exit_fragment_spread(
        &mut self,
        ctx: &mut VisitorContext<'ctx, Registry>,
        fragment_spread: &'ctx Positioned<FragmentSpread>,
    ) {
        self.0.exit_fragment_spread(ctx, fragment_spread);
        self.1.exit_fragment_spread(ctx, fragment_spread);
    }

    fn enter_inline_fragment(
        &mut self,
        ctx: &mut VisitorContext<'ctx, Registry>,
        inline_fragment: &'ctx Positioned<InlineFragment>,
    ) {
        self.0.enter_inline_fragment(ctx, inline_fragment);
        self.1.enter_inline_fragment(ctx, inline_fragment);
    }

    fn exit_inline_fragment(
        &mut self,
        ctx: &mut VisitorContext<'ctx, Registry>,
        inline_fragment: &'ctx Positioned<InlineFragment>,
    ) {
        self.0.exit_inline_fragment(ctx, inline_fragment);
        self.1.exit_inline_fragment(ctx, inline_fragment);
    }

    fn enter_input_value(
        &mut self,
        ctx: &mut VisitorContext<'ctx, Registry>,
        pos: Pos,
        expected_type: &Option<MetaTypeName<'_>>,
        value: &'ctx Value,
        meta: Option<Registry::MetaInputValue<'ctx>>,
    ) {
        self.0.enter_input_value(ctx, pos, expected_type, value, meta);
        self.1.enter_input_value(ctx, pos, expected_type, value, meta);
    }

    fn exit_input_value(
        &mut self,
        ctx: &mut VisitorContext<'ctx, Registry>,
        pos: Pos,
        expected_type: &Option<MetaTypeName<'_>>,
        value: &Value,
        meta: Option<Registry::MetaInputValue<'ctx>>,
    ) {
        self.0.exit_input_value(ctx, pos, expected_type, value, meta);
        self.1.exit_input_value(ctx, pos, expected_type, value, meta);
    }
}

pub(crate) fn visit<'a, Registry: AnyRegistry, V: Visitor<'a, Registry>>(
    v: &mut V,
    ctx: &mut VisitorContext<'a, Registry>,
    doc: &'a ExecutableDocument,
) {
    v.enter_document(ctx, doc);

    for (name, fragment) in &doc.fragments {
        ctx.with_type(
            ctx.registry
                .lookup_type(fragment.node.type_condition.node.on.node.as_str()),
            |ctx| visit_fragment_definition(v, ctx, name, fragment),
        );
    }

    for (name, operation) in doc.operations.iter() {
        visit_operation_definition(v, ctx, name, operation);
    }

    v.exit_document(ctx, doc);
}

fn visit_operation_definition<'a, Registry: AnyRegistry, V: Visitor<'a, Registry>>(
    v: &mut V,
    ctx: &mut VisitorContext<'a, Registry>,
    name: Option<&'a Name>,
    operation: &'a Positioned<OperationDefinition>,
) {
    v.enter_operation_definition(ctx, name, operation);
    let root_ty = match &operation.node.ty {
        OperationType::Query => Some(ctx.registry.query_type()),
        OperationType::Mutation => ctx.registry.mutation_type(),
        OperationType::Subscription => ctx.registry.subscription_type(),
    };

    let is_subscription = matches!(operation.node.ty, OperationType::Subscription);

    if let Some(root) = root_ty {
        ctx.with_type(Some(root), |ctx| {
            visit_variable_definitions(v, ctx, &operation.node.variable_definitions);
            visit_directives(v, ctx, &operation.node.directives);
            visit_selection_set(v, ctx, &operation.node.selection_set, is_subscription);
        });
    } else {
        ctx.report_error(
            vec![operation.pos],
            // The only one with an irregular plural, "query", is always present
            format!("Schema is not configured for {}s.", operation.node.ty),
        );
    }
    v.exit_operation_definition(ctx, name, operation);
}

fn visit_selection_set<'a, Registry: AnyRegistry, V: Visitor<'a, Registry>>(
    v: &mut V,
    ctx: &mut VisitorContext<'a, Registry>,
    selection_set: &'a Positioned<SelectionSet>,
    is_subscription: bool,
) {
    if !selection_set.node.items.is_empty() {
        v.enter_selection_set(ctx, selection_set);
        for selection in &selection_set.node.items {
            visit_selection(v, ctx, selection, is_subscription);
        }
        v.exit_selection_set(ctx, selection_set);
    }
}

fn visit_selection<'a, Registry: AnyRegistry, V: Visitor<'a, Registry>>(
    v: &mut V,
    ctx: &mut VisitorContext<'a, Registry>,
    selection: &'a Positioned<Selection>,
    is_subscription: bool,
) {
    v.enter_selection(ctx, selection);
    match &selection.node {
        Selection::Field(field) => {
            if field.node.name.node != "__typename" {
                ctx.with_type(
                    ctx.current_type()
                        .and_then(|ty| ty.field(&field.node.name.node))
                        .map(|schema_field| schema_field.named_type()),
                    |ctx| {
                        visit_field(v, ctx, field);
                    },
                );
            } else if is_subscription {
                ctx.report_error(
                    vec![field.pos],
                    "Unknown field \"__typename\" on type \"Subscription\".",
                );
            }
        }
        Selection::FragmentSpread(fragment_spread) => visit_fragment_spread(v, ctx, fragment_spread),
        Selection::InlineFragment(inline_fragment) => {
            if let Some(TypeCondition { on: name }) = &inline_fragment.node.type_condition.as_ref().map(|c| &c.node) {
                ctx.with_type(ctx.registry.lookup_type(name.node.as_str()), |ctx| {
                    visit_inline_fragment(v, ctx, inline_fragment);
                });
            } else {
                visit_inline_fragment(v, ctx, inline_fragment);
            }
        }
    }
    v.exit_selection(ctx, selection);
}

fn visit_field<'a, Registry: AnyRegistry, V: Visitor<'a, Registry>>(
    v: &mut V,
    ctx: &mut VisitorContext<'a, Registry>,
    field: &'a Positioned<Field>,
) {
    v.enter_field(ctx, field);
    for (name, value) in &field.node.arguments {
        v.enter_argument(ctx, name, value);
        let meta_input_value = ctx
            .parent_type()
            .and_then(|ty| ty.field(&field.node.name.node))
            .and_then(|schema_field| schema_field.argument(&name.node));

        let type_string = meta_input_value.map(|value| value.type_string());
        let expected_ty = type_string
            .as_ref()
            .map(|type_string| MetaTypeName::create(type_string));

        ctx.with_input_type(meta_input_value, |ctx| {
            visit_input_value(v, ctx, field.pos, expected_ty, &value.node, meta_input_value);
        });
        v.exit_argument(ctx, name, value);
    }

    visit_directives(v, ctx, &field.node.directives);
    visit_selection_set(v, ctx, &field.node.selection_set, false);
    v.exit_field(ctx, field);
}

fn visit_input_value<'a, Registry: AnyRegistry, V: Visitor<'a, Registry>>(
    v: &mut V,
    ctx: &mut VisitorContext<'a, Registry>,
    pos: Pos,
    expected_ty: Option<MetaTypeName<'_>>,
    value: &'a Value,
    meta: Option<Registry::MetaInputValue<'a>>,
) {
    v.enter_input_value(ctx, pos, &expected_ty, value, meta);

    match value {
        Value::List(values) => {
            if let Some(expected_ty) = expected_ty {
                let elem_ty = expected_ty.unwrap_non_null();
                if let MetaTypeName::List(expected_ty) = elem_ty {
                    let (_name, _description) = ctx
                        .registry
                        .lookup_type(MetaTypeName::concrete_typename(expected_ty))
                        .map(|meta_type| (meta_type.name().to_string(), meta_type.description().map(String::from)))
                        .unwrap_or_default();
                    for value in values {
                        visit_input_value(v, ctx, pos, Some(MetaTypeName::create(expected_ty)), value, None);
                    }
                }
            }
        }
        Value::Object(values) => {
            if let Some(expected_ty) = expected_ty {
                let expected_ty = expected_ty.unwrap_non_null();
                if let MetaTypeName::Named(expected_ty) = expected_ty {
                    let maybe_type = ctx.registry.lookup_type(MetaTypeName::concrete_typename(expected_ty));

                    let is_input_object = maybe_type.as_ref().map(|ty| ty.is_input_object()).unwrap_or_default();

                    if is_input_object {
                        let input_object = maybe_type.unwrap();
                        for (item_key, item_value) in values {
                            if let Some(input_value) = input_object.input_field(item_key.as_str()) {
                                let typename = input_value.type_string();
                                visit_input_value(
                                    v,
                                    ctx,
                                    pos,
                                    Some(MetaTypeName::create(&typename)),
                                    item_value,
                                    Some(input_value),
                                );
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }

    v.exit_input_value(ctx, pos, &expected_ty, value, meta);
}

fn visit_variable_definitions<'a, Registry: AnyRegistry, V: Visitor<'a, Registry>>(
    v: &mut V,
    ctx: &mut VisitorContext<'a, Registry>,
    variable_definitions: &'a [Positioned<VariableDefinition>],
) {
    for d in variable_definitions {
        v.enter_variable_definition(ctx, d);
        v.exit_variable_definition(ctx, d);
    }
}

fn visit_directives<'a, Registry: AnyRegistry, V: Visitor<'a, Registry>>(
    v: &mut V,
    ctx: &mut VisitorContext<'a, Registry>,
    directives: &'a [Positioned<Directive>],
) {
    for d in directives {
        v.enter_directive(ctx, d);

        let schema_directive = ctx
            .registry
            .directives()
            .find(|directive| directive.name() == d.node.name.node.as_str());

        for (name, value) in &d.node.arguments {
            v.enter_argument(ctx, name, value);
            let meta_input_value = schema_directive.and_then(|schema_directive| schema_directive.argument(&name.node));
            let expected_typename = meta_input_value.map(|input_value| input_value.type_string());
            let expected_ty = expected_typename.as_ref().map(|name| MetaTypeName::create(name));
            ctx.with_input_type(meta_input_value, |ctx| {
                visit_input_value(v, ctx, d.pos, expected_ty, &value.node, meta_input_value);
            });
            v.exit_argument(ctx, name, value);
        }

        v.exit_directive(ctx, d);
    }
}

fn visit_fragment_definition<'a, Registry: AnyRegistry, V: Visitor<'a, Registry>>(
    v: &mut V,
    ctx: &mut VisitorContext<'a, Registry>,
    name: &'a Name,
    fragment: &'a Positioned<FragmentDefinition>,
) {
    if v.mode() == VisitMode::Normal {
        v.enter_fragment_definition(ctx, name, fragment);
        visit_directives(v, ctx, &fragment.node.directives);
        visit_selection_set(v, ctx, &fragment.node.selection_set, false);
        v.exit_fragment_definition(ctx, name, fragment);
    }
}

fn visit_fragment_spread<'a, Registry: AnyRegistry, V: Visitor<'a, Registry>>(
    v: &mut V,
    ctx: &mut VisitorContext<'a, Registry>,
    fragment_spread: &'a Positioned<FragmentSpread>,
) {
    v.enter_fragment_spread(ctx, fragment_spread);
    visit_directives(v, ctx, &fragment_spread.node.directives);
    if v.mode() == VisitMode::Inline {
        if let Some(fragment) = ctx.fragments.get(fragment_spread.node.fragment_name.node.as_str()) {
            visit_selection_set(v, ctx, &fragment.node.selection_set, false);
        }
    }
    v.exit_fragment_spread(ctx, fragment_spread);
}

fn visit_inline_fragment<'a, Registry: AnyRegistry, V: Visitor<'a, Registry>>(
    v: &mut V,
    ctx: &mut VisitorContext<'a, Registry>,
    inline_fragment: &'a Positioned<InlineFragment>,
) {
    v.enter_inline_fragment(ctx, inline_fragment);
    visit_directives(v, ctx, &inline_fragment.node.directives);
    visit_selection_set(v, ctx, &inline_fragment.node.selection_set, false);
    v.exit_inline_fragment(ctx, inline_fragment);
}

#[derive(Debug, PartialEq)]
pub struct RuleError {
    pub locations: Vec<Pos>,
    pub message: String,
}

impl RuleError {
    pub(crate) fn new(locations: Vec<Pos>, msg: impl Into<String>) -> Self {
        Self {
            locations,
            message: msg.into(),
        }
    }
}

impl Display for RuleError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for (idx, loc) in self.locations.iter().enumerate() {
            if idx == 0 {
                write!(f, "[")?;
            } else {
                write!(f, ", ")?;
            }

            write!(f, "{}:{}", loc.line, loc.column)?;

            if idx == self.locations.len() - 1 {
                write!(f, "] ")?;
            }
        }

        write!(f, "{}", self.message)?;
        Ok(())
    }
}

#[cfg(test)]
pub(crate) mod test {

    use super::{MetaTypeName, Pos, Value, Visitor, VisitorContext};

    #[allow(dead_code)]
    pub(crate) fn visit_input_value<'a, V: Visitor<'a, registry_v2::Registry>>(
        v: &mut V,
        ctx: &mut VisitorContext<'a, registry_v2::Registry>,
        pos: Pos,
        expected_ty: Option<MetaTypeName<'_>>,
        value: &'a Value,
        meta: Option<registry_v2::MetaInputValue<'a>>,
    ) {
        super::visit_input_value(v, ctx, pos, expected_ty, value, meta);
    }
}
