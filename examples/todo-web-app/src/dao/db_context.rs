use super::{Todo};
use sqlx::postgres::{PgRow, PgPool};
use sqlx::{FromRow};
use std::sync::Arc;


pub struct Table<'c, T>
where
    T: FromRow<'c, PgRow<'c>>,
{
    pub pool: Arc<PgPool>,
    _from_row: fn(&PgRow<'c>) -> Result<T, sqlx::Error>,
}

impl<'c, T> Table<'c, T>
where
    T: FromRow<'c, PgRow<'c>>,
{
    fn new(pool: Arc<PgPool>) -> Self {
        Table {
            pool,
            _from_row: T::from_row,
        }
    }
}


pub struct Database<'c> {
    pub todos: Arc<Table<'c, Todo>>,
}

impl Database<'_> {
    pub async fn new(sql_url: &str) -> Database<'_> {
        let pool = PgPool::new(sql_url).await.unwrap();
        let pool = Arc::new(pool);

        Database {
            todos: Arc::from(Table::new(pool.clone())),
        }
    }
}
