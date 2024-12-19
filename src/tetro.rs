const HEIGHT: usize = 10;
const WIDTH: usize = 10;

pub struct Grid {
    width: usize,
    height: usize,
    pub final_high: usize,
    board: [bool; HEIGHT * WIDTH],
    col_high: [i32; WIDTH],
    row_count: [usize; HEIGHT],
}

impl Grid {
    pub fn new() -> Self {
        Self {
            width: WIDTH,
            height: HEIGHT,
            final_high: 0,
            board: [false; HEIGHT * WIDTH],
            col_high: [HEIGHT as i32; WIDTH],
            row_count: [0; HEIGHT],
        }
    }

    pub fn show_board(&self) {
        for r in 0..self.height {
            let row: Vec<String> = (0..self.width)
                .map(|c| {
                    if self.board[self.get_index(r, c)] {
                        "#".to_string() // Filled cell
                    } else {
                        ".".to_string() // Empty cell
                    }
                })
                .collect();

            println!("{}", row.join(" "));
        }
        println!("---------------------");
    }

    // Validate position
    fn inbound(&self, row: i32, col: i32) -> bool {
        row >= 0 && col >= 0 && (row as usize) < self.height && (col as usize) < self.width
    }

    pub fn get_index(&self, row: usize, col: usize) -> usize {
        row * self.width + col
    }

    pub fn valid_block(&self, row: i32, col: i32, shape: &Vec<(i32, i32)>) -> bool {
        for &(dr, dc) in shape {
            let row = row + dr;
            let col = col + dc;
            if !self.inbound(row, col) {
                return false;
            }
            // println!("inbound at ({}, {})", row, col);
            if self.board[self.get_index(row as usize, col as usize)] {
                return false;
            }
        }
        true
    }

    /// Clears a specified row by resetting it to `false`
    pub fn clear_row(&mut self, row: usize) {
        for col in 0..self.width {
            self.board[self.get_index(row, col)] = false;
            self.col_high[col] = (self.col_high[col] + 1).clamp(0, self.height as i32);
        }
        self.row_count[row] = 0; // Reset row count
                                 // println!("Cleared row {}", row);
    }

    fn update_final_high(&mut self) {
        // println!("col high {:?}", self.col_high);
        if let Some(&min_col) = self.col_high.iter().min() {
            self.final_high = self.height - min_col as usize;
        } else {
            eprintln!("col_block is empty! Unable to calculate final_high.");
        }
    }

    /// Moves all rows above the cleared row down by one
    pub fn move_down(&mut self, cleared_row: usize) {
        for row in (1..=cleared_row).rev() {
            for col in 0..self.width {
                // Move content from the row above down
                let src_idx = self.get_index(row - 1, col);
                let dest_idx = self.get_index(row, col);

                self.board[dest_idx] = self.board[src_idx];
                self.row_count[row] = self.row_count[row - 1];
            }
        }

        // Clear the top row after shifting
        for col in 0..self.width {
            self.board[self.get_index(0, col)] = false;
        }
        self.row_count[0] = 0;

        // println!("Moved rows down from row {}", cleared_row);
    }
    // Place a piece
    pub fn place_piece(&mut self, position: i32, shape: &Vec<(i32, i32)>) {
        let mut st_row = shape
            .iter()
            .map(|&(_, c)| self.col_high[(position + c) as usize])
            .min()
            .unwrap_or((self.height - 1) as i32);

        // println!("good start {}", st_row);
        while !self.valid_block(st_row, position, shape) {
            // println!("move {}", st_row);
            st_row -= 1;
        }

        for &(dr, dc) in shape {
            let row = st_row + dr;
            let col = position + dc;
            // we know that the position is garanted to be valid
            // println!("Place at row {} and column {}", row, col);
            self.board[self.get_index(row as usize, col as usize)] = true;
            self.col_high[col as usize] = std::cmp::min(row, self.col_high[col as usize]);
            self.row_count[row as usize] += 1;
            if self.row_count[row as usize] == self.width {
                self.clear_row(row as usize);
                self.move_down(row as usize);
            }
        }

        // Use checked_add to prevent overflow
        self.update_final_high();
        // self.show_board();
    }
}
