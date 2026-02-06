fn main() {
    let n = 100;
    
    let total_partitions = count_partitions(n);
    
    // Subtract 1 to exclude the trivial partition (100 itself)
    let result = total_partitions - 1;
    
    println!("Number of ways to write {} as a sum of at least two positive integers: {}", n, result);
}

fn count_partitions(n: usize) -> u64 {
    // dp[i] will store the number of partitions of i
    let mut dp = vec![0u64; n + 1];
    dp[0] = 1; // Empty partition
    
    // For each number k from 1 to n
    for k in 1..=n {
        // Update dp[i] for all i >= k
        for i in k..=n {
            dp[i] += dp[i - k];
        }
    }
    
    dp[n]
}
