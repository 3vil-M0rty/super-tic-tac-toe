use rand::Rng;

use crate::entities::{board::Board, player::Player};
use std::{collections::VecDeque, io};

pub struct Game {
    pub status: String,
    pub players: VecDeque<Player>,
    pub board: Board,
}

impl Game {
    pub fn new(players: VecDeque<Player>, board: Board) -> Self {
        Self {
            players,
            board,
            status: "NOT_STARTED".to_string()
        }
    }
}

pub fn make_players() -> VecDeque<Player> {
    println!("Please enter the first player's name: ");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let name_one = input.trim();

    println!("Please enter the second player's name: ");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let name_two = input.trim();

    let names: (String, String) = (name_one.to_string(), name_two.to_string());
    let mut rng = rand::thread_rng();
    let random_number: u8 = rng.gen_range(0..=1);
    let mut player_x = Player::new(&names.0, 'X');
    let mut player_o = Player::new(&names.1, 'O');
    if random_number == 1 {
        player_x = Player::new(&names.1, 'X');
        player_o = Player::new(&names.0, 'O');
    }
    let mut players: VecDeque<Player> = VecDeque::new();
    players.push_back(player_x);
    players.push_back(player_o);
    players
}
