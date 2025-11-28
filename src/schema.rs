use crate::resolvers::QueryRoot;
use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use sqlx::{PgPool, postgres::PgPoolOptions};
use std::{env, time::Duration};
use tokio::time::timeout;

pub type AppSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

pub async fn build_schema() -> anyhow::Result<AppSchema> {
    let pool = create_pool().await?;
    Ok(Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .data(pool)
        .finish())
}

async fn create_pool() -> anyhow::Result<PgPool> {
    dotenv::dotenv()?;
    let user = env::var("POSTGRES_USER")?;
    let password = env::var("POSTGRES_PASSWORD")?;
    let db = env::var("POSTGRES_DB")?;
    let port = env::var("POSTGRES_PORT")?;

    let url = format!("postgres://{user}:{password}@localhost:{port}/{db}");

    let pool = timeout(
        Duration::from_secs(5),
        PgPoolOptions::new().max_connections(10).connect(&url),
    )
    .await??;

    Ok(pool)
}
