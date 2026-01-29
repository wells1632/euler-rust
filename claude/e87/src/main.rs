use std::collections::HashSet;

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
        .filter(|(_, &prime)| prime)
        .map(|(i, _)| i)
        .collect()
}

fn main() {
    let limit = 50_000_000u64;
    
    // Find the maximum prime we need
    // For p^2: p < sqrt(50000000) ≈ 7071
    // For p^3: p < cbrt(50000000) ≈ 368
    // For p^4: p < 4thrt(50000000) ≈ 84
    let max_prime = (limit as f64).sqrt() as usize + 1;
    
    println!("Generating primes up to {}...", max_prime);
    let primes = sieve_of_eratosthenes(max_prime);
    println!("Found {} primes", primes.len());
    
    // Generate prime powers
    println!("Generating prime powers...");
    let squares: Vec<u64> = primes.iter()
        .map(|&p| (p as u64).pow(2))
        .filter(|&sq| sq < limit)
        .collect();
    
    let cubes: Vec<u64> = primes.iter()
        .map(|&p| (p as u64).pow(3))
        .filter(|&cb| cb < limit)
        .collect();
    
    let fourth_powers: Vec<u64> = primes.iter()
        .map(|&p| (p as u64).pow(4))
        .filter(|&fp| fp < limit)
        .collect();
    
    println!("Squares: {}, Cubes: {}, Fourth powers: {}", 
             squares.len(), cubes.len(), fourth_powers.len());
    
    // Find all valid sums
    println!("Computing sums...");
    let mut valid_numbers = HashSet::new();
    
    for &sq in &squares {
        for &cb in &cubes {
            if sq + cb >= limit {
                break;
            }
            for &fp in &fourth_powers {
                let sum = sq + cb + fp;
                if sum >= limit {
                    break;
                }
                valid_numbers.insert(sum);
            }
        }
    }
    
    println!("\nAnswer: {} numbers below {} can be expressed as p² + q³ + r⁴", 
             valid_numbers.len(), limit);
}
