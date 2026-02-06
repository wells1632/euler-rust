use std::env;
use std::fs;
use std::process;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        process::exit(1);
    }
    
    let filename = &args[1];
    
    let matrix = match read_matrix(filename) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            process::exit(1);
        }
    };
    
    let min_sum = find_min_path_sum(&matrix);
    
    println!("Minimum path sum: {}", min_sum);
}

fn read_matrix(filename: &str) -> Result<Vec<Vec<i32>>, Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(filename)?;
    
    let mut matrix = Vec::new();
    
    for line in contents.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        
        let row: Result<Vec<i32>, _> = line
            .split(|c: char| c == ',' || c.is_whitespace())
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<i32>())
            .collect();
        
        matrix.push(row?);
    }
    
    Ok(matrix)
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: i32,
    row: usize,
    col: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn find_min_path_sum(matrix: &[Vec<i32>]) -> i32 {
    if matrix.is_empty() || matrix[0].is_empty() {
        return 0;
    }
    
    let rows = matrix.len();
    let cols = matrix[0].len();
    
    // Use Dijkstra's algorithm
    let mut dist = vec![vec![i32::MAX; cols]; rows];
    let mut heap = BinaryHeap::new();
    
    // Start at top-left
    dist[0][0] = matrix[0][0];
    heap.push(State { cost: matrix[0][0], row: 0, col: 0 });
    
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)]; // right, down, left, up
    
    while let Some(State { cost, row, col }) = heap.pop() {
        // If we've reached the bottom-right, return the cost
        if row == rows - 1 && col == cols - 1 {
            return cost;
        }
        
        // Skip if we've already found a better path
        if cost > dist[row][col] {
            continue;
        }
        
        // Explore neighbors
        for (dr, dc) in directions.iter() {
            let new_row = row as i32 + dr;
            let new_col = col as i32 + dc;
            
            // Check bounds
            if new_row >= 0 && new_row < rows as i32 && new_col >= 0 && new_col < cols as i32 {
                let new_row = new_row as usize;
                let new_col = new_col as usize;
                
                let new_cost = cost + matrix[new_row][new_col];
                
                // If we found a better path, update and add to heap
                if new_cost < dist[new_row][new_col] {
                    dist[new_row][new_col] = new_cost;
                    heap.push(State { cost: new_cost, row: new_row, col: new_col });
                }
            }
        }
    }
    
    dist[rows - 1][cols - 1]
}
