use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        eprintln!("Example: {} sets.txt", args[0]);
        std::process::exit(1);
    }
    
    let filename = &args[1];
    
    match process_sets_file(filename) {
        Ok(total) => {
            println!("\n{}", "=".repeat(70));
            println!("FINAL ANSWER: {}", total);
            println!("{}", "=".repeat(70));
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

fn process_sets_file(filename: &str) -> Result<u32, Box<dyn std::error::Error>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    
    let mut total_sum = 0u32;
    let mut qualifying_sets = Vec::new();
    let mut line_num = 0;
    
    println!("Processing sets from file: {}\n", filename);
    
    for line in reader.lines() {
        line_num += 1;
        let line = line?;
        
        if line.trim().is_empty() {
            continue;
        }
        
        let set: Result<Vec<u32>, _> = line
            .split(',')
            .map(|s| s.trim().parse::<u32>())
            .collect();
        
        match set {
            Ok(mut s) => {
                s.sort();
                
                if is_special_sum_set(&s) {
                    let set_sum: u32 = s.iter().sum();
                    println!("Line {}: {:?} - QUALIFIES (sum: {})", line_num, s, set_sum);
                    total_sum += set_sum;
                    qualifying_sets.push(s);
                } else {
                    println!("Line {}: {:?} - does not qualify", line_num, s);
                }
            }
            Err(e) => {
                eprintln!("Line {}: Parse error: {}", line_num, e);
            }
        }
    }
    
    println!("\n{}", "=".repeat(70));
    println!("SUMMARY:");
    println!("Total sets processed: {}", line_num);
    println!("Qualifying sets: {}", qualifying_sets.len());
    println!("\nQualifying sets:");
    for (i, set) in qualifying_sets.iter().enumerate() {
        println!("  {}: {:?} (sum: {})", i + 1, set, set.iter().sum::<u32>());
    }
    println!("\nTotal sum: {}", total_sum);
    println!("{}", "=".repeat(70));
    
    Ok(total_sum)
}

fn is_special_sum_set(set: &[u32]) -> bool {
    let n = set.len();
    
    if n == 0 {
        return false;
    }
    
    // Property 1: If subset B has more elements than subset C, then S(B) > S(C)
    // For sorted set: sum of k smallest > sum of (k-1) largest
    for k in 2..=n/2 + 1 {
        if k > n {
            break;
        }
        
        let sum_smallest_k: u32 = set.iter().take(k).sum();
        let sum_largest_k_minus_1: u32 = set.iter().rev().take(k - 1).sum();
        
        if sum_smallest_k <= sum_largest_k_minus_1 {
            return false;
        }
    }
    
    // Property 2: All subset sums must be unique
    let mut subset_sums = HashSet::new();
    
    for mask in 1..(1 << n) {
        let mut sum = 0u32;
        
        for i in 0..n {
            if mask & (1 << i) != 0 {
                sum += set[i];
            }
        }
        
        if !subset_sums.insert(sum) {
            return false;
        }
    }
    
    true
}
