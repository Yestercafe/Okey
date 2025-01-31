use axum::{http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use rand::seq::IteratorRandom;

use crate::{global, model::room::RoomData};

#[derive(Serialize, Deserialize)]
pub struct FetchCardReq {
    pub room_id: i32,
    pub player_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct FetchCardResp {
    pub status_code: global::StatusCode,
    pub error_info: Option<String>,
    pub room_data: Option<RoomData>,
}

pub async fn fetch_card(Json(payload): Json<FetchCardReq>) -> (StatusCode, Json<FetchCardResp>) {
    let mut rng = rand::rng();
    let mut room_pool = global::GLOBAL_ROOM_ID_POOL.lock().unwrap();
    let room = room_pool.get_mut(&payload.room_id);
    if room.is_none() {
        return (
            StatusCode::OK,
            Json(FetchCardResp {
                status_code: global::StatusCode::NotFound,
                error_info: Some("Room not found".to_string()),
                room_data: None,
            }),
        );
    }
    let room = room.unwrap();
    if let Some(random_card) = room.deck.iter().choose(&mut rng).copied() {
        room.deck.remove(&random_card);
        if let Some(player_hands) = room.hands.get_mut(&payload.player_name) {
            player_hands.insert(random_card);
            (
                StatusCode::OK,
                Json(FetchCardResp {
                    status_code: global::StatusCode::OK,
                    error_info: None,
                    room_data: Some(room.clone()),
                }),
            )
        } else {
            (
                StatusCode::OK,
                Json(FetchCardResp {
                    status_code: global::StatusCode::NotFound,
                    error_info: Some("Player not found".to_string()),
                    room_data: None,
                }),
            )
        }
    } else {
        (
            StatusCode::OK,
            Json(FetchCardResp {
                status_code: global::StatusCode::Empty,
                error_info: Some("No card left".to_string()),
                room_data: None,
            }),
        )
    }
}

