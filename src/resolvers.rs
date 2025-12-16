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

        Ok(rows.into_iter().map(Dataset::from).collect())
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

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    pub async fn create_dataset(
        &self,
        ctx: &Context<'_>,
        name: String,
        description: Option<String>,
    ) -> anyhow::Result<i32> {
        let pool = get_pg_pool(ctx);

        let id: i32 = sqlx::query_scalar(
            r#"INSERT INTO datasets (name, description)
            VALUES ($1, $2)
            RETURNING id;"#,
        )
        .bind(name)
        .bind(description)
        .fetch_one(pool)
        .await?;

        Ok(id)
    }

    pub async fn create_location(
        &self,
        ctx: &Context<'_>,
        name: String,
        lat: f64,
        lon: f64,
    ) -> anyhow::Result<i32> {
        let pool = get_pg_pool(ctx);

        let id: i32 = sqlx::query_scalar(
            r#"INSERT INTO locations (name, lat, lon)
            VALUES ($1, $2)
            RETURNING id;"#,
        )
        .bind(name)
        .bind(lat)
        .bind(lon)
        .fetch_one(pool)
        .await?;

        Ok(id)
    }

    pub async fn record_into_dataset(
        &self,
        ctx: &Context<'_>,
        dataset_id: i32,
        location_id: Option<i32>,
        data: serde_json::Value,
    ) -> anyhow::Result<Record> {
        let pool = get_pg_pool(ctx);

        let row: RecordRow = sqlx::query_as(
            r#"
                INSERT INTO records (dataset_id, location_id, data)
                VALUES ($1, $2, $3)
                RETURNING *;
            "#,
        )
        .bind(dataset_id)
        .bind(location_id)
        .bind(data)
        .fetch_one(pool)
        .await?;

        Ok(Record::from(row))
    }
}
