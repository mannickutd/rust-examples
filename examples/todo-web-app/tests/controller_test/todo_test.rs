use super::init_app_state;
use actix_web::{http, test, App};
use todo_web_app::controller;
use todo_web_app::model::Todo;

#[actix_rt::test]
async fn get_todo_returns_err_when_not_found() -> () {
    let app_state = init_app_state().await;
    let mut app = test::init_service(
        App::new()
            .app_data(app_state.clone())
            .configure(controller::init_todo_controller),
    )
    .await;

    let req = test::TestRequest::get().uri("/todo/-5").to_request();

    let resp = test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), http::StatusCode::NOT_FOUND);
}

#[actix_rt::test]
async fn get_todo_returns_200_when_todo_exists() -> Result<(), sqlx::Error> {
    let app_state = init_app_state().await;
    let mut app = test::init_service(
        App::new()
            .app_data(app_state.clone())
            .configure(controller::init_todo_controller),
    )
    .await;

    let todo = Todo {
        id: None,
        name: "test todo".to_string(),
        description: "this is a todo".to_string(),
        date_added: None,
    };

    let todo_id = app_state.context.todos.add_todo(&todo).await?;

    let req = test::TestRequest::get()
        .uri(&format!("/todo/{0}", todo_id))
        .to_request();

    let resp = test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), http::StatusCode::OK);
    Ok(())
}

#[actix_rt::test]
async fn post_todo_returns_202_when_todo_is_valid() -> () {
    let app_state = init_app_state().await;
    let mut app = test::init_service(
        App::new()
            .app_data(app_state.clone())
            .configure(controller::init_todo_controller),
    )
    .await;

    let todo = Todo {
        id: None,
        name: "test post todo".to_string(),
        description: "This is a todo".to_string(),
        date_added: None
    };

    let req = test::TestRequest::post()
        .uri("/todo")
        .set_json(&todo)
        .to_request();

    let resp = test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), http::StatusCode::ACCEPTED)
}

#[actix_rt::test]
async fn patch_todo_returns_404_when_todo_does_not_exist() -> () {
    let app_state = init_app_state().await;
    let mut app = test::init_service(
        App::new()
            .app_data(app_state.clone())
            .configure(controller::init_todo_controller),
    )
    .await;

    let todo = Todo {
        id: None,
        name: "The Treachery of Todos".to_string(),
        description: "Ceci n'est pas une todo".to_string(),
        date_added: None,
    };

    let req = test::TestRequest::patch()
        .uri("/todo")
        .set_json(&todo)
        .to_request();

    let resp = test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), http::StatusCode::NOT_FOUND)
}

#[actix_rt::test]
async fn delete_todo_returns_404_when_todo_does_not_exist() -> () {
    let app_state = init_app_state().await;
    let mut app = test::init_service(
        App::new()
            .app_data(app_state.clone())
            .configure(controller::init_todo_controller),
    )
    .await;

    let req = test::TestRequest::delete()
        .uri(&format!("/todo/{0}", -1))
        .to_request();

    let resp = test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), http::StatusCode::NOT_FOUND)
}

#[actix_rt::test]
async fn delete_todo_returns_200_when_todo_exists() -> Result<(), sqlx::Error> {
    let app_state = init_app_state().await;
    let mut app = test::init_service(
        App::new()
            .app_data(app_state.clone())
            .configure(controller::init_todo_controller),
    )
    .await;

    let todo = Todo {
        id: None,
        name: "short lived".to_string(),
        description: "To live is to die".to_string(),
        date_added: None,
    };
    let todo_id = app_state.context.todos.add_todo(&todo).await?;

    let req = test::TestRequest::delete()
        .uri(&format!("/todo/{0}", todo_id))
        .to_request();

    let resp = test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), http::StatusCode::OK);
    Ok(())
}
