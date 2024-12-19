## Tetris Engine - Getting Started

This is a pre-built Rust application for processing Tetris piece placements from an input file. You can also rebuild the binary if you have cargo installed.

1. Get the repository:

```bash
cd tetris_engine
```

2. This is the pre-build application, but if you want to update, can rebuild via the following
   (make sure you have cargo installed)

```bash
cargo build --release
```

This generates the binary in ./target/release/tetris_engine (Linux/Mac) or target\release\tetris_engine.exe (Windows).

Based on the description, the application expects an `input.txt` file with the following format:

For example

```plaintext
I0,I4,Q8
T1,Z3,I4
L0,J1,I6,S3,Z6,T3,Q0,Z5,Q8
Z3
L0,J1,I0,S1,Z0,I0,Q4,Q4,Q4,Q4,Q4,I6,I6,I6,I6,I6,Q8,Q7
Q0,I2,I6,I0,I6,I6,Q2,Q4
```

3. Use the following command to run the program with the input file:

```bash
./target/release/tetris_engine < input.txt > output.txt    # Linux/Mac
target\release\tetris_engine.exe < input.txt > output.txt  # Windows
```

This command will read the input from `input.txt`, process it, and write the final heights of each line-seqeuence to `output.txt`.

## Notes

Project Notes

- `main.rs` – Entry point of the Tetris engine application.

- `tetro.rs` – Contains the logic for handling tetromino placements and height calculations.

- `tetris_impl.py` – A Python implementation for baseline testing and debugging. This version includes graphical examples to help understand the mechanics.

Custom Input: You can modify input.txt to test different scenarios.

Debug Output: Uncomment show_board() in tetro.rs to display the game board for better visualization.

Let me know if you need additional adjustments or more detailed sections! 🚀
