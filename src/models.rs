use async_graphql::{ID, Json, SimpleObject};
use serde_json::Value;

#[derive(SimpleObject)]
pub struct Dataset {
    pub id: ID,
    pub name: String,
    pub description: Option<String>,
}

#[derive(sqlx::FromRow)]
pub struct DatasetRow {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
}

impl From<DatasetRow> for Dataset {
    fn from(row: DatasetRow) -> Self {
        Dataset {
            id: row.id.into(),
            name: row.name,
            description: row.description,
        }
    }
}

#[derive(SimpleObject)]
pub struct Location {
    pub id: ID,
    pub name: Option<String>,
    pub lat: f64,
    pub lon: f64,
}

#[derive(sqlx::FromRow)]
pub struct LocationRow {
    pub id: i32,
    pub name: Option<String>,
    pub lat: f64,
    pub lon: f64,
}

impl From<LocationRow> for Location {
    fn from(row: LocationRow) -> Self {
        Location {
            id: row.id.into(),
            name: row.name,
            lat: row.lat,
            lon: row.lon,
        }
    }
}

#[derive(SimpleObject)]
pub struct Record {
    pub id: ID,
    pub dataset_id: ID,
    pub location_id: Option<ID>,
    pub timestamp: String,
    pub data: Option<Json<Value>>,
}

#[derive(sqlx::FromRow)]
pub struct RecordRow {
    pub id: i32,
    pub dataset_id: i32,
    pub location_id: Option<i32>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub data: Option<Value>,
}

impl From<RecordRow> for Record {
    fn from(row: RecordRow) -> Self {
        Record {
            id: row.id.into(),
            dataset_id: row.dataset_id.into(),
            location_id: row.location_id.map(|id| id.into()),
            timestamp: row.timestamp.to_rfc3339(),
            data: row.data.map(Json),
        }
    }
}
