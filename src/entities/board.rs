use std::fmt;


use crate::entities::mini::Miniboard;


#[derive(Debug)]
pub struct Board {
    pub size: i32,
    pub boards: Vec<Vec<Miniboard>>
}


impl Board {

    pub fn new(size: i32) -> Self {
        let boards = vec![vec![Miniboard::new(size); size as usize]; size as usize];
        Self {
            size,
            boards
        }
    }

    pub fn insert_symbol(&mut self, board_row: i32, board_col: i32, cell_row: i32, cell_col: i32, symbol: char) -> bool {
        if board_row >= self.size || board_col >= self.size {
            println!("Board row and column must be less than {}", self.size);
            return false;
        }

        self.boards[board_row as usize][board_col as usize].insert_symbol(cell_row, cell_col, symbol)
    }

    pub fn is_board_winner(&self, row: i32, col: i32, cell: &Miniboard) -> bool {
        self.is_row_complete(row, cell) || self.is_col_complete(col, cell) || self.is_diagonal_complete(cell)
    }

    pub fn is_row_complete(&self, row: i32, cell: &Miniboard) -> bool {
        for i in 0..self.size{
            if self.boards[row as usize][i as usize] != *cell {
                return false;
            }
        }
        true
    }

    pub fn is_col_complete(&self, col: i32, cell: &Miniboard) -> bool {
        for i in 0..self.size{
            if self.boards[i as usize][col as usize] != *cell {
                return false;
            }
        }
        true
    }

    pub fn is_diagonal_complete(&self, cell: &Miniboard) -> bool {
        let n = self.size;
        let mut diagonal_symbols = 0;
        for i in 0..n {
            if self.boards[i as usize][i as usize] != *cell{
                break;
            }
            diagonal_symbols +=1;
        }
        if diagonal_symbols == n {return true};
        diagonal_symbols = 0;
        for i in 0..n {
            if self.boards[i as usize][(n-i-1) as usize] != *cell{
                break;
            }
            diagonal_symbols +=1;
        }
        return diagonal_symbols == n
    }

}


impl fmt::Display for Board {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {

        println!("╔═══════╦═══════╦═══════╗ ▒");
        let mut count = 0;
        for row in &self.boards {
            for i in 0..=2 {
                println!("║{} ║{} ║{} ║ ▒", vec_to_string(&row[0].cells[i]), 
                vec_to_string(&row[1].cells[i]), 
                vec_to_string(&row[2].cells[i]));
            }
            if count < &self.boards.len() - 1 {
                print!("╠═══════╬═══════╬═══════╣ ▒");
            }
            else {
                print!("╚═══════╩═══════╩═══════╝ ▒");
            }
            count += 1; 
            fmt.write_str("\n")?;
        }
        print!("   ▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒");
        Ok(())
    }
}

pub fn vec_to_string(vector: &Vec<char>) -> String {
    let mut result: String = "".to_string();
    for data in vector {
        result = result + " " + &data.to_string(); 
    }

    result
}