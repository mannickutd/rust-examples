use actix_web::{web, App, HttpServer};
use todo_web_app::config::Config;
use todo_web_app::dao::Database;
use todo_web_app::{controller, AppState};
use std::sync::{Arc, Mutex};

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    // Read in the configuration file.
    // In small projects this can be a local configuration, but in more sophisticated systems, it is
    // best practice to keep the configuration file on a remote server where it can be retrieved
    // with an http request.
    //let config_file: &'static str = "config.json";
    //let config = Config::from_file(config_file);
    //println!("Using configuration file from {0}", config_file);

    // Connect to the database
    let db_context = Database::new("postgresql://localhost:5432/todo-web-app").await;
    //println!("Connected to database: {0}", config.get_database_url());

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
    .bind("127.0.0.1:8080")?;
    println!("Listening on: 127.0.0.1:8080");

    app.run().await
}
