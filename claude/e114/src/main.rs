fn main() {
    println!("Counting ways to fill rows with red blocks (min length 3)\n");
    
    // Show progression for small values
    println!("Progression:");
    for len in 0..=10 {
        let ways = count_ways(len);
        println!("  Length {:2}: {:6} ways", len, ways);
    }
    
    println!();
    
    // Verify example
    let test = count_ways(7);
    if test == 17 {
        println!("✓ Length 7 verification passed (17 ways)\n");
    } else {
        println!("✗ ERROR: Expected 17, got {}\n", test);
        return;
    }
    
    // Calculate for length 50
    let result = count_ways(50);
    
    println!("{}", "=".repeat(70));
    println!("ANSWER: {} ways to fill a row of length 50", result);
    println!("{}", "=".repeat(70));
}

fn count_ways(length: usize) -> u64 {
    if length == 0 {
        return 1;
    }
    
    let mut dp = vec![0u64; length + 1];
    dp[0] = 1;
    
    for n in 1..=length {
        // End with grey square
        dp[n] = dp[n - 1];
        
        // End with red block of length k
        for block_len in 3..=n {
            if block_len == n {
                // Entire row is one red block
                dp[n] += 1;
            } else {
                // Need at least 1 grey before this red block
                let before = n - block_len - 1;
                dp[n] += dp[before];
            }
        }
    }
    
    dp[length]
}
