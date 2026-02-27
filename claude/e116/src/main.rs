fn main() {
    println!("Three-Color Tile Arrangement Problem\n");
    
    let row_length = 50;
    
    // Verify example
    println!("Verification:");
    let red_5 = count_arrangements(2, 5);
    println!("  F(2, 5) = {} (expected 7)", red_5);
    
    if red_5 != 7 {
        println!("  ✗ Failed!");
        return;
    }
    println!("  ✓ Correct!\n");
    
    // Calculate for length 50
    println!("For row length {}:", row_length);
    let red = count_arrangements(2, row_length);
    let green = count_arrangements(3, row_length);
    let blue = count_arrangements(4, row_length);
    
    println!("  Red tiles (length 2):   {:>20} ways", red);
    println!("  Green tiles (length 3): {:>20} ways", green);
    println!("  Blue tiles (length 4):  {:>20} ways", blue);
    println!("                          {:>20}", "-".repeat(20));
    
    let total = red + green + blue;
    
    println!("  Total:                  {:>20}", total);
    
    println!("\n{}", "=".repeat(70));
    println!("ANSWER: {}", total);
    println!("{}", "=".repeat(70));
}

fn count_arrangements(tile_len: usize, row_len: usize) -> u64 {
    // f[i] = number of ways to fill first i positions with at least one tile
    
    let mut dp = vec![0u64; row_len + 1];
    dp[0] = 1;  // Base case: empty row
    
    for i in 1..=row_len {
        // Option 1: Don't place a tile ending at position i
        dp[i] = dp[i - 1];
        
        // Option 2: Place a tile ending at position i
        if i >= tile_len {
            dp[i] += dp[i - tile_len];
        }
    }
    
    // Subtract 1 to exclude the empty arrangement
    dp[row_len] - 1
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_red_5() {
        assert_eq!(count_arrangements(2, 5), 7);
    }
    
    #[test]
    fn test_manual_count() {
        // For length 2, row 5:
        // 1. RR at 0-1
        // 2. RR at 1-2
        // 3. RR at 2-3
        // 4. RR at 3-4
        // 5. RR at 0-1, RR at 2-3
        // 6. RR at 0-1, RR at 3-4
        // 7. RR at 1-2, RR at 3-4
        assert_eq!(count_arrangements(2, 5), 7);
    }
}
