use std::collections::VecDeque;

use crate::{
    entities::board::Board,
    game::moves::{board_move, miniboard_move, update_board, cell_symbol},
    game::game::Game,
    utils::instructions::{clear, turn, welcome},
    entities::mini::Miniboard,
    entities::player::Player,
};


pub struct GameManager {
    pub game: Game,
    pub moves: i32,
}

impl GameManager {
    pub fn new(players: VecDeque<Player>, size: i32) -> Self {
        let board = Board::new(size);
        let game = Game::new(players, board);

        Self { game, moves: 0 }
    }
    
    pub fn change_player(&mut self) {
        let current = self.game.players.pop_front();
        match current {
            Some(current_player) => self.game.players.push_back(current_player),
            _ => {}
        }
    }

    pub fn play(&mut self) {
        let mut full: Vec<u32> = Vec::new();
        let mut won: Vec<u32> = Vec::new();
        let mut board_check = false;
        let mut draw_check = false;
        let size = self.game.board.boards.len() as i32;
        let mut board_move_input;
        let mut mini_move_input ;

        clear();
        welcome(&self.game.players[0]);
        println!("{}", self.game.board);

        board_move_input = board_move();
        mini_move_input = miniboard_move();
        
        update_board(
            &mut self.game.board,
            board_move_input,
            mini_move_input,
            self.game.players[0].symbol,
            size,
        );
        self.moves += 1;
        self.change_player();

        clear();
        turn(&self.game.players[0]);
        println!("{}", self.game.board);

        while self.game.status != "COMPLETE".to_string() {
            
            board_move_input = mini_move_input;
            while full.contains(&board_move_input) || won.contains(&board_move_input) {
                println!(
                    "the miniboard {} is full/won choose another one:",
                    board_move_input
                );
                board_move_input = board_move();
            }

            let i = ((board_move_input - 1) / size as u32) as usize;
            let j = ((board_move_input - 1) % size as u32) as usize;

            println!("you're playing in miniboard {}", board_move_input);
            mini_move_input = miniboard_move();

            let mut k = ((mini_move_input - 1) / size as u32) as usize;
            let mut m = ((mini_move_input - 1) % size as u32) as usize;
            while !self.game.board.boards[i][j].insert_symbol(
                k as i32,
                m as i32,
                self.game.players[0].symbol,
            ) {
                println!(
                    "this cell {} is marked, choose another one.",
                    mini_move_input
                );
                mini_move_input = miniboard_move();
                k = ((mini_move_input - 1) / size as u32) as usize;
                m = ((mini_move_input - 1) % size as u32) as usize;
            }

            update_board(
                &mut self.game.board,
                board_move_input,
                mini_move_input,
                self.game.players[0].symbol,
                size,
            );

            if self.game.board.boards[i][j].is_miniboard_winner(
                k as i32,
                m as i32,
                self.game.players[0].symbol,
            ) {
                won.push(board_move_input);
                println!("{:?} {:?}", won, full)
            }

            if self.game.board.boards[i][j].is_miniboard_full() {
                full.push(board_move_input);
            }

            if won.contains(&board_move_input) {
                self.game.board.boards[i][j] = cell_symbol(self.game.players[0].symbol)
            } else {
                if full.contains(&board_move_input) {
                    draw_check = true;
                    self.game.board.boards[i][j] = Miniboard {
                        size: (3),
                        cells: vec![
                            vec!['╔', '╦', '╗'],
                            vec!['╠', '╬', '╣'],
                            vec!['╚', '╩', '╝'],
                        ],
                    };
                }
            }

            if self.game.board.is_board_winner(
                i as i32,
                j as i32,
                &cell_symbol(self.game.players[0].symbol),
            ) {
                board_check = true;

                self.game.status = "COMPLETE".to_string();
            }
            clear();
            if draw_check {
                println!("you've drawn the miniboard {}", board_move_input);
            }
            if board_check {
                println!(
                    "congratulations {} with symbol {} you won the game!",
                    self.game.players[0].name, self.game.players[0].symbol
                );
            }

            self.moves += 1;
            self.change_player();
            if self.moves == 81 {
                println!("The game is drawn!");
                break;
            }
            
            board_check = false;
            draw_check = false;

            turn(&self.game.players[0]);
            println!("{}", self.game.board);
        }
    }
}
