use actix_web::{web, App, HttpServer};
use todo_web_app::config::Config;
use todo_web_app::dao::Database;
use todo_web_app::{controller, AppState};
use std::sync::{Arc, Mutex};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load config from 
    let config = Config::new();

    // Connect to the database
    let db_context = Database::new(&config.get_database_url()).await;
    println!("Connected to database: {0}", config.get_database_url());

    // Instantiate the app_state. This application state will be cloned for each Actix thread but
    // the Arc of the DbContext will be reused in each Actix thread.
    let app_state = web::Data::new(AppState {
        connections: Mutex::new(0),
        context: Arc::new(db_context),
    });

    // Start the web application.
    // We'll need to transfer ownership of the AppState to the HttpServer via the `move`.
    // Then we can instantiate our controllers.
    let app = HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .configure(controller::init_index_controller)
            .configure(controller::init_todo_controller)
    })
    .bind(config.get_app_url())?;
    println!("Listening on: 127.0.0.1:8080");

    app.run().await
}
