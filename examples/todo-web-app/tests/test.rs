use todo_web_app::config::Config;
use todo_web_app::dao::Database;


async fn init_db_context() -> Database<'static> {
    //let config = Config::from_file("test_resource/config.test.json");
    Database::new("postgresql://localhost:5432/test-todo-web-app").await
}


#[cfg(test)]
mod controller_test;

#[cfg(test)]
mod dao_test;
