/* Super Tic-Tac-Toe Game in Rust */

/* This file contains the main entry point of the Super Tic-Tac-Toe game.
The game logic and user interaction are managed through this module.*/

/* Author: HB::3vil-M0rty
Created: 2023-08-28 */


/*                     Creative Commons Attribution-NonCommercial-ShareAlike 4.0 International License

This is a human-readable summary of (and not a substitute for) the license.

You are free to:
- Share — copy and redistribute the material in any medium or format
- Adapt — remix, transform, and build upon the material

Under the following terms:
- Attribution — You must give appropriate credit, provide a link to the license, and indicate if changes were made. You may do so in any reasonable manner, but not in any way that suggests the licensor endorses you or your use.
- NonCommercial — You may not use the material for commercial purposes.
- ShareAlike — If you remix, transform, or build upon the material, you must distribute your contributions under the same license as the original.
- No additional restrictions — You may not apply legal terms or technological measures that legally restrict others from doing anything the license permits. */

use std::{collections::VecDeque, io};

use entities::player::Player;
use game::{game::make_players, manager::GameManager};
use utils::instructions::{clear, rules};

mod entities {
    pub mod board;
    pub mod mini;
    pub mod player;
}

mod game {
    pub mod game;
    pub mod manager;
    pub mod moves;
}

mod utils {
    pub mod instructions;
}

fn main() {
    let size = 3;
    clear();
    rules();
    clear();
    let players: VecDeque<Player> = make_players();
    let mut manager: GameManager = GameManager::new(players, size);
    manager.play();

    loop {
        println!("Thanks for playing!");
        println!("Press 'n' to exit or 'y' to restart [y/n]:");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().to_lowercase().as_str() {
            "n" => {
                println!("Exiting the game. Goodbye!");
                break;
            }
            "y" => {
                main();
            }
            _ => {
                println!("Invalid input. Press 'n' to exit or 'y' to continue:");
            }
        }
    }
}
