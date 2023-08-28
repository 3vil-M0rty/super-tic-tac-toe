use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Miniboard {
    pub size: i32,
    pub cells: Vec<Vec<char>>
}

impl Miniboard {
    pub fn new(size: i32) -> Self {
        let cells = vec![vec!['-'; size as usize]; size as usize];
        Self {
            size,
            cells
        }
    }
    
    pub fn insert_symbol(&mut self, row: i32, col: i32, symbol: char) -> bool {
        if row >= self.size || col >= self.size{
            println!("row and column must be less than {}", self.size);
            return false;
        } 

        if self.cells[row as usize][col as usize] != '-' {
            println!("cell is already marked");
            return false;
        }

        self.cells[row as usize][col as usize] = symbol;
        return true;
    }

    pub fn is_miniboard_full(&self) -> bool {
        for row in &self.cells{
            for cell in row{
                if *cell == '-' {
                    return false
                }
            }
        }
        true
    }

    pub fn is_miniboard_winner(&self, row: i32, col: i32, symbol: char) -> bool {
        self.is_mini_row_complete(row, symbol) || self.is_mini_col_complete(col, symbol) || self.is_mini_diagonal_complete(symbol)
    }

    pub fn is_mini_row_complete(&self, row: i32, symbol: char) -> bool {
        for i in 0..self.size{
            if self.cells[row as usize][i as usize] != symbol {
                return false
            }
        }
        true
    }

    pub fn is_mini_col_complete(&self, col: i32, symbol: char) -> bool {
        for i in 0..self.size{
            if self.cells[i as usize][col as usize] != symbol {
                return false
            }
        }
        true
    }

    pub fn is_mini_diagonal_complete(&self, symbol: char) -> bool {
        let n = self.size;
        let mut diagonal_symbols = 0;
        for i in 0..n {
            if self.cells[i as usize][i as usize] != symbol{
                break;
            }
            diagonal_symbols +=1;
        }
        if diagonal_symbols == n {return true};
        diagonal_symbols = 0;
        for i in 0..n {
            if self.cells[i as usize][(n-i-1) as usize] != symbol{
                break;
            }
            diagonal_symbols +=1;
        }
        return diagonal_symbols == n
    }

}

impl fmt::Display for Miniboard {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        for row in &self.cells {
            for cell in row {
                fmt.write_str(&cell.to_string())?;
                fmt.write_str(" ")?;
            }
            let _ = fmt.write_str("\n");
        }
        Ok(())
    }
}

