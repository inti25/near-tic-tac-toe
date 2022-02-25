use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::ptr::addr_of_mut;
// use std::iter::Map;
use near_contract_standards::fungible_token::metadata::{
    FungibleTokenMetadata, FungibleTokenMetadataProvider, FT_METADATA_SPEC,
};
use near_contract_standards::fungible_token::FungibleToken;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::{ValidAccountId};
#[allow(unused_imports)]
use near_sdk::{env, near_bindgen, AccountId, Balance, PanicOnDefault, PromiseOrValue};
pub use crate::views::ContractMetadata;
near_sdk::setup_alloc!();
use near_sdk::{log};
use near_sdk::serde::de::Unexpected::Str;

mod views;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Game {
    pub status: i32,// 0: init , 1: done
    pub player1: String,
    pub player2: String,
    pub main: [i32; 9],
    pub winner: String,
    pub amount: u64,
    pub player_turn: i32
}
pub type GameId = String;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    pub owner_id: AccountId,
    pub games: HashMap<String, Game>,
    pub players: HashMap<AccountId, GameId>
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(owner_id: ValidAccountId) -> Self {
        Contract {
            owner_id: owner_id.into(),
            games: HashMap::new(),
            players: HashMap::new(),
        }
    }

    pub fn new_game(&mut self, second_player: AccountId) -> String {
        let current_player = env::predecessor_account_id();
        log!("cur_games = {}", self.games.len());
        let game_id = (self.games.len() + 1).to_string();
        let game = Game {
            status: 0,
            player1: current_player.clone(),
            player2: second_player.clone(),
            main: [0,0,0,0,0,0,0,0,0],
            winner: String::new(),
            amount: 0,
            player_turn: 1
        };
        self.games.insert(game_id.clone(), game);
        log!("new_games = {}", self.games.len());
        self.players.insert(current_player.clone(), game_id.clone());
        self.players.insert(second_player.clone(), game_id.clone());
        log!("players = {}", self.players.len());
        return game_id;
    }

    pub fn play(&mut self, game_id: String, position: usize) {
        let current_account = env::predecessor_account_id();
        let g = self.games.get(&game_id).unwrap();
        assert_eq!(0, g.status, "GAME_WAS_ENDED");
        if g.player_turn == 1 {
            assert_eq!(current_account, g.player1, "NOT_IS_YOUR_TURN");
        } else {
            assert_eq!(current_account, g.player2, "NOT_IS_YOUR_TURN");
        }

        let mut main_game: [i32; 9] = g.main.clone();
        log!("main_game old = {}", Contract::arr_to_string(main_game));
        main_game[position] = g.player_turn;
        log!("main_game new = {}", Contract::arr_to_string(main_game));
        let mut winner = g.winner.clone();
        let mut turn = g.player_turn;
        let status = Contract::check_winner(main_game.clone());
        if status == 2 { winner = String::new() }
        if status == 1 {
            if g.player_turn == 1 {
                winner = g.player1.clone();
            } else {
                winner = g.player2.clone();
            }
        }
        if status == 0 {
            turn = 1;
            if g.player_turn == 1 {
                turn = 2;
            }
        }

        let mut game = Game {
            status: status,
            player1: g.player1.clone(),
            player2: g.player2.clone(),
            main: main_game.clone(),
            winner,
            amount: g.amount.clone(),
            player_turn: turn
        };
        self.games.insert(game_id.clone(), game);
    }

    pub fn update_game_status(&mut self, game_id: String) {
        let g = self.games.get(&game_id).unwrap();
        let mut main_game: [i32; 9] = g.main.clone();
        let mut winner = g.winner.clone();
        let status = Contract::check_winner(main_game.clone());
        log!("main_game = {}", Contract::arr_to_string(main_game));
        log!("main_game status = {}", status);
        if status == 2 { winner = String::new() }
        if status == 1 {
            if g.player_turn == 1 {
                winner = g.player1.clone();
            } else {
                winner = g.player2.clone();
            }
        }

        let mut game = Game {
            status: status,
            player1: g.player1.clone(),
            player2: g.player2.clone(),
            main: main_game.clone(),
            winner,
            amount: g.amount.clone(),
            player_turn: g.player_turn
        };
        self.games.insert(game_id.clone(), game);
    }

    fn check_winner(arr: [i32; 9]) -> i32 {
        if
            (arr[0] == arr[1] && arr[1] == arr[2] && arr[2] != 0) ||
            (arr[3] == arr[4] && arr[4] == arr[5] && arr[5] != 0) ||
            (arr[6] == arr[7] && arr[7] == arr[8] && arr[8] != 0) ||
            (arr[0] == arr[3] && arr[3] == arr[6] && arr[6] != 0) ||
            (arr[1] == arr[4] && arr[4] == arr[7] && arr[7] != 0) ||
            (arr[2] == arr[5] && arr[5] == arr[8] && arr[8] != 0) ||
            (arr[0] == arr[4] && arr[4] == arr[8] && arr[8] != 0) ||
            (arr[2] == arr[4] && arr[4] == arr[6] && arr[6] != 0)
        {
            return 1
        }
        for num in arr {
            if num == 0 { return 0 }
        }
        return 2
    }

    fn arr_to_string(arr: [i32; 9]) -> String {
        let mut result:String = String::new();
        for num in arr {
            result.push_str(num.to_string().as_str());
            result.push_str(",");
        }
        return result;
    }


}

