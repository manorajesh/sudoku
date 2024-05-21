use std::ops::Range;

use rand::Rng;

struct Board([[Option<usize>; 9]; 9]);

impl Board {
    fn new() -> Self {
        Board([[None; 9]; 9])
    }

    fn set(&mut self, row: usize, col: usize, value: usize) {
        self.0[row][col] = Some(value);
    }

    fn valid_place(&self, row: usize, col: usize, value: usize) -> bool {
        // check rows and cols
        for i in 0..9 {
            if self.0[row][i] == Some(value) || self.0[i][col] == Some(value) {
                return false;
            }
        }

        // check 3x3 grid
        let row_start = (row / 3) * 3;
        let col_start = (col / 3) * 3;

        for i in 0..3 {
            for j in 0..3 {
                if self.0[row_start + i][col_start + j] == Some(value) {
                    return false;
                }
            }
        }

        true
    }

    fn place_number(
        &mut self,
        value: usize,
        row_range: Range<usize>,
        col_range: Range<usize>
    ) -> bool {
        for _ in 0..10 {
            let row = rand::thread_rng().gen_range(row_range.clone());
            let col = rand::thread_rng().gen_range(col_range.clone());
            if self.valid_place(row, col, value) {
                self.set(row, col, value);
                return true;
            }
        }
        false
    }

    fn populate_board_with_number(&mut self, value: usize) {
        for i in 0..9 {
            let row_range = (i * 3) % 9..((i + 1) * 3) % 9;
            let col_range = (i / 3) * 3..((i + 3) / 3) * 3;
            println!("row_range: {:?}, col_range: {:?}", row_range, col_range);
            if !self.place_number(value, row_range, col_range) {
                panic!("Could not place number");
            }
        }
    }

    fn print(&self) {
        for row in self.0.iter() {
            for cell in row.iter() {
                match cell {
                    Some(value) => print!("{} ", value),
                    None => print!(". "),
                }
            }
            println!();
        }
    }
}

fn main() {
    let mut board = Board::new();
    board.populate_board_with_number(1);
    board.print();
}
