use std::collections::{HashMap, HashSet};

fn main() {
    let limit = 1_000_000;
    let target_length = 60;
    
    // Pre-compute factorials 0! through 9!
    let factorials: [u64; 10] = [1, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880];
    
    // Memoization cache for chain lengths
    let mut cache: HashMap<u64, usize> = HashMap::new();
    
    let mut count = 0;
    
    for n in 1..limit {
        let length = chain_length(n, &factorials, &mut cache);
        if length == target_length {
            count += 1;
        }
    }
    
    println!("Number of chains with exactly {} non-repeating terms: {}", target_length, count);
}

fn chain_length(start: u64, factorials: &[u64; 10], cache: &mut HashMap<u64, usize>) -> usize {
    if let Some(&length) = cache.get(&start) {
        return length;
    }
    
    let mut seen = HashSet::new();
    let mut chain = Vec::new();
    let mut current = start;
    
    while !seen.contains(&current) {
        // Check if we've cached this number
        if let Some(&cached_len) = cache.get(&current) {
            let total_len = chain.len() + cached_len;
            
            // Cache all numbers in our chain
            for (i, &num) in chain.iter().enumerate() {
                cache.insert(num, total_len - i);
            }
            
            return total_len;
        }
        
        seen.insert(current);
        chain.push(current);
        current = digit_factorial_sum(current, factorials);
    }
    
    let length = chain.len();
    
    // Cache all numbers in the chain
    for (i, &num) in chain.iter().enumerate() {
        cache.insert(num, length - i);
    }
    
    length
}

fn digit_factorial_sum(mut n: u64, factorials: &[u64; 10]) -> u64 {
    let mut sum = 0;
    
    while n > 0 {
        sum += factorials[(n % 10) as usize];
        n /= 10;
    }
    
    sum
}
