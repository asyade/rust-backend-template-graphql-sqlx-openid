use async_graphql::{EmptySubscription, Schema};
use rave_entity::async_graphql;

use crate::{mutation::Mutation, query::Query};
use crate::prelude::*;
use rave_api_service_feed_provider::FeedProvider;

pub type AppSchema = Schema<Query, Mutation, EmptySubscription>;

/// Builds the GraphQL Schema, attaching the Database to the context
pub async fn build_schema(db: Database) -> RaveApiResult<AppSchema> {
    let feed = FeedProvider::new().await;
    let schema = Schema::build(Query::default(), Mutation::default(), EmptySubscription)
        .data(db)
        .data(feed)
        .finish();
    Ok(schema)
}
