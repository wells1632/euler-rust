fn main() {
    println!("Mixed-Color Tile Arrangement\n");
    
    let row_length = 50;
    
    // Show what's being counted
    println!("This counts all possible arrangements including:");
    println!("  - Completely blank rows");
    println!("  - Any single color");
    println!("  - Mixed colors (e.g., Red+Green+Blue)");
    println!();
    
    let result = count_mixed_arrangements(row_length);
    
    // Compare to single-color counts
    println!("For comparison:");
    println!("  Red only (length 2):   {}", count_single_color(2, row_length));
    println!("  Green only (length 3): {}", count_single_color(3, row_length));
    println!("  Blue only (length 4):  {}", count_single_color(4, row_length));
    println!("  Mixed colors:          {}", result);
    println!();
    
    println!("{}", "=".repeat(70));
    println!("ANSWER: {} total arrangements", result);
    println!("{}", "=".repeat(70));
}

fn count_mixed_arrangements(row_len: usize) -> u64 {
    let mut dp = vec![0u64; row_len + 1];
    dp[0] = 1;
    
    for i in 1..=row_len {
        dp[i] = dp[i - 1];
        
        if i >= 2 { dp[i] += dp[i - 2]; }
        if i >= 3 { dp[i] += dp[i - 3]; }
        if i >= 4 { dp[i] += dp[i - 4]; }
    }
    
    dp[row_len]
}

fn count_single_color(tile_len: usize, row_len: usize) -> u64 {
    let mut dp = vec![0u64; row_len + 1];
    dp[0] = 1;
    
    for i in 1..=row_len {
        dp[i] = dp[i - 1];
        if i >= tile_len {
            dp[i] += dp[i - tile_len];
        }
    }
    
    dp[row_len]
}
