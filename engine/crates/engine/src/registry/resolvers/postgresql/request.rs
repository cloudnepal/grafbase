mod find_many;
mod find_one;
mod query;

use super::{context::PostgresContext, Operation};
use crate::{registry::resolvers::ResolvedValue, Error};

pub(super) async fn execute(ctx: PostgresContext<'_>, operation: Operation) -> Result<ResolvedValue, Error> {
    match operation {
        Operation::FindOne => find_one::execute(ctx).await,
        Operation::FindMany => find_many::execute(ctx).await,
    }
}