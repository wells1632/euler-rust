use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        process::exit(1);
    }
    
    let filename = &args[1];
    
    let triangle = match read_triangle(filename) {
        Ok(tri) => tri,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            process::exit(1);
        }
    };
    
    let max_total = find_max_path(&triangle);
    
    println!("Maximum total: {}", max_total);
}

fn read_triangle(filename: &str) -> Result<Vec<Vec<i32>>, Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(filename)?;
    
    let mut triangle = Vec::new();
    
    for line in contents.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        
        let row: Result<Vec<i32>, _> = line
            .split_whitespace()
            .map(|s| s.parse::<i32>())
            .collect();
        
        triangle.push(row?);
    }
    
    Ok(triangle)
}

fn find_max_path(triangle: &[Vec<i32>]) -> i32 {
    if triangle.is_empty() {
        return 0;
    }
    
    // Create a mutable copy for dynamic programming
    let mut dp = triangle.to_vec();
    
    // Work from second-to-last row up to the top
    for i in (0..dp.len() - 1).rev() {
        for j in 0..dp[i].len() {
            // For each position, add the maximum of the two adjacent values below
            dp[i][j] += dp[i + 1][j].max(dp[i + 1][j + 1]);
        }
    }
    
    dp[0][0]
}
