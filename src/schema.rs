use crate::resolvers::QueryRoot;
use async_graphql::{EmptyMutation, EmptySubscription, Schema};

pub type AppSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

pub fn build_schema() -> AppSchema {
    Schema::build(QueryRoot, EmptyMutation, EmptySubscription).finish()
}
