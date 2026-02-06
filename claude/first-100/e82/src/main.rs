use std::env;
use std::fs;
use std::process;
use std::cmp::min;

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

fn find_min_path_sum(matrix: &[Vec<i32>]) -> i32 {
    if matrix.is_empty() || matrix[0].is_empty() {
        return 0;
    }
    
    let rows = matrix.len();
    let cols = matrix[0].len();
    
    // dp[i][j] = minimum sum to reach cell (i, j)
    let mut dp = vec![vec![i32::MAX; cols]; rows];
    
    // Initialize first column - we can start at any cell
    for i in 0..rows {
        dp[i][0] = matrix[i][0];
    }
    
    // Process each column from left to right
    for j in 1..cols {
        // First pass: come from the left
        for i in 0..rows {
            if dp[i][j - 1] != i32::MAX {
                dp[i][j] = dp[i][j - 1] + matrix[i][j];
            }
        }
        
        // Second pass: propagate down
        for i in 1..rows {
            if dp[i - 1][j] != i32::MAX {
                dp[i][j] = min(dp[i][j], dp[i - 1][j] + matrix[i][j]);
            }
        }
        
        // Third pass: propagate up
        for i in (0..rows - 1).rev() {
            if dp[i + 1][j] != i32::MAX {
                dp[i][j] = min(dp[i][j], dp[i + 1][j] + matrix[i][j]);
            }
        }
    }
    
    // Find minimum in the last column
    let mut result = i32::MAX;
    for i in 0..rows {
        result = min(result, dp[i][cols - 1]);
    }
    
    result
}
