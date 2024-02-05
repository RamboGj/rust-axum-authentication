#[allow(unused)]
use axum::{
  extract::{Path, Query},
  http::{HeaderName, Method},
  middleware,
  response::{Html, IntoResponse, Response},
  routing::{get, get_service},
  Router,
};
use model::ModelController;
use tower_cookies::{CookieManager, CookieManagerLayer};
use web::mw_auth;

pub use self::error::{Error, Result};
use tokio::net::TcpListener;
use tower_http::{
  cors::{Any, CorsLayer},
  services::ServeDir,
};

mod ctx;
mod error;
mod model;
mod web;

#[tokio::main]
async fn main() -> Result<()> {
  // Initialize Model controller
  let mc = ModelController::new().await?;

  let cors = CorsLayer::new()
    .allow_methods([Method::GET, Method::POST])
    .allow_headers(vec![HeaderName::from_static("content-type")])
    .allow_origin(Any);

  let routes_apis = web::routes_tickets::routes(mc.clone())
    .route_layer(middleware::from_fn(web::mw_auth::mw_require_auth));

  let routes_all = Router::new()
    .merge(web::routes_login::routes())
    .nest("/api", routes_apis)
    .layer(cors)
    .layer(middleware::map_response(main_response_mapper))
    .layer(CookieManagerLayer::new())
    .fallback_service(routes_static());

  let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
  axum::serve(listener, routes_all.into_make_service())
    .await
    .unwrap();

  Ok(())
}

async fn main_response_mapper(res: Response) -> Response {
  println!("->> {:<12} - main_response_mapper", "RES_MAPPER");

  println!();
  res
}

fn routes_static() -> Router {
  Router::new().nest_service("/", get_service(ServeDir::new("./")))
}
