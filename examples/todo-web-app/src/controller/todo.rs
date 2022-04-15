use super::{log_request, log_db_error};
use super::AppState;
use crate::model::Todo;
use actix_web::{delete, get, post, web, HttpResponse, Responder};


pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(get_todo);
    cfg.service(post_todo);
    cfg.service(delete_todo);
}

#[get("/todo/{id}")]
async fn get_todo(
    todo_id: web::Path<String>,
    app_state: web::Data<AppState<'_>>,
) -> impl Responder {
    log_request("GET: /todo", &app_state.connections);
    let todo = app_state.context.todos.get_todo_by_id(&todo_id).await;
    match todo {
        Err(err) => {
            log_db_error(err);
            HttpResponse::NotFound().finish()
        },
        Ok(todo) => HttpResponse::Ok().json(todo),
    }
}

#[post("/todo")]
async fn post_todo(
    todo: web::Json<Todo>,
    app_state: web::Data<AppState<'_>>,
) -> impl Responder {
    log_request("POST: /todo", &app_state.connections);
    let todo = todo.into_inner();
    let x = app_state.context.todos.add_todo(&todo).await;
    match x {
        Ok(_) => HttpResponse::Accepted().body(format!("{:?}", todo.id)),
        Err(err) => {
            log_db_error(err);
            return HttpResponse::InternalServerError().finish();
        }
    }
}

#[delete("/todo/{id}")]
async fn delete_todo(
    id: web::Path<String>,
    app_state: web::Data<AppState<'_>>,
) -> impl Responder {
    log_request("DELETE: /todo", &app_state.connections);
    let x = app_state.context.todos.delete_todo(&id).await;

    match x {
        Ok(0) => HttpResponse::NotFound().finish(),
        Ok(_) => HttpResponse::Ok().finish(),
        Err(err) => {
            log_db_error(err);
            HttpResponse::InternalServerError().finish()
        }
    }
}