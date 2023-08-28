use std::io;

use crate::entities::{board::Board, mini::Miniboard};

pub fn board_move() -> u32 {
    loop {
        println!("Please enter Board's cell to Play [1-9]:");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        
        match input.trim().parse::<u32>() {
            Ok(number) if (1..=9).contains(&number) => {
                return number;
            }
            _ => {
                println!("Invalid input. Please enter a digit from 1 to 9.");
            }
        }
    }
}


pub fn miniboard_move() -> u32 {
    loop {
        println!("Please enter Miniboard's cell to Play [1-9]:");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        
        match input.trim().parse::<u32>() {
            Ok(number) if (1..=9).contains(&number) => {
                return number;
            }
            _ => {
                println!("Invalid input. Please enter a digit from 1 to 9.");
            }
        }
    }
}

pub fn update_board(
    board: &mut Board,
    board_cell: u32,
    miniboard_cell: u32,
    symbol: char,
    size: i32,
) {
    let i_board = ((board_cell - 1) / size as u32) as i32;
    let j_board = ((board_cell - 1) % size as u32) as i32;
    let i_mini = ((miniboard_cell - 1) / size as u32) as i32;
    let j_mini = ((miniboard_cell - 1) % size as u32) as i32;

    board.insert_symbol(i_board, j_board, i_mini, j_mini, symbol);
}

pub fn cell_symbol(symbol: char) -> Miniboard {
    let xcell: Miniboard = Miniboard {
        size: (3),
        cells: vec![
            vec!['╔', '═', '╗'],
            vec!['║', 'X', '║'],
            vec!['╚', '═', '╝'],
        ],
    };

    let ocell: Miniboard = Miniboard {
        size: (3),
        cells: (vec![
            vec!['╔', '═', '╗'],
            vec!['║', 'O', '║'],
            vec!['╚', '═', '╝'],
        ]),
    };
    if symbol == 'X' {
        return xcell;
    }
    ocell
}
