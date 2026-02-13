use std::collections::HashSet;

fn main() {
    println!("Finding optimum special sum set with 7 elements...\n");
    
    // Based on known patterns, search around optimal ranges
    // Known optimal for 6 elements: {11, 18, 19, 20, 22, 25} sum=115
    // For 7 elements, we expect sum around 155-160
    
    let mut best_set = Vec::new();
    let mut best_sum = u32::MAX;
    
    // Search systematically around known good values
    for a in 20..=21 {
        for b in a+1..=32 {
            for c in b+1..=39 {
                for d in c+1..=40 {
                    for e in d+1..=41 {
                        for f in e+1..=43 {
                            for g in f+1..=46 {
                                let candidate = vec![a, b, c, d, e, f, g];
                                
                                if is_special_sum_set(&candidate) {
                                    let sum: u32 = candidate.iter().sum();
                                    
                                    if sum < best_sum {
                                        best_sum = sum;
                                        best_set = candidate.clone();
                                        println!("Found: {:?}, sum = {}", best_set, best_sum);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    
    println!("\n{}", "=".repeat(70));
    println!("RESULT:");
    println!("Optimum special sum set: {:?}", best_set);
    println!("Sum: {}", best_sum);
    let concatenated: String = best_set.iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join("");
    println!("Concatenated: {}", concatenated);
    println!("{}", "=".repeat(70));
}

fn is_special_sum_set(set: &[u32]) -> bool {
    let n = set.len();
    
    // Property 2: If |B| > |C|, then S(B) > S(C)
    // For a sorted set, check smallest k > largest k-1
    for k in 2..=n/2 + 1 {
        let sum_smallest_k: u32 = set.iter().take(k).sum();
        let sum_largest_k_minus_1: u32 = set.iter().rev().take(k - 1).sum();
        
        if sum_smallest_k <= sum_largest_k_minus_1 {
            return false;
        }
    }
    
    // Property 1: All subset sums are unique
    let mut subset_sums = HashSet::new();
    
    for mask in 1..(1 << n) {
        let mut sum = 0;
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
