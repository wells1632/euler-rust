use std::collections::HashMap;

fn main() {
    println!("Finding least n where F(50, n) > 1,000,000\n");
    
    let m = 50;
    let target = 1_000_000u64;
    
    let mut cache = HashMap::new();
    
    println!("Searching...");
    
    for n in m.. {
        let ways = count_ways_cached(m, n, &mut cache);
        
        if n % 10 == 0 {
            println!("  F({}, {}) = {}", m, n, ways);
        }
        
        if ways > target {
            println!("\n{}", "=".repeat(70));
            println!("ANSWER: n = {}", n);
            println!("F({}, {}) = {}", m, n, ways);
            println!("{}", "=".repeat(70));
            break;
        }
    }
}

fn count_ways_cached(m: usize, n: usize, cache: &mut HashMap<(usize, usize), u64>) -> u64 {
    if let Some(&result) = cache.get(&(m, n)) {
        return result;
    }
    
    let result = count_ways(m, n);
    cache.insert((m, n), result);
    result
}

fn count_ways(m: usize, n: usize) -> u64 {
    let mut f = vec![0u64; n + 1];
    f[0] = 1;
    
    for len in 1..=n {
        f[len] = f[len - 1];
        
        for k in m..=len {
            if k == len {
                f[len] += 1;
            } else {
                f[len] += f[len - k - 1];
            }
        }
    }
    
    f[n]
}
