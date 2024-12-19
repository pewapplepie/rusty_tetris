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
    // Create a new board
    let mut playground = Grid::new();

    // Use iterator chaining
    let result: Result<(), String> = line
        .split(',')
        .map(|element| element.split_at(1))
        .try_for_each(|(piece, pos_str)| {
            let position = pos_str
                .parse::<i32>()
                .map_err(|_| format!("Invalid position: {}", pos_str))?;

            let shape = tetris_map
                .get(piece)
                .ok_or_else(|| format!("Invalid tetromino: {}", piece))?;

            playground.place_piece(position, shape);
            Ok(())
        });

    match result {
        Ok(_) => playground.final_high.to_string(),
        Err(e) => {
            eprintln!("{}", e);
            "Invalid input".to_string()
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let tetrominoes = get_tetrominoes();

    for line in handle.lines() {
        match line {
            Ok(content) => {
                let output = process(&content, &tetrominoes);
                // println!("==========");
                println!("Final height: {:?}", output);
            }
            Err(e) => eprintln!("Error reading line: {}", e),
        }
    }
}
