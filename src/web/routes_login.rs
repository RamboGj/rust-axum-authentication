use crate::{Error, Result};
use axum::{routing::post, Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};

#[derive(Debug, Deserialize)]
struct LoginPayload {
  username: String,
  pwd: String,
}

async fn api_login(payload: Json<LoginPayload>) -> Result<Json<Value>> {
  // TO-DO: Implement real db/auth logic
  if payload.username != "demo1" || payload.pwd != "welcome" {
    return Err(Error::LoginFail);
  }

  // TO-DO: Set cookies

  // Create the success body
  let body = Json(json!({
    "result": {
      "success": true
    }
  }));

  Ok(body)
}

pub fn routes() -> Router {
  Router::new().route("/api/login", post(api_login))
}
