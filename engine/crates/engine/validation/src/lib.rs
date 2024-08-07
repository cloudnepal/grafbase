#[cfg(test)]
#[macro_use]
mod test_harness;

pub mod dynamic_validators;
mod registries;
mod rules;
mod suggestion;
pub mod utils;
mod visitor;
mod visitors;

use std::collections::{HashMap, HashSet};

use engine_response::error::ServerError;
use engine_value::Variables;
use registry_for_cache::PartialCacheRegistry;
pub use visitor::VisitorContext;
use visitor::{visit, VisitorNil};

use engine_parser::types::ExecutableDocument;
use registry_v2::{cache_control::CacheInvalidationPolicy, CacheControl};

pub use visitor::RuleError;

/// Validation results.
#[derive(Debug, Clone)]
pub struct ValidationResult {
    /// Cache control
    pub cache_control: CacheControl,

    /// Cache mutation invalidation policies
    pub cache_invalidation_policies: HashSet<CacheInvalidation>,

    /// Query complexity
    pub complexity: usize,

    /// Query depth
    pub depth: usize,

    /// Query height
    ///
    /// Limits the number of unique fields included in an operation, including fields of fragments. If a particular field is included multiple times via aliases, it's counted only once.
    pub height: usize,

    /// Root fields in the query
    pub root_field_count: usize,

    /// Alias  count.
    pub alias_count: usize,

    /// Matches the format specified by OperationMetricsAttributes
    pub used_fields: Option<String>,
}

#[derive(Clone, PartialEq, Eq, Debug, serde::Deserialize, serde::Serialize, Hash)]
pub struct CacheInvalidation {
    pub ty: String,
    pub policy: CacheInvalidationPolicy,
}

pub fn check_strict_rules(
    registry: &registry_v2::Registry,
    doc: &ExecutableDocument,
    variables: Option<&Variables>,
) -> Result<ValidationResult, Vec<RuleError>> {
    let mut ctx = VisitorContext::new(registry, doc, variables);
    let mut cache_control = None;
    let mut cache_invalidation_policies = Default::default();
    let complexity = 0;
    let mut depth = 0;
    let mut root_field_count: usize = 0;
    let mut height: usize = 0;
    let mut alias_count: usize = 0;
    let mut used_fields = HashMap::new();

    let mut visitor = VisitorNil
        .with(rules::ArgumentsOfCorrectType::default())
        .with(rules::DefaultValuesOfCorrectType)
        .with(rules::FieldsOnCorrectType)
        .with(rules::FragmentsOnCompositeTypes)
        .with(rules::KnownArgumentNames::default())
        .with(rules::NoFragmentCycles::default())
        .with(rules::KnownFragmentNames)
        .with(rules::KnownTypeNames)
        .with(rules::NoUndefinedVariables::default())
        .with(rules::NoUnusedFragments::default())
        .with(rules::NoUnusedVariables::default())
        .with(rules::UniqueArgumentNames::default())
        .with(rules::UniqueVariableNames::default())
        .with(rules::VariablesAreInputTypes)
        .with(rules::VariableInAllowedPosition::default())
        .with(rules::ScalarLeafs)
        .with(rules::PossibleFragmentSpreads::default())
        .with(rules::ProvidedNonNullArguments)
        .with(rules::KnownDirectives::default())
        .with(rules::DirectivesUnique)
        .with(rules::OverlappingFieldsCanBeMerged)
        .with(visitors::CacheControlCalculate::new(
            &mut cache_control,
            &mut cache_invalidation_policies,
        ))
        .with(visitors::DepthCalculate::new(&mut depth))
        .with(visitors::HeightCalculate::new(&mut height))
        .with(visitors::AliasCountCalculate::new(&mut alias_count))
        .with(visitors::RootFieldCountCalculate::new(&mut root_field_count))
        .with(visitors::InputValidationVisitor)
        .with(visitors::UsedFieldsAggregator(&mut used_fields));

    visit(&mut visitor, &mut ctx, doc);

    if !ctx.errors.is_empty() {
        return Err(ctx.errors);
    }

    let used_fields = visitors::UsedFieldsAggregator(&mut used_fields).finalize();
    Ok(ValidationResult {
        cache_control: cache_control.unwrap_or_default(),
        cache_invalidation_policies,
        complexity,
        depth,
        root_field_count,
        height,
        alias_count,
        used_fields: Some(used_fields),
    })
}

pub fn check_fast_rules(
    registry: &PartialCacheRegistry,
    doc: &ExecutableDocument,
    variables: Option<&Variables>,
) -> Result<ValidationResult, Vec<RuleError>> {
    let mut ctx = VisitorContext::new(registry, doc, variables);
    let mut cache_control = None;
    let mut cache_invalidation_policies = Default::default();
    let complexity = 0;
    let mut depth = 0;
    let mut root_field_count: usize = 0;
    let mut height: usize = 0;
    let mut alias_count: usize = 0;

    let mut visitor = VisitorNil
        .with(rules::NoFragmentCycles::default())
        .with(visitors::CacheControlCalculate::new(
            &mut cache_control,
            &mut cache_invalidation_policies,
        ))
        .with(visitors::DepthCalculate::new(&mut depth))
        .with(visitors::HeightCalculate::new(&mut height))
        .with(visitors::AliasCountCalculate::new(&mut alias_count))
        .with(visitors::RootFieldCountCalculate::new(&mut root_field_count));

    visit(&mut visitor, &mut ctx, doc);

    if !ctx.errors.is_empty() {
        return Err(ctx.errors);
    }

    Ok(ValidationResult {
        cache_control: cache_control.unwrap_or_default(),
        cache_invalidation_policies,
        complexity,
        depth,
        root_field_count,
        height,
        alias_count,
        used_fields: None,
    })
}

impl From<RuleError> for ServerError {
    fn from(e: RuleError) -> Self {
        Self {
            message: e.message,
            source: None,
            locations: e.locations,
            path: Vec::new(),
            extensions: None,
        }
    }
}
