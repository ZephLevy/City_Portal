use crate::models::*;
use async_graphql::{Context, Object};
use sqlx::PgPool;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    pub async fn datasets(&self, ctx: &Context<'_>) -> anyhow::Result<Vec<Dataset>> {
        let pool = ctx
            .data::<PgPool>()
            .expect("A database connection does not exist");

        let rows: Vec<DatasetRow> = sqlx::query_as("SELECT * FROM datasets")
            .fetch_all(pool)
            .await?;

        let result = rows.into_iter().map(|r| Dataset::from(r)).collect();
        return Ok(result);
    }

    async fn query_dataset(&self, id: i32) -> anyhow::Result<Record> {
        unimplemented!("This doesn't work yet");
    }
}
