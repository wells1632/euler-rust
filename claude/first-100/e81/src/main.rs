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
    
    // Create DP table
    let mut dp = vec![vec![0; cols]; rows];
    
    // Initialize starting position
    dp[0][0] = matrix[0][0];
    
    // Fill first row (can only come from left)
    for j in 1..cols {
        dp[0][j] = dp[0][j - 1] + matrix[0][j];
    }
    
    // Fill first column (can only come from above)
    for i in 1..rows {
        dp[i][0] = dp[i - 1][0] + matrix[i][0];
    }
    
    // Fill rest of the table
    for i in 1..rows {
        for j in 1..cols {
            dp[i][j] = min(dp[i - 1][j], dp[i][j - 1]) + matrix[i][j];
        }
    }
    
    dp[rows - 1][cols - 1]
}
