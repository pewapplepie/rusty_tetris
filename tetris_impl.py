from typing import List

TETROMINOES = {
    "Q": [(0, 0), (0, 1), (-1, 0), (-1, 1)],
    "Z": [(-1, 0), (-1, 1), (0, 1), (0, 2)],
    "S": [(0, 0), (0, 1), (-1, 1), (-1, 2)],
    "T": [(-1, 0), (-1, 1), (-1, 2), (0, 1)],
    "I": [(0, 0), (0, 1), (0, 2), (0, 3)],
    "L": [(0, 0), (-1, 0), (-2, 0), (0, 1)],
    "J": [(0, 0), (0, 1), (-1, 1), (-2, 1)],
}

TETROSYMBOL = {"Q": "+", "Z": "*", "S": "x", "T": "#", "I": "@", "L": "~", "J": "%"}


class Grid:
    def __init__(self, width=10, height=10):
        self.width = width
        self.height = height
        self.grid_array = ["."] * (width * height)
        self.col_block = [height] * width
        self.row_count = [0] * height
        self.final_high = 0
        self.current_symbol = "+"

    def get_index(self, row, col):
        return row * self.width + col

    def full_line(self, line):
        return all(self.grid_array[line + k] != "." for k in range(self.width))

    def clear_line(self, line):
        self.grid_array[line : line + self.width] = ["."] * self.width
        for c in range(self.width):
            self.col_block[c] = min(self.col_block[c] + 1, self.height)

    def check_full_line(self, clear_line):
        lastrow = clear_line
        max_high = max(1, min(self.col_block))
        cancel = False

        for r in range(clear_line, max_high - 2, -1):
            line = lastrow * self.width
            runner = r * self.width

            if self.full_line(runner):
                self.clear_line(runner)
                cancel = True
            else:
                self.grid_array[line : line + self.width] = self.grid_array[
                    runner : runner + self.width
                ]
                lastrow -= 1

        for r in range(lastrow, -1, -1):
            line = r * self.width
            self.grid_array[line : line + self.width] = ["."] * self.width
        return cancel

    def place_piece(self, piece: List[tuple], position):
        st_row = self.height - 1
        while any(self.col_block[position + c] <= st_row + r for r, c in piece):
            st_row -= 1
            if st_row < 0:
                raise ValueError("Reach top, Game Over!!")

        for r, c in piece:
            row, col = st_row + r, position + c
            self.col_block[col] = min(row, self.col_block[col])
            pos = self.get_index(row, col)
            self.grid_array[pos] = self.current_symbol
            self.row_count[row] += 1
            if self.row_count[row] == self.width:
                self.check_full_line(row)

        self.final_high = self.height - min(self.col_block)


def show_board(playground: Grid) -> None:
    for r in range(playground.height):
        k = [
            playground.grid_array[playground.get_index(r, c)]
            for c in range(playground.width)
        ]
        print(" ".join(k))
    print("---------------------")


def play(case: List[str]) -> int:
    playground = Grid()
    for element in case:
        tetro = TETROMINOES.get(element[0])
        playground.current_symbol = TETROSYMBOL.get(element[0])
        pos = int(element[1])
        playground.place_piece(tetro, pos)
        show_board(playground)
    return playground.final_high


# unmark for python std usage
# import sys
# if __name__ == "__main__":
#     for line in sys.stdin:
#         result = play(line.strip().split(","))
#         print("Final building height:", result)
#         print()
