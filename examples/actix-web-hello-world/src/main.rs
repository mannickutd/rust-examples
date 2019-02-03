#![allow(unused_variables)]
#![cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]

extern crate actix;
extern crate actix_web;
extern crate bytes;
extern crate env_logger;
extern crate futures;

use bytes::Bytes;
use futures::sync::mpsc;
use futures::Stream;

use actix_web::http::{header, Method, StatusCode};
use actix_web::middleware::session::{self, RequestSession};
use actix_web::{
    error, fs, middleware, pred, server, App, Error, HttpRequest, HttpResponse, Path,
    Result,
};
use futures::future::{result, FutureResult};
use std::{env, io};


fn index(_req: &HttpRequest) -> &'static str {
    "Hello world!"
}

/// 404 handler
fn p404(req: &HttpRequest) -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open("static/404.html")?.set_status_code(StatusCode::NOT_FOUND))
}

/// async handler
fn index_async(req: &HttpRequest) -> FutureResult<HttpResponse, Error> {
    println!("{:?}", req);

    result(Ok(HttpResponse::Ok().content_type("text/html").body(
        format!("Hello {}!", req.match_info().get("name").unwrap()),
    )))
}


fn main() {
    env::set_var("RUST_LOG", "actix_web=debug");
    env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
    server::new(|| App::new()
        .resource("/", |r| r.f(index))
        // async handler
        .resource("/async/{name}", |r| r.method(Method::GET).a(index_async))
        .default_resource(|r| {
            // 404 for GET request
            r.method(Method::GET).f(p404);

            // all requests that are not `GET`
            r.route().filter(pred::Not(pred::Get())).f(
                |req| HttpResponse::MethodNotAllowed());
        })
    )
    .bind("127.0.0.1:8088")
    .unwrap()
    .run();
}
