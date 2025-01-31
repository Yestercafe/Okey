use axum::{http::StatusCode, Json};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct EchoReq {
    pub message: String,
}

#[derive(Serialize, Deserialize)]
pub struct EchoResp {
    pub message: String,
}

pub async fn echo(Json(payload): Json<EchoReq>) -> (StatusCode, Json<EchoResp>) {
    (
        StatusCode::ACCEPTED,
        Json(EchoResp {
            message: payload.message,
        }),
    )
}
