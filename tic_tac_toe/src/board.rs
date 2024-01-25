pub struct Board {
    grid: [[Option<char>; 3]; 3],
}

impl Board {
    pub fn new() -> Self {
        Board {
            grid: [[None; 3]; 3],
        }
    }

    pub fn display(&self) {
        for row in &self.grid {
            for &cell in row {
                match cell {
                    Some(marker) => print!(" {} ", marker),
                    None => print!("   "),
                }
            }
            println!();
        }
    }

    pub fn make_move(&mut self, row: usize, col: usize, marker: char) -> bool {
        if row < 3 && col < 3 && self.grid[row][col].is_none() {
            self.grid[row][col] = Some(marker);
            true
        } else {
            false
        }
    }

    pub fn check_winner(&self) -> Option<char> {
        // Check rows, columns, and diagonals for a winner
        for i in 0..3 {
            if self.grid[i][0] == self.grid[i][1] && self.grid[i][1] == self.grid[i][2] {
                return self.grid[i][0];
            }
            if self.grid[0][i] == self.grid[1][i] && self.grid[1][i] == self.grid[2][i] {
                return self.grid[0][i];
            }
        }

        if self.grid[0][0] == self.grid[1][1] && self.grid[1][1] == self.grid[2][2] {
            return self.grid[0][0];
        }

        if self.grid[0][2] == self.grid[1][1] && self.grid[1][1] == self.grid[2][0] {
            return self.grid[0][2];
        }

        None
    }

    pub fn is_full(&self) -> bool {
        for row in &self.grid {
            for &cell in row {
                if cell.is_none() {
                    return false;
                }
            }
        }
        true
    }
}
