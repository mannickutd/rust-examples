use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use sqlx::postgres::PgRow;
use sqlx::{FromRow, Row};

#[derive(Serialize, Deserialize, Clone)]
pub struct Todo {
    pub id: Option<i64>,
    pub name: String,
    pub description: String,
    pub date_added: Option<DateTime<Utc>>,
}

impl<'c> FromRow<'c, PgRow<'c>> for Todo {
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        Ok(Todo {
            id: row.get(0),
            name: row.get(1),
            description: row.get(2),
            date_added: row.get(3),
        })
    }
}