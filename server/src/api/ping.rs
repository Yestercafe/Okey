use axum::{http::StatusCode, Json};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PingReq {}

#[derive(Serialize, Deserialize)]
pub struct PingResp {
    pub message: String,
}

pub async fn ping() -> (StatusCode, Json<PingResp>) {
    (
        StatusCode::ACCEPTED,
        Json(PingResp {
            message: "pong".to_string(),
        }),
    )
}
