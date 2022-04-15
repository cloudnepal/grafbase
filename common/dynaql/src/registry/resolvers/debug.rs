use super::{ResolverContext, ResolverTrait};
use crate::{Context, Error};

#[non_exhaustive]
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub enum DebugResolver {
    Value { inner: serde_json::Value },
}

#[async_trait::async_trait]
impl ResolverTrait for DebugResolver {
    async fn resolve(
        &self,
        _ctx: &Context<'_>,
        _resolver_ctx: &ResolverContext<'_>,
    ) -> Result<serde_json::Value, Error> {
        match &self {
            Self::Value { inner } => Ok(inner.clone()),
        }
    }
}
