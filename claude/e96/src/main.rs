use std::env;
use std::fs;
use std::process;
use std::time::Instant;

#[derive(Clone)]
struct Sudoku {
    grid: [[u8; 9]; 9],
}

impl Sudoku {
    fn new() -> Self {
        Sudoku { grid: [[0; 9]; 9] }
    }

    fn is_valid(&self, row: usize, col: usize, num: u8) -> bool {
        // Check row
        for i in 0..9 {
            if self.grid[row][i] == num {
                return false;
            }
        }

        // Check column
        for i in 0..9 {
            if self.grid[i][col] == num {
                return false;
            }
        }

        // Check 3x3 box
        let box_row = (row / 3) * 3;
        let box_col = (col / 3) * 3;
        for i in 0..3 {
            for j in 0..3 {
                if self.grid[box_row + i][box_col + j] == num {
                    return false;
                }
            }
        }

        true
    }

    fn solve(&mut self) -> bool {
        for row in 0..9 {
            for col in 0..9 {
                if self.grid[row][col] == 0 {
                    for num in 1..=9 {
                        if self.is_valid(row, col, num) {
                            self.grid[row][col] = num;

                            if self.solve() {
                                return true;
                            }

                            self.grid[row][col] = 0;
                        }
                    }
                    return false;
                }
            }
        }
        true
    }

    fn get_first_three(&self) -> u32 {
        (self.grid[0][0] as u32) * 100 + (self.grid[0][1] as u32) * 10 + (self.grid[0][2] as u32)
    }
}

fn parse_puzzles(filename: &str) -> Vec<Sudoku> {
    let contents = fs::read_to_string(filename)
        .unwrap_or_else(|_| {
            eprintln!("Error reading file: {}", filename);
            process::exit(1);
        });

    let lines: Vec<&str> = contents.lines().collect();
    let mut puzzles = Vec::new();
    let mut i = 0;

    while i < lines.len() {
        if i + 9 < lines.len() {
            let mut sudoku = Sudoku::new();
            
            // Skip title line
            i += 1;

            // Parse 9 lines of puzzle
            for row in 0..9 {
                let line = lines[i + row].replace(" ", "");
                for (col, ch) in line.chars().enumerate() {
                    if col < 9 {
                        sudoku.grid[row][col] = ch.to_digit(10).unwrap_or(0) as u8;
                    }
                }
            }

            puzzles.push(sudoku);
            i += 9;
        } else {
            break;
        }
    }

    puzzles
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        process::exit(1);
    }

    let filename = &args[1];
    let mut puzzles = parse_puzzles(filename);

    let mut total = 0u32;
    let overall_start = Instant::now();

    for (i, puzzle) in puzzles.iter_mut().enumerate() {
        let start = Instant::now();
        
        if puzzle.solve() {
            let duration = start.elapsed();
            let first_three = puzzle.get_first_three();
            
            println!("Puzzle {}: First three digits = {} (solved in {:.3}ms)", 
                     i + 1, first_three, duration.as_secs_f64() * 1000.0);
            total += first_three;
        } else {
            let duration = start.elapsed();
            eprintln!("Puzzle {} could not be solved! (attempted for {:.3}ms)", 
                     i + 1, duration.as_secs_f64() * 1000.0);
        }
    }

    let overall_duration = overall_start.elapsed();

    println!("\nTotal sum: {}", total);
    println!("Total time: {:.3}ms", overall_duration.as_secs_f64() * 1000.0);
}
