use axum::{http::StatusCode, Json};
use serde::{Deserialize, Serialize};

use crate::{global, model::room::RoomData};

#[derive(Serialize, Deserialize)]
pub struct FetchRoomsResp {
    pub rooms: Vec<RoomData>,
}

pub async fn fetch_rooms() -> (StatusCode, Json<FetchRoomsResp>) {
    let rooms = global::GLOBAL_ROOM_ID_POOL
        .lock()
        .unwrap()
        .values()
        .cloned()
        .collect();
    (StatusCode::OK, Json(FetchRoomsResp { rooms }))
}
