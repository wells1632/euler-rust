fn main() {
    let limit = 100000; // Start with a reasonable upper bound
    let target = 5000;
    
    // Generate primes
    let primes = sieve_of_eratosthenes(limit);
    
    // Count prime partitions, using Option to track if we've exceeded target
    let mut dp = vec![0u64; limit + 1];
    let mut exceeded = vec![false; limit + 1];
    dp[0] = 1; // Base case
    
    for &prime in &primes {
        for i in prime..=limit {
            if !exceeded[i] {
                if let Some(new_val) = dp[i].checked_add(dp[i - prime]) {
                    dp[i] = new_val;
                    if dp[i] > target {
                        exceeded[i] = true;
                    }
                } else {
                    // Overflow means definitely > target
                    exceeded[i] = true;
                }
            }
        }
    }
    
    // Find the first value with more than target ways
    for i in 1..=limit {
        if dp[i] > target || exceeded[i] {
            println!("First value with more than {} prime partitions: {}", target, i);
            println!("Number of ways: {}", if exceeded[i] { "over 5000".to_string() } else { dp[i].to_string() });
            return;
        }
    }
    
    println!("Need to increase limit!");
}

fn sieve_of_eratosthenes(limit: usize) -> Vec<usize> {
    let mut is_prime = vec![true; limit + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    
    for i in 2..=((limit as f64).sqrt() as usize) {
        if is_prime[i] {
            for j in (i * i..=limit).step_by(i) {
                is_prime[j] = false;
            }
        }
    }
    
    is_prime.iter()
        .enumerate()
        .filter_map(|(i, &prime)| if prime { Some(i) } else { None })
        .collect()
}
