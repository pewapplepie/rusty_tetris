mod shape;
mod tetris;

use crate::tetris::Grid;
use std::env;
use std::fs;
fn main() {
    println!("Hello, world!");
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {query}");
    println!("In file {file_path}");

    let contents = fs::read_to_string(file_path).expect("Error reading file");
    println!("With input text: {}", contents);

    let grid = Grid::new(10, 20);
    println!("Grid: {:?}", grid);
    println!("Grid width: {}", grid.width);
    println!("Grid height: {}", grid.height);
}
