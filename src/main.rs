mod tetro;
use std::io::{self, BufRead};

use std::collections::HashMap;
use tetro::Grid;

type Tetromino = Vec<(i32, i32)>;
fn get_tetrominoes() -> HashMap<&'static str, Tetromino> {
    HashMap::from([
        ("Q", vec![(0, 0), (0, 1), (-1, 0), (-1, 1)]),
        ("Z", vec![(-1, 0), (-1, 1), (0, 1), (0, 2)]),
        ("S", vec![(0, 0), (0, 1), (-1, 1), (-1, 2)]),
        ("T", vec![(-1, 0), (-1, 1), (-1, 2), (0, 1)]),
        ("I", vec![(0, 0), (0, 1), (0, 2), (0, 3)]),
        ("L", vec![(0, 0), (-1, 0), (-2, 0), (0, 1)]),
        ("J", vec![(0, 0), (0, 1), (-1, 1), (-2, 1)]),
    ])
}

fn process(line: &str, tetris_map: &HashMap<&str, Tetromino>) -> String {
    // println!("New game: {}", line);
    let mut playground = Grid::new();

    for element in line.split(',').enumerate() {
        let (piece, pos_str) = element.1.split_at(1);
        if let Ok(position) = pos_str.parse::<i32>() {
            match tetris_map.get(piece) {
                Some(shape) => {
                    // println!("{position} at {:?} ", shape);
                    playground.place_piece(position, shape);
                    // println!("=====");
                }
                None => eprintln!("Invalid tetromino: {}", piece),
            }
        } else {
            eprintln!("Invalid position: {}", pos_str);
            return "Invalid input".to_string();
        }
    }
    playground.final_high.to_string()
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let tetrominoes = get_tetrominoes();

    for line in handle.lines() {
        match line {
            Ok(content) => {
                let output = process(&content, &tetrominoes);
                println!("Final height: {:?}", output);
            }
            Err(e) => eprintln!("Error reading line: {}", e),
        }
    }
}
