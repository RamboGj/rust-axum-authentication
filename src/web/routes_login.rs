use crate::{web::AUTH_TOKEN, Error, Result};
use axum::{routing::post, Json, Router};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tower_cookies::{Cookie, Cookies};

#[derive(Debug, Deserialize, Serialize)]
struct LoginPayload {
  username: String,
  pwd: String,
}

async fn api_login(cookies: Cookies, payload: Json<LoginPayload>) -> Result<Json<Value>> {
  // TO-DO: Implement real db/auth logic
  if payload.username != "demo1" || payload.pwd != "welcome" {
    return Err(Error::LoginFail);
  }

  cookies.add(Cookie::new(AUTH_TOKEN, "user-1-exp.sign"));

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
