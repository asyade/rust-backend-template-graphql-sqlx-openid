use async_graphql::{EmptySubscription, Schema};
use rave_entity::async_graphql;

use crate::graphql::{mutation::Mutation, query::Query};
use crate::prelude::RaveApiResult;
use crate::services::database::Database;

pub type AppSchema = Schema<Query, Mutation, EmptySubscription>;

/// Builds the GraphQL Schema, attaching the Database to the context
pub async fn build_schema() -> RaveApiResult<AppSchema> {
    let db = Database::new().await?;
    let schema = Schema::build(Query::default(), Mutation::default(), EmptySubscription)
        .data(db)
        .finish();
    Ok(schema)
}