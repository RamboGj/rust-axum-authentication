#![allow(unused)]

use anyhow::Result;
use serde_json::json;

#[tokio::test]
async fn quick_dev() -> Result<()> {
  // HTTP Client
  let hc = httpc_test::new_client("http://localhost:8080")?;

  let req_login = hc.do_post(
    "/api/login",
    json!({
      "username" : "demo1",
      "pwd": "welcomeDDD"
    }),
  );

  req_login.await?.print().await?;

  Ok(())
}
