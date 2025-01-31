use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Mutex};

use crate::model::room::RoomData;

pub static GLOBAL_ROOM_ID_POOL: Lazy<Mutex<HashMap<i32, RoomData>>> =
    Lazy::new(|| Mutex::<HashMap<i32, RoomData>>::new(HashMap::new()));

#[derive(Serialize, Deserialize)]
pub enum StatusCode {
    OK = 0,
    Duplicate = 1,
    Full = 2,
    NotFound = 3,
    NotAvailable = 4,
    InternalError = 1000,
}

pub const GLOBAL_TOTAL_CARDS: i32 = 4 * 2 * 13 + 4;

pub fn get_semantic_card_from_card_id(id: i32) -> String {
    if id < 4 * 2 * 13 {
        let color_idx = id / 13;
        let color = if (0..=1).contains(&color_idx) {
            "red"
        } else if (2..=3).contains(&color_idx) {
            "black"
        } else if (4..=5).contains(&color_idx) {
            "blue"
        } else {
            "yellow"
        }.to_string();
        let number = (id % 13 + 1).to_string();
        color + &number
    } else if id == 4 * 2 * 13 {
        "concat".to_string()
    } else if id == 4 * 2 * 13 + 1 {
        "coloredConcat".to_string()
    } else if id == 4 * 2 * 13 + 2 {
        "double".to_string()
    } else {
        "mirror".to_string()
    }
}