#[derive(Debug)]
pub struct Grid {
    pub width: i32,
    pub height: i32,
    grid_array: Vec<Vec<bool>>,
    full_lines: usize,
}
impl Grid {
    pub fn new(width: i32, height: i32) -> Self {
        Grid {
            width,
            height,
            grid_array: vec![vec![false; width as usize]; height as usize],
            full_lines: 0,
        }
    }

    fn get_grid_array(&self) -> &Vec<Vec<bool>> {
        &self.grid_array
    }

    fn check_full_lines(&mut self) {
        let mut row = self.height as isize - 1;
        for y in (0..self.height as isize).rev() {
            // Copy current row into the target row
            for x in 0..self.width {
                self.grid_array[row as usize][x as usize] = self.grid_array[y as usize][x as usize];
            }

            // Check if the line is full
            if self.grid_array[y as usize]
                .iter()
                .filter(|&&cell| cell)
                .count()
                < self.width as usize
            {
                row -= 1; // Only decrease if row wasn't full
            } else {
                // Clear the current row
                for x in 0..self.width {
                    self.grid_array[row as usize][x as usize] = false; // Clear row
                }
                self.full_lines += 1; // Count cleared lines
            }
        }
    }
}
