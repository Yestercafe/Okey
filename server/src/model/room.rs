use rand::seq::IteratorRandom;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

use crate::global;

#[derive(Serialize, Deserialize, Clone)]
pub struct RoomData {
    id: i32,
    name: String,
    capacity: i32,
    players: HashSet<String>,
    is_started: bool,

    pub deck: HashSet<i32>,
    pub hands: HashMap<String, HashSet<i32>>,
    pub table: Vec<Vec<i32>>,
}

impl RoomData {
    pub fn new(id: i32, name: String) -> Self {
        RoomData {
            id,
            name,
            capacity: 4,
            players: HashSet::new(),
            is_started: false,
            deck: (1..=global::GLOBAL_TOTAL_CARDS).collect(),
            hands: HashMap::new(),
            table: Vec::new(),
        }
    }

    pub fn add_player(&mut self, player: String) -> bool {
        self.players.insert(player)
    }

    pub fn remove_player(&mut self, player: String) -> bool {
        self.players.remove(&player)
    }

    pub fn get_available(&self) -> i32 {
        self.capacity - self.players.len() as i32
    }

    pub fn can_start(&self) -> bool {
        self.players.len() as i32 == self.capacity
    }

    pub fn is_started(&self) -> bool {
        self.is_started
    }

    pub fn start(&mut self) {
        self.is_started = true;

        let mut rng = rand::rng();

        for player_name in &self.players {
            let selected_items: Vec<i32> = self.deck.iter().cloned().choose_multiple(&mut rng, 14);
            let selected_items: HashSet<i32> = selected_items
                .into_iter()
                .filter_map(|x| self.deck.take(&x))
                .collect();

            self.hands.insert(player_name.clone(), selected_items.into_iter().collect());
        }
    }
}
