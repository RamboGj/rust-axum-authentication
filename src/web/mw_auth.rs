use crate::ctx::Ctx;
use crate::web::AUTH_TOKEN;
use crate::{Error, Result};
use async_trait::async_trait;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::RequestPartsExt;
use axum::{body::Body, extract::Request, middleware::Next, response::Response};
use lazy_regex::regex_captures;
use tower_cookies::Cookies;

pub async fn mw_require_auth(cookies: Cookies, req: Request<Body>, next: Next) -> Result<Response> {
  let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.to_string());

  let (user_id, exp, sign) = auth_token
    .ok_or(Error::AuthFailNoAuthTokenCookie)
    .and_then(parse_token)?;

  Ok(next.run(req).await)
}

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Ctx {
  type Rejection = Error;

  async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
    println!("->> {:<12} - Ctx", "EXTRACTOR");

    parts
      .extensions
      .get::<Result<Ctx>>()
      .ok_or(Error::AuthFailCtxNotInRequestExt)?
      .clone()
  }
}

// Parse token of format `user-[user-id]-[expiration].[signature]`
// Returns (user_id, expiration, signature)
fn parse_token(token: String) -> Result<(u64, String, String)> {
  let (_whole, user_id, exp, sign) = regex_captures!(r#"^user-(\d+)\.(.+)\.(.+)"#, &token)
    .ok_or(Error::AuthFailTokenWrongFormat)
    .unwrap();

  let user_id: u64 = user_id
    .parse()
    .map_err(|_| Error::AuthFailTokenWrongFormat)?;

  Ok((user_id, exp.to_string(), sign.to_string()))
}
