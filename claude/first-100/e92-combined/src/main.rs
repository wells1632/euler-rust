use std::collections::HashMap;
use std::time::Instant;

fn main() {
    let limit = 10_000_000;
    
    let separator = "=".repeat(70);
    println!("{}", separator);
    println!("NUMBER CHAIN PROBLEM: Finding numbers that reach 89");
    println!("Limit: {} numbers", limit);
    println!("{}", separator);
    
    // Method 1: HashMap
    println!("\n--- METHOD 1: HashMap Caching ---");
    println!("Uses a HashMap to cache results dynamically.");
    println!("Memory usage grows as needed based on unique chain values.");
    println!();
    
    let start1 = Instant::now();
    let mut cache = HashMap::new();
    let count1 = count_with_hashmap(limit, &mut cache);
    let duration1 = start1.elapsed();
    
    println!("Result: {} numbers reach 89", count1);
    println!("Time: {:.3} seconds", duration1.as_secs_f64());
    println!("Cache size: {} entries", cache.len());
    
    // Method 2: Fixed Array
    println!("\n--- METHOD 2: Fixed Array ---");
    println!("Uses a pre-allocated 10MB array with direct indexing.");
    println!("Each index stores 0 (unknown), 1, or 89.");
    println!();
    
    let start2 = Instant::now();
    let mut memo = vec![0u8; limit as usize];
    memo[1] = 1;
    memo[89] = 89;
    let count2 = count_with_array(limit as usize, &mut memo);
    let duration2 = start2.elapsed();
    
    println!("Result: {} numbers reach 89", count2);
    println!("Time: {:.3} seconds", duration2.as_secs_f64());
    
    // Comparison
    println!("\n{}", separator);
    println!("COMPARISON:");
    println!("HashMap method: {:.3}s", duration1.as_secs_f64());
    println!("Array method:   {:.3}s", duration2.as_secs_f64());
    let speedup = duration1.as_secs_f64() / duration2.as_secs_f64();
    println!("Speedup: {:.2}x faster with array", speedup);
    println!("{}", separator);
}

// ===== METHOD 1: HashMap =====

fn sum_of_squared_digits_v1(mut n: u32) -> u32 {
    let mut sum = 0;
    while n > 0 {
        let digit = n % 10;
        sum += digit * digit;
        n /= 10;
    }
    sum
}

fn reaches_89(n: u32, cache: &mut HashMap<u32, bool>) -> bool {
    if n == 1 {
        return false;
    }
    if n == 89 {
        return true;
    }
    
    if let Some(&result) = cache.get(&n) {
        return result;
    }
    
    let next = sum_of_squared_digits_v1(n);
    let result = reaches_89(next, cache);
    cache.insert(n, result);
    result
}

fn count_with_hashmap(limit: u32, cache: &mut HashMap<u32, bool>) -> u32 {
    let mut count = 0;
    for i in 1..limit {
        if reaches_89(i, cache) {
            count += 1;
        }
        if i % 1_000_000 == 0 {
            print!("\rHashMap: {} million...", i / 1_000_000);
        }
    }
    println!();
    count
}

// ===== METHOD 2: Array =====

fn sum_of_squared_digits_v2(mut n: usize) -> usize {
    let mut sum = 0;
    while n > 0 {
        let digit = n % 10;
        sum += digit * digit;
        n /= 10;
    }
    sum
}

fn find_destination(n: usize, memo: &mut [u8]) -> u8 {
    if n < memo.len() && memo[n] != 0 {
        return memo[n];
    }
    
    let next = sum_of_squared_digits_v2(n);
    let result = find_destination(next, memo);
    
    if n < memo.len() {
        memo[n] = result;
    }
    
    result
}

fn count_with_array(limit: usize, memo: &mut [u8]) -> usize {
    let mut count = 0;
    
    for i in 1..limit {
        let destination = find_destination(i, memo);
        if destination == 89 {
            count += 1;
        }
        
        if i % 1_000_000 == 0 {
            print!("\rArray: {} million...", i / 1_000_000);
        }
    }
    println!();
    count
}
