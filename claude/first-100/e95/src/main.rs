use std::collections::HashSet;
use std::time::Instant;

fn main() {
    let limit = 1_000_000usize;
    
    println!("{}", "=".repeat(70));
    println!("AMICABLE CHAIN FINDER - PERFORMANCE COMPARISON");
    println!("{}", "=".repeat(70));
    
    // Method 1: Using HashSet
    println!("\n--- METHOD 1: HashSet for chain tracking ---");
    let start1 = Instant::now();
    let (chain1, length1, smallest1) = find_longest_chain_hashset(limit);
    let duration1 = start1.elapsed();
    
    println!("Result: Chain length = {}", length1);
    println!("Chain: {:?}", chain1);
    println!("Smallest member: {}", smallest1);
    println!("Time: {:.3} seconds", duration1.as_secs_f64());
    
    // Method 2: Using fixed array
    println!("\n--- METHOD 2: Fixed array for chain tracking ---");
    let start2 = Instant::now();
    let (chain2, length2, smallest2) = find_longest_chain_array(limit);
    let duration2 = start2.elapsed();
    
    println!("Result: Chain length = {}", length2);
    println!("Chain: {:?}", chain2);
    println!("Smallest member: {}", smallest2);
    println!("Time: {:.3} seconds", duration2.as_secs_f64());
    
    // Comparison
    println!("\n{}", "=".repeat(70));
    println!("COMPARISON:");
    println!("HashSet method: {:.3}s", duration1.as_secs_f64());
    println!("Array method:   {:.3}s", duration2.as_secs_f64());
    let speedup = duration1.as_secs_f64() / duration2.as_secs_f64();
    println!("Speedup: {:.2}x faster with array", speedup);
    println!("{}", "=".repeat(70));
}

fn find_longest_chain_hashset(limit: usize) -> (Vec<usize>, usize, usize) {
    println!("Computing sum of divisors...");
    
    let divisor_start = Instant::now();
    let mut divisor_sum = vec![0usize; limit + 1];
    for i in 1..=limit {
        for j in (2*i..=limit).step_by(i) {
            divisor_sum[j] += i;
        }
    }
    let divisor_duration = divisor_start.elapsed();
    println!("  Divisor computation: {:.3} seconds", divisor_duration.as_secs_f64());
    
    println!("Finding amicable chains...");
    let chain_start = Instant::now();
    
    let mut longest_chain = Vec::new();
    let mut longest_length = 0;
    let mut visited = vec![false; limit + 1];
    
    for start in 2..=limit {
        if visited[start] {
            continue;
        }
        
        let mut chain = Vec::new();
        let mut seen = HashSet::new();
        let mut current = start;
        
        loop {
            if current == 0 || current > limit {
                break;
            }
            
            if seen.contains(&current) {
                if let Some(pos) = chain.iter().position(|&x| x == current) {
                    let cycle: Vec<usize> = chain[pos..].to_vec();
                    
                    for &num in &cycle {
                        if num <= limit {
                            visited[num] = true;
                        }
                    }
                    
                    if cycle.len() > longest_length {
                        longest_length = cycle.len();
                        longest_chain = cycle.clone();
                    }
                }
                break;
            }
            
            seen.insert(current);
            chain.push(current);
            current = divisor_sum[current];
            
            if chain.len() > 1000 {
                break;
            }
        }
    }
    
    let chain_duration = chain_start.elapsed();
    println!("  Chain search: {:.3} seconds", chain_duration.as_secs_f64());
    
    let smallest = *longest_chain.iter().min().unwrap_or(&0);
    (longest_chain, longest_length, smallest)
}

fn find_longest_chain_array(limit: usize) -> (Vec<usize>, usize, usize) {
    println!("Computing sum of divisors...");
    
    let divisor_start = Instant::now();
    let mut divisor_sum = vec![0usize; limit + 1];
    for i in 1..=limit {
        for j in (2*i..=limit).step_by(i) {
            divisor_sum[j] += i;
        }
    }
    let divisor_duration = divisor_start.elapsed();
    println!("  Divisor computation: {:.3} seconds", divisor_duration.as_secs_f64());
    
    println!("Finding amicable chains...");
    let chain_start = Instant::now();
    
    let mut longest_chain = Vec::new();
    let mut longest_length = 0;
    let mut visited = vec![false; limit + 1];
    let mut in_current_chain = vec![false; limit + 1];
    
    for start in 2..=limit {
        if visited[start] {
            continue;
        }
        
        let mut chain = Vec::new();
        let mut current = start;
        
        loop {
            if current == 0 || current > limit {
                // Clean up
                for &num in &chain {
                    if num <= limit {
                        in_current_chain[num] = false;
                    }
                }
                break;
            }
            
            if in_current_chain[current] {
                if let Some(pos) = chain.iter().position(|&x| x == current) {
                    let cycle: Vec<usize> = chain[pos..].to_vec();
                    
                    for &num in &cycle {
                        if num <= limit {
                            visited[num] = true;
                        }
                    }
                    
                    if cycle.len() > longest_length {
                        longest_length = cycle.len();
                        longest_chain = cycle.clone();
                    }
                }
                
                // Clean up
                for &num in &chain {
                    if num <= limit {
                        in_current_chain[num] = false;
                    }
                }
                break;
            }
            
            in_current_chain[current] = true;
            chain.push(current);
            current = divisor_sum[current];
            
            if chain.len() > 1000 {
                // Clean up
                for &num in &chain {
                    if num <= limit {
                        in_current_chain[num] = false;
                    }
                }
                break;
            }
        }
    }
    
    let chain_duration = chain_start.elapsed();
    println!("  Chain search: {:.3} seconds", chain_duration.as_secs_f64());
    
    let smallest = *longest_chain.iter().min().unwrap_or(&0);
    (longest_chain, longest_length, smallest)
}
