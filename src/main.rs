#[allow(unused)]
use axum::{
  extract::{Path, Query},
  http::{HeaderName, Method},
  middleware,
  response::{Html, IntoResponse, Response},
  routing::{get, get_service},
  Router,
};
use tower_cookies::{CookieManager, CookieManagerLayer};

pub use self::error::{Error, Result};
use serde::Deserialize;
use tokio::net::TcpListener;
use tower_http::{
  cors::{Any, CorsLayer},
  services::ServeDir,
};

mod error;
mod model;
mod web;

#[tokio::main]
async fn main() {
  let cors = CorsLayer::new()
    .allow_methods([Method::GET, Method::POST])
    .allow_headers(vec![HeaderName::from_static("content-type")])
    .allow_origin(Any);

  let routes_all = Router::new()
    .merge(web::routes_login::routes())
    .layer(cors)
    .layer(middleware::map_response(main_response_mapper))
    .layer(CookieManagerLayer::new())
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
