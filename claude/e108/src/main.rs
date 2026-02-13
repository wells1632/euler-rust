fn main() {
    println!("Finding least n where 1/x + 1/y = 1/n has > 1000 distinct solutions\n");
    
    let target = 1000;
    
    for n in 1.. {
        let ordered_solutions = count_divisors_of_square(n);
        // For unordered solutions: (τ(n²) + 1) / 2
        // Since n² always has odd number of divisors (perfect square)
        let unordered_solutions = (ordered_solutions + 1) / 2;
        
        if unordered_solutions > target {
            println!("\n{}", "=".repeat(70));
            println!("ANSWER: n = {}", n);
            println!("Number of distinct (unordered) solutions: {}", unordered_solutions);
            println!("Number of ordered pairs: {}", ordered_solutions);
            println!("{}", "=".repeat(70));
            
            println!("\nPrime factorization of n:");
            show_prime_factorization(n);
            
            println!("\nFirst 10 solutions:");
            verify_solutions(n, 10);
            break;
        }
        
        if n % 1000 == 0 {
            println!("n = {}: {} unordered solutions ({} ordered)", 
                     n, unordered_solutions, ordered_solutions);
        }
    }
}

fn count_divisors_of_square(n: u64) -> u64 {
    // Count divisors of n²
    // If n = p₁^a₁ × p₂^a₂ × ... × pₖ^aₖ
    // Then n² = p₁^(2a₁) × p₂^(2a₂) × ... × pₖ^(2aₖ)
    // τ(n²) = (2a₁ + 1)(2a₂ + 1)...(2aₖ + 1)
    
    let factors = prime_factorization(n);
    let mut count = 1u64;
    
    for (_, exp) in factors {
        count *= 2 * exp + 1;
    }
    
    count
}

fn prime_factorization(mut n: u64) -> Vec<(u64, u64)> {
    let mut factors = Vec::new();
    
    // Check for 2
    let mut exp = 0;
    while n % 2 == 0 {
        n /= 2;
        exp += 1;
    }
    if exp > 0 {
        factors.push((2, exp));
    }
    
    // Check odd numbers
    let mut p = 3u64;
    while p * p <= n {
        let mut exp = 0;
        while n % p == 0 {
            n /= p;
            exp += 1;
        }
        if exp > 0 {
            factors.push((p, exp));
        }
        p += 2;
    }
    
    if n > 1 {
        factors.push((n, 1));
    }
    
    factors
}

fn show_prime_factorization(n: u64) {
    let factors = prime_factorization(n);
    
    print!("n = {}", n);
    if !factors.is_empty() {
        print!(" = ");
        for (i, (p, exp)) in factors.iter().enumerate() {
            if i > 0 {
                print!(" × ");
            }
            if *exp == 1 {
                print!("{}", p);
            } else {
                print!("{}^{}", p, exp);
            }
        }
    }
    println!();
    
    println!("n² has {} divisors", count_divisors_of_square(n));
}

fn verify_solutions(n: u64, limit: usize) {
    let n_squared = n * n;
    let divisors = get_divisors(n_squared);
    
    println!("Showing unordered solutions:");
    
    let mut count = 0;
    for &d in &divisors {
        if d > n {
            break; // Only show unique unordered pairs
        }
        
        if count >= limit {
            break;
        }
        
        let x = n + d;
        let y = n_squared / d + n;
        
        if x <= y {
            println!("  {{x={}, y={}}} → 1/{} + 1/{} = 1/{}", x, y, x, y, n);
            count += 1;
        }
    }
    
    if divisors.len() / 2 > limit {
        println!("  ... and {} more", (divisors.len() + 1) / 2 - limit);
    }
}

fn get_divisors(n: u64) -> Vec<u64> {
    let mut divisors = Vec::new();
    let mut i = 1u64;
    
    while i * i <= n {
        if n % i == 0 {
            divisors.push(i);
            if i != n / i {
                divisors.push(n / i);
            }
        }
        i += 1;
    }
    
    divisors.sort();
    divisors
}
