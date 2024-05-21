use std::time::Instant;

use rand::seq::SliceRandom;
use rand::thread_rng;

struct Board([[Option<usize>; 9]; 9]);

impl Board {
    fn new() -> Self {
        Board([[None; 9]; 9])
    }

    fn set(&mut self, row: usize, col: usize, value: usize) {
        self.0[row][col] = Some(value);
    }

    fn valid_place(&self, row: usize, col: usize, value: usize) -> bool {
        // Check row and column
        for i in 0..9 {
            if self.0[row][i] == Some(value) || self.0[i][col] == Some(value) {
                return false;
            }
        }

        // Check 3x3 grid
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

    fn find_empty(&self) -> Option<(usize, usize)> {
        for row in 0..9 {
            for col in 0..9 {
                if self.0[row][col].is_none() {
                    return Some((row, col));
                }
            }
        }
        None
    }

    // Recusrive backtracking
    fn solve(&mut self) -> bool {
        // find empty cell
        if let Some((row, col)) = self.find_empty() {
            let mut numbers: Vec<usize> = (1..=9).collect();
            numbers.shuffle(&mut thread_rng());
            // Try each number
            for &num in &numbers {
                // If valid place, set the number and try to solve the rest
                if self.valid_place(row, col, num) {
                    self.set(row, col, num);
                    if self.solve() {
                        return true;
                    }
                    // If not valid, reset the cell
                    self.0[row][col] = None;
                }
            }
            false
        } else {
            true
        }
    }

    fn print(&self) {
        for row in self.0.iter() {
            for cell in row.iter() {
                match cell {
                    Some(value) => print!("\x1b[3{}m{}\x1b[0m ", (value % 7) + 1, value),
                    None => print!(". "),
                }
            }
            println!();
        }
    }
}

fn main() {
    let mut board = Board::new();

    let now = Instant::now();
    if board.solve() {
        board.print();
        println!("Time taken: {:?}", now.elapsed());
    } else {
        println!("No solution found");
    }
}
