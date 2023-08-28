use std::io::{self};

use crate::entities::{board::Board, mini::Miniboard, player::Player};

pub fn clear() {
    clearscreen::clear().expect("failed to clear screen");
}

pub fn pause() {
    println!("Press Enter to continue...");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
}

pub fn rules() {
    println!(r"                     _____ __ __  ____     ___  ____                     ");
    println!(r"                    / ___/|  |  ||    \   /  _]|    \                    ");
    println!(r"                   (   \_ |  |  ||  o  ) /  [_ |  D  )                   ");
    println!(r"                    \__  ||  |  ||   _/ |    _]|    /                    ");
    println!(r"                    /  \ ||  :  ||  |   |   [_ |    \                    ");
    println!(r"                    \    ||     ||  |   |     ||  .  \                   ");
    println!(r"                     \___| \__,_||__|   |_____||__|\_|                   ");
    println!("\n\n");
    println!(r" ______  ____     __      ______   ____     __      ______   ___     ___ ");
    println!(r"|      ||    |   /  ]    |      | /    |   /  ]    |      | /   \   /  _]");
    println!(r"|      | |  |   /  /     |      ||  o  |  /  /     |      ||     | /  [_ ");
    println!(r"|_|  |_| |  |  /  /      |_|  |_||     | /  /      |_|  |_||  O  ||    _]");
    println!(r"  |  |   |  | /   \_       |  |  |  _  |/   \_       |  |  |     ||   [_ ");
    println!(r"  |__|  |____| \____|      |__|  |__|__| \____|      |__|   \___/ |_____|");

    println!("\n");
    println!("Super Tic Tac Toe Rules:");
    println!("1. The game is played on a 3x3 grid of 3x3 Tic Tac Toe Miniboards.");
    println!("2. Players take turns placing their symbol (X or O) in an empty cell.");
    println!("3. The cell chosen determines the Miniboard where the next player must move.");
    println!(
        "4. To win a Miniboard, get 3 symbols in a row (horizontally, vertically, or diagonally)."
    );
    println!("5. If the next Miniboard is already won or full(drawn), you can play in any other valid miniboard.");
    println!(
        "6. The game ends when a player wins 3 Miniboards in a row or the entire board is filled."
    );
    println!("7. The player achieving the win condition first is declared the winner.");
    println!("\n");
    pause();
    clear();
    let cells: Vec<Miniboard> = vec![
        Miniboard {
            size: (3),
            cells: vec![
                vec!['╔', '═', '╗'],
                vec!['║', '1', '║'],
                vec!['╚', '═', '╝'],
            ],
        },
        Miniboard {
            size: (3),
            cells: vec![
                vec!['╔', '═', '╗'],
                vec!['║', '2', '║'],
                vec!['╚', '═', '╝'],
            ],
        },
        Miniboard {
            size: (3),
            cells: vec![
                vec!['╔', '═', '╗'],
                vec!['║', '3', '║'],
                vec!['╚', '═', '╝'],
            ],
        },
        Miniboard {
            size: (3),
            cells: vec![
                vec!['╔', '═', '╗'],
                vec!['║', '4', '║'],
                vec!['╚', '═', '╝'],
            ],
        },
        Miniboard {
            size: (3),
            cells: vec![
                vec!['╔', '═', '╗'],
                vec!['║', '5', '║'],
                vec!['╚', '═', '╝'],
            ],
        },
        Miniboard {
            size: (3),
            cells: vec![
                vec!['╔', '═', '╗'],
                vec!['║', '6', '║'],
                vec!['╚', '═', '╝'],
            ],
        },
        Miniboard {
            size: (3),
            cells: vec![
                vec!['╔', '═', '╗'],
                vec!['║', '7', '║'],
                vec!['╚', '═', '╝'],
            ],
        },
        Miniboard {
            size: (3),
            cells: vec![
                vec!['╔', '═', '╗'],
                vec!['║', '8', '║'],
                vec!['╚', '═', '╝'],
            ],
        },
        Miniboard {
            size: (3),
            cells: vec![
                vec!['╔', '═', '╗'],
                vec!['║', '9', '║'],
                vec!['╚', '═', '╝'],
            ],
        },
    ];

    let top_border = "╔═══════════════════════╗ ";
    let side_border = "║";
    let bottom_border = "╚═══════════════════════╝ ▒";

    println!("{}", top_border);
    println!("{}      Welcome to       {} ▒", side_border, side_border);
    println!("{}  Super tic tac toe    {} ▒", side_border, side_border);
    println!("{}", bottom_border);

    println!("{}", top_border);
    println!("{}       the cells       {} ▒", side_border, side_border);
    println!("{}       of board        {} ▒", side_border, side_border);
    println!("{}", bottom_border);

    let mut board = Board::new(3);

    let mut cell_number: usize = 0;
    for i in 0..=2 {
        for j in 0..=2 {
            if let Some(cell) = cells.get(cell_number) {
                board.boards[i][j] = cell.clone();
            }
            cell_number += 1;
        }
    }
    println!("{}", board);
    pause();
    clear();

    println!("{}", top_border);
    println!("{}       the cells       {} ▒", side_border, side_border);
    println!("{}      of MiniBoard     {} ▒", side_border, side_border);
    println!("{}", bottom_border);

    let mini_cell: Miniboard = Miniboard {
        size: (3),
        cells: vec![
            vec!['1', '2', '3'],
            vec!['4', '5', '6'],
            vec!['7', '8', '9'],
        ],
    };

    for i in 0..=2 {
        for j in 0..=2 {
            board.boards[i][j] = mini_cell.clone();

            cell_number += 1;
        }
    }
    println!("{}", board);
    pause();
    clear();

    println!("{}", top_border);
    println!("{}   to choose a cell    {} ▒", side_border, side_border);
    println!("{} just enter its number {} ▒", side_border, side_border);
    println!("{}", bottom_border);
    println!("   ▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒");
    pause();
    clear();
}

pub fn welcome(player: &Player) {
    let top_border = "╔═══════════════════════╗ ";
    let side_border = "║";
    let bottom_border = "╚═════════════╩═════════╝ ▒";

    println!("{}", top_border);
    println!("{}      Welcome to       {} ▒", side_border, side_border);
    println!("{}  Super tic tac toe    {} ▒", side_border, side_border);
    println!("╠═════════════╦═════════╣ ▒");
    println!(
        "{} {:11.11} ║    {}    {} ▒",
        side_border, player.name, player.symbol, side_border
    );

    println!("{}", bottom_border);
}

pub fn turn(player: &Player) {
    let top_border = "╔═════════════╦═════════╗ ";
    let side_border = "║";
    let bottom_border = "╚═════════════╩═════════╝ ▒";
    println!("{}", top_border);
    println!(
        "{} {:11.11} ║    {}    {} ▒",
        side_border, player.name, player.symbol, side_border
    );
    println!("{}", bottom_border);
}


