use axum::{routing::post, Json, Router};
use serde::Deserialize;
use serde_json::Value;

use crate::{Error, Result};

// Route
pub fn routes() -> Router {
    Router::new().route("/api/login", post(api_login))
}

// Route controller
async fn api_login(payload: Json<LoginPayload>) -> Result<Json<Value>> {
    println!("->> {:<12} - api_login", "HANDLER");

    // TODO: Implement real db/auth logic
    if payload.username != "demo1" || payload.pwd != "welcome" {
        return Err(Error::LoginFail);
    }

    // TODO: Set cookies

    // Create the success body
    let body = Json(serde_json::json!({
        "result": {
            "success": true
        }
    }));

    Ok(body)
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    pwd: String,
}
