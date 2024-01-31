#[allow(unused)]
use axum::{
  extract::{Path, Query},
  response::{Html, IntoResponse},
  routing::{get, get_service},
  Router,
};

use serde::Deserialize;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

pub use self::error::{Error, Result};

mod error;
mod web;

#[tokio::main]
async fn main() {
  let routes_all = Router::new()
    .merge(routes_hello())
    .merge(web::routes_login::routes())
    .fallback_service(routes_static());

  let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
  axum::serve(listener, routes_all.into_make_service())
    .await
    .unwrap();
}

fn routes_static() -> Router {
  Router::new().nest_service("/", get_service(ServeDir::new("./")))
}

// region: -- Routes Hello
fn routes_hello() -> Router {
  Router::new()
    .route("/hello", get(handler_hello))
    .route("/hello2/:name", get(handler_hello2))
}

#[derive(Debug, Deserialize)]
struct HelloParams {
  name: Option<String>,
}

async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
  println!("->> {:<12} - handler_hello", "HANDLER");

  let name = params.name.as_deref().unwrap_or("World");
  Html(format!("Hello <strong>{name}</strong>"))
}

async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {}
