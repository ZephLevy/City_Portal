use crate::models::*;
use async_graphql::{Context, Object};
use sqlx::PgPool;

pub struct QueryRoot;

fn get_pg_pool<'a>(ctx: &'a Context<'_>) -> &'a PgPool {
    ctx.data::<PgPool>()
        .expect("A database connection does not exist")
}

#[Object]
impl QueryRoot {
    pub async fn datasets(&self, ctx: &Context<'_>) -> anyhow::Result<Vec<Dataset>> {
        let pool = get_pg_pool(ctx);

        let rows: Vec<DatasetRow> = sqlx::query_as("SELECT * FROM datasets")
            .fetch_all(pool)
            .await?;

        return Ok(rows.into_iter().map(|r| Dataset::from(r)).collect());
    }

    pub async fn query_dataset(
        &self,
        ctx: &Context<'_>,
        id: i32,
    ) -> anyhow::Result<Option<Record>> {
        let pool = get_pg_pool(ctx);

        let row: Option<RecordRow> = sqlx::query_as(
            r#"SELECT * FROM records
            WHERE dataset_id = $1
            ORDER BY timestamp DESC
            LIMIT 1;"#,
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;

        Ok(row.map(Into::into))
    }

    pub async fn get_location(&self, ctx: &Context<'_>, id: i32) -> anyhow::Result<Location> {
        let pool = get_pg_pool(ctx);

        let row: LocationRow = sqlx::query_as(
            r#"SELECT * FROM locations
            WHERE id = $1
            LIMIT 1;"#,
        )
        .bind(id)
        .fetch_one(pool)
        .await?;

        Ok(row.into())
    }
}
