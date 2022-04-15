use super::Table;
use super::Todo;
use sqlx::postgres::PgQueryAs;

impl<'c> Table<'c, Todo> {

    pub async fn get_todo_by_id(&self, todo_id: &str) -> Result<Todo, sqlx::Error> {
        sqlx::query_as(
            r#"
            SELECT id, name, description, date_added 
            FROM todos
            WHERE id = $1"#,
        )
        .bind(todo_id.parse::<i64>().unwrap())
        .fetch_one(&*self.pool)
        .await
    }

    pub async fn add_todo(&self, todo: &Todo) -> Result<u64, sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO todos (name, description)
            VALUES($1, $2)"#,
        )
        .bind(&todo.name)
        .bind(&todo.description)
        .execute(&*self.pool)
        .await
    }

    pub async fn delete_todo(&self, todo_id: &str) -> Result<u64, sqlx::Error> {
        sqlx::query(
            r#"
            DELETE FROM todos
            WHERE id = $1
            "#,
        )
        .bind(todo_id.parse::<i64>().unwrap())
        .execute(&*self.pool)
        .await
    }
}