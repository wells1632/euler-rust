fn main() {
    println!("Diophantine Reciprocal: 1/x + 1/y = 1/n\n");
    
    // Verify known example
    let test = 1260u64;
    let test_div = count_divisors_of_square(test);
    let test_sol = (test_div + 1) / 2;
    println!("Verification: n = {} has {} solutions (expected 113)\n", test, test_sol);
    
    let target = 4_000_000u64;
    
    println!("Searching for smallest n with > {} solutions...\n", target);
    
    // Build base from primorial
    let primes = vec![2u64, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53];
    
    let mut best_n = u64::MAX;
    
    // Try different combinations more efficiently
    // Start with primorial base and add factors
    
    for num_primes in 10..=16 {
        let mut base = 1u64;
        
        for i in 0..num_primes {
            base = match base.checked_mul(primes[i]) {
                Some(v) => v,
                None => break,
            };
        }
        
        if base == 1 {
            continue;
        }
        
        println!("Trying base with {} primes: {}", num_primes, base);
        
        // Try multiplying by small factors
        for mult2 in 0..=8 {
            for mult3 in 0..=5 {
                for mult5 in 0..=3 {
                    for mult7 in 0..=2 {
                        
                        let n = match base
                            .checked_mul(2u64.pow(mult2))
                            .and_then(|v| v.checked_mul(3u64.pow(mult3)))
                            .and_then(|v| v.checked_mul(5u64.pow(mult5)))
                            .and_then(|v| v.checked_mul(7u64.pow(mult7))) {
                            Some(v) => v,
                            None => continue,
                        };
                        
                        if n >= best_n {
                            continue;
                        }
                        
                        let divisors = count_divisors_of_square(n);
                        let solutions = (divisors + 1) / 2;
                        
                        if solutions > target && n < best_n {
                            best_n = n;
                            println!("\nFound: n = {}", n);
                            println!("  Solutions: {}", solutions);
                            show_factorization(n);
                        }
                    }
                }
            }
        }
    }
    
    println!("\n{}", "=".repeat(70));
    println!("ANSWER: n = {}", best_n);
    println!("{}", "=".repeat(70));
}

fn count_divisors_of_square(n: u64) -> u64 {
    let factors = prime_factorization(n);
    let mut count = 1u64;
    
    for (_, exp) in factors {
        count = count.saturating_mul(2 * exp + 1);
    }
    
    count
}

fn prime_factorization(mut n: u64) -> Vec<(u64, u64)> {
    let mut factors = Vec::new();
    
    for p in 2..=100 {
        let mut exp = 0u64;
        while n % p == 0 {
            n /= p;
            exp += 1;
        }
        if exp > 0 {
            factors.push((p, exp));
        }
        if n == 1 {
            break;
        }
    }
    
    if n > 1 {
        factors.push((n, 1));
    }
    
    factors
}

fn show_factorization(n: u64) {
    let factors = prime_factorization(n);
    
    print!("  n = ");
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
    println!();
}
