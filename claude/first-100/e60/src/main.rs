use std::time::Instant;
use std::sync::{Arc, Mutex};
use rayon::prelude::*;

fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    let sqrt_n = (n as f64).sqrt() as u64;
    for i in (3..=sqrt_n).step_by(2) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn concatenate(a: u64, b: u64) -> u64 {
    let b_digits = b.to_string().len() as u32;
    a * 10u64.pow(b_digits) + b
}

fn are_pair_compatible(a: u64, b: u64) -> bool {
    is_prime(concatenate(a, b)) && is_prime(concatenate(b, a))
}

fn find_sequential(primes: &[u64]) -> Option<(u64, Vec<u64>)> {
    for i in 0..primes.len() {
        for j in i+1..primes.len() {
            if !are_pair_compatible(primes[i], primes[j]) {
                continue;
            }
            
            for k in j+1..primes.len() {
                if !are_pair_compatible(primes[i], primes[k]) 
                    || !are_pair_compatible(primes[j], primes[k]) {
                    continue;
                }
                
                for l in k+1..primes.len() {
                    if !are_pair_compatible(primes[i], primes[l])
                        || !are_pair_compatible(primes[j], primes[l])
                        || !are_pair_compatible(primes[k], primes[l]) {
                        continue;
                    }
                    
                    for m in l+1..primes.len() {
                        if !are_pair_compatible(primes[i], primes[m])
                            || !are_pair_compatible(primes[j], primes[m])
                            || !are_pair_compatible(primes[k], primes[m])
                            || !are_pair_compatible(primes[l], primes[m]) {
                            continue;
                        }
                        
                        let sum = primes[i] + primes[j] + primes[k] + primes[l] + primes[m];
                        let set = vec![primes[i], primes[j], primes[k], primes[l], primes[m]];
                        return Some((sum, set));
                    }
                }
            }
        }
    }
    None
}

fn find_parallel(primes: &[u64]) -> Option<(u64, Vec<u64>)> {
    let found = Arc::new(Mutex::new(false));
    
    (0..primes.len()).into_par_iter().find_map_any(|i| {
        for j in i+1..primes.len() {
            if *found.lock().unwrap() {
                return None;
            }
            
            if !are_pair_compatible(primes[i], primes[j]) {
                continue;
            }
            
            for k in j+1..primes.len() {
                if *found.lock().unwrap() {
                    return None;
                }
                
                if !are_pair_compatible(primes[i], primes[k]) 
                    || !are_pair_compatible(primes[j], primes[k]) {
                    continue;
                }
                
                for l in k+1..primes.len() {
                    if *found.lock().unwrap() {
                        return None;
                    }
                    
                    if !are_pair_compatible(primes[i], primes[l])
                        || !are_pair_compatible(primes[j], primes[l])
                        || !are_pair_compatible(primes[k], primes[l]) {
                        continue;
                    }
                    
                    for m in l+1..primes.len() {
                        if *found.lock().unwrap() {
                            return None;
                        }
                        
                        if !are_pair_compatible(primes[i], primes[m])
                            || !are_pair_compatible(primes[j], primes[m])
                            || !are_pair_compatible(primes[k], primes[m])
                            || !are_pair_compatible(primes[l], primes[m]) {
                            continue;
                        }
                        
                        let sum = primes[i] + primes[j] + primes[k] + primes[l] + primes[m];
                        let set = vec![primes[i], primes[j], primes[k], primes[l], primes[m]];
                        *found.lock().unwrap() = true;
                        return Some((sum, set));
                    }
                }
            }
        }
        None
    })
}

fn main() {
    let limit = 10000;
    let mut primes = Vec::new();
    
    // Generate primes
    println!("Generating primes up to {}...\n", limit);
    for n in 2..limit {
        if is_prime(n) {
            primes.push(n);
        }
    }
    println!("Generated {} primes\n", primes.len());
    
    // Sequential version
    println!("=== SEQUENTIAL VERSION ===");
    let start = Instant::now();
    match find_sequential(&primes) {
        Some((sum, set)) => {
            let duration = start.elapsed();
            println!("Found set: {:?}", set);
            println!("Sum: {}", sum);
            println!("Time: {:.3}s ({} ms)\n", duration.as_secs_f64(), duration.as_millis());
        }
        None => {
            let duration = start.elapsed();
            println!("No set found within the limit.");
            println!("Time: {:.3}s ({} ms)\n", duration.as_secs_f64(), duration.as_millis());
        }
    }
    
    // Parallel version
    println!("=== PARALLEL VERSION ===");
    let start = Instant::now();
    match find_parallel(&primes) {
        Some((sum, set)) => {
            let duration = start.elapsed();
            println!("Found set: {:?}", set);
            println!("Sum: {}", sum);
            println!("Time: {:.3}s ({} ms)\n", duration.as_secs_f64(), duration.as_millis());
        }
        None => {
            let duration = start.elapsed();
            println!("No set found within the limit.");
            println!("Time: {:.3}s ({} ms)\n", duration.as_secs_f64(), duration.as_millis());
        }
    }
}
