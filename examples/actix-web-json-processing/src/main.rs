use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde_json::{Result, Value};
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
struct User {
    username: String,
}


#[post("/json/parse")]
async fn parse_json(info: web::Json<User>) -> Result<impl Responder> {
    return Ok(web::Json(info));
}


#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(parse_json)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
