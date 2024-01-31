#[allow(unused)]
use axum::{
  extract::{Path, Query},
  response::{Html, IntoResponse},
  routing::{get, get_service},
  Router,
};
use axum::{middleware, response::Response};

use serde::Deserialize;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

pub use self::error::{Error, Result};

mod error;
mod web;

#[tokio::main]
async fn main() {
  let routes_all = Router::new()
    .merge(web::routes_login::routes())
    .layer(middleware::map_response(main_response_mapper))
    .fallback_service(routes_static());

  let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
  axum::serve(listener, routes_all.into_make_service())
    .await
    .unwrap();
}

async fn main_response_mapper(res: Response) -> Response {
  println!("->> {:<12} - main_response_mapper", "RES_MAPPER");

  println!();
  res
}

fn routes_static() -> Router {
  Router::new().nest_service("/", get_service(ServeDir::new("./")))
}
