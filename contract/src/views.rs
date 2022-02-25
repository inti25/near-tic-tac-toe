//! View functions for the contract.
use crate::*;
use near_sdk::serde::{Serialize};

#[derive(Serialize)]
#[serde(crate = "near_sdk::serde")]
#[cfg_attr(not(target_arch = "wasm32"), derive(Deserialize, Debug))]
pub struct ContractMetadata {
    pub version: String,
    pub owner_id: AccountId,
    pub games_count: i32,
    pub player_count: i32
}
#[derive(Serialize)]
#[serde(crate = "near_sdk::serde")]
#[cfg_attr(not(target_arch = "wasm32"), derive(Deserialize, Debug))]
pub struct GameDetail {
    player_turn: i32,
    game_status: i32,
    game_id: String,
    game: [i32; 9],
    player1: String,
    player2: String,
    winner: String
}

#[near_bindgen]
impl Contract {
    /// Return contract basic info
    pub fn contract_metadata(&self) -> ContractMetadata {
        // only update version number here
        ContractMetadata {
            version: env!("CARGO_PKG_VERSION").to_string(),
            owner_id: self.owner_id.clone(),
            games_count: self.games.len() as i32,
            player_count: self.players.len() as i32
        }
    }

    pub fn get_current_game(&self, player_id: AccountId) -> String {
        let gameid = self.players.get(&player_id).clone().map(|a| a.into()).unwrap_or_else(|| String::new());
        gameid.to_string()
    }

    pub fn get_game_detail(&self, game_id: String) -> GameDetail {
        let g = self.games.get(&game_id);
        let game = g.unwrap();
        GameDetail {
            player_turn: game.player_turn,
            game_status: game.status,
            game_id: game_id.clone(),
            game: game.main.clone(),
            player1: game.player1.clone(),
            player2: game.player2.clone(),
            winner: game.winner.clone()
        }
    }

    // fn hash_to_string(hash: HashMap<String, i32>) -> String {
    //     let mut result = String::new();
    //     for (key, val) in &hash {
    //         println!("key: {} val: {}", key, val);
    //         result.push_str("{");
    //         result.push_str(key);
    //         result.push_str(":");
    //         result.push_str(val.to_string().as_str());
    //         result.push_str("},");
    //     }
    //     result
    // }

    // pub fn get_virtual_price(&self) -> U128 {
    //     if self.ft.total_supply == 0 {
    //         100_000_000.into()
    //     } else {
    //         ((self.locked_token_amount
    //             + self.try_distribute_reward(nano_to_sec(env::block_timestamp())))
    //             * 100_000_000
    //             / self.ft.total_supply)
    //             .into()
    //     }
    // }
}
