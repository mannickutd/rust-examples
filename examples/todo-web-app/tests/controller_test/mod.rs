use super::init_db_context;

use actix_web::web;
use actix_web::web::Data;
use todo_web_app::AppState;
use std::sync::{Arc, Mutex};

async fn init_app_state() -> Data<AppState<'static>> {
    let db_context = init_db_context().await;

    web::Data::new(AppState {
        connections: Mutex::new(0),
        context: Arc::new(db_context),
    })
}

#[cfg(test)]
mod index_test;
#[cfg(test)]
mod todo_test;
