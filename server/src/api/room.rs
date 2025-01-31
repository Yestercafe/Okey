use axum::{http::StatusCode, Json};
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};

use crate::{global, model::room::RoomData};

#[derive(Serialize, Deserialize)]
pub struct CreateRoomReq {
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreateRoomResp {
    pub room_id: i32,
    pub room_data: Option<RoomData>,
}

pub async fn create_room(Json(payload): Json<CreateRoomReq>) -> (StatusCode, Json<CreateRoomResp>) {
    let mut rng = rand::rng();
    let mut rand_list: Vec<i32> = (1..=10000).collect();
    rand_list.shuffle(&mut rng);
    for room_id in rand_list {
        if global::GLOBAL_ROOM_ID_POOL
            .lock()
            .unwrap()
            .contains_key(&room_id)
        {
            continue;
        }
        let new_room = RoomData::new(room_id, payload.name.clone());
        global::GLOBAL_ROOM_ID_POOL
            .lock()
            .unwrap()
            .insert(room_id, new_room.clone());
        return (
            StatusCode::OK,
            Json(CreateRoomResp {
                room_id,
                room_data: Some(new_room),
            }),
        );
    }
    (
        StatusCode::OK,
        Json(CreateRoomResp {
            room_id: -1,
            room_data: None,
        }),
    )
}

#[derive(Serialize, Deserialize)]
pub struct EnterRoomReq {
    pub room_id: i32,
    pub player_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct EnterRoomResp {
    pub status_code: global::StatusCode,
    pub error_info: Option<String>,
    pub room_id: i32,
    pub room_data: Option<RoomData>,
}

pub async fn enter_room(Json(payload): Json<EnterRoomReq>) -> (StatusCode, Json<EnterRoomResp>) {
    let room_id = payload.room_id;
    let mut pool = global::GLOBAL_ROOM_ID_POOL.lock().unwrap();
    let response = if let Some(room) = pool.get_mut(&room_id) {
        if room.get_available() > 0 {
            if room.is_started() {
                return (
                    StatusCode::OK,
                    Json(EnterRoomResp {
                        status_code: global::StatusCode::NotAvailable,
                        error_info: Some("Game is started".to_string()),
                        room_id: payload.room_id,
                        room_data: Some(room.clone()),
                    }),
                );
            }
            if room.add_player(payload.player_name.clone()) {
                EnterRoomResp {
                    status_code: global::StatusCode::OK,
                    error_info: None,
                    room_id: payload.room_id,
                    room_data: Some(room.clone()),
                }
            } else {
                EnterRoomResp {
                    status_code: global::StatusCode::Duplicate,
                    error_info: Some("Duplicate player name".to_string()),
                    room_id: payload.room_id,
                    room_data: Some(room.clone()),
                }
            }
        } else {
            EnterRoomResp {
                status_code: global::StatusCode::Full,
                error_info: Some("Room is full".to_string()),
                room_id: payload.room_id,
                room_data: Some(room.clone()),
            }
        }
    } else {
        EnterRoomResp {
            status_code: global::StatusCode::NotFound,
            error_info: Some("Room not found".to_string()),
            room_id: -1,
            room_data: None,
        }
    };
    (StatusCode::OK, Json(response))
}

#[derive(Serialize, Deserialize)]
pub struct StartRoomReq {
    pub room_id: i32,
}

#[derive(Serialize, Deserialize)]
pub struct StartRoomResp {
    pub status_code: global::StatusCode,
}

pub async fn start_room(Json(payload): Json<StartRoomReq>) -> (StatusCode, Json<StartRoomResp>) {
    let room_id = payload.room_id;
    let mut pool = global::GLOBAL_ROOM_ID_POOL.lock().unwrap();
    if let Some(room) = pool.get_mut(&room_id) {
        if room.can_start() {
            room.start();
            (
                StatusCode::OK,
                Json(StartRoomResp {
                    status_code: global::StatusCode::OK,
                }),
            )
        } else {
            (
                StatusCode::OK,
                Json(StartRoomResp {
                    status_code: global::StatusCode::NotAvailable,
                }),
            )
        }
    } else {
        (
            StatusCode::OK,
            Json(StartRoomResp {
                status_code: global::StatusCode::NotFound,
            }),
        )
    }
}

#[derive(Serialize, Deserialize)]
pub struct QueryRoomReq {
    pub room_id: i32,
}

#[derive(Serialize, Deserialize)]
pub struct QueryRoomResp {
    pub status_code: global::StatusCode,
    pub room_id: i32,
    pub room_data: Option<RoomData>,
}

pub async fn query_room(Json(payload): Json<QueryRoomReq>) -> (StatusCode, Json<QueryRoomResp>) {
    let room_id = payload.room_id;
    let pool = global::GLOBAL_ROOM_ID_POOL.lock().unwrap();
    if let Some(room) = pool.get(&room_id) {
        (
            StatusCode::OK,
            Json(QueryRoomResp {
                status_code: global::StatusCode::OK,
                room_id,
                room_data: Some(room.clone()),
            }),
        )
    } else {
        (
            StatusCode::OK,
            Json(QueryRoomResp {
                status_code: global::StatusCode::NotFound,
                room_id: -1,
                room_data: None,
            }),
        )
    }
}
