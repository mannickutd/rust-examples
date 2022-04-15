use super::AppState;
use std::sync::Mutex;

pub mod index;
pub mod todo;

pub use index::init as init_index_controller;
pub use todo::init as init_todo_controller;

fn log_request(route: &'static str, connections: &Mutex<u32>) {
    let mut con = connections.lock().unwrap();
    *con += 1;
    println!("{}\n\tconnections: {}", route, con);
}

fn log_db_error(error: sqlx::Error) {
    println!("{}", error);
}