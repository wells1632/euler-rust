fn main() {
    let limit = 100_000_000u64;
    let mut palindromic_sums = std::collections::HashSet::new();
    
    // Try all possible starting points for consecutive squares
    let max_start = (limit as f64).sqrt() as u64;
    
    for start in 1..=max_start {
        let mut sum = 0u64;
        
        // Add consecutive squares starting from 'start'
        for i in start..=max_start {
            sum += i * i;
            
            if sum >= limit {
                break;
            }
            
            // Need at least 2 consecutive squares
            if i > start && is_palindrome(sum) {
                palindromic_sums.insert(sum);
            }
        }
    }
    
    let total: u64 = palindromic_sums.iter().sum();
    
    println!("Found {} palindromic numbers", palindromic_sums.len());
    println!("Sum of all palindromic consecutive square sums: {}", total);
    
    // Show some examples
    let mut examples: Vec<u64> = palindromic_sums.iter().copied().collect();
    examples.sort();
    println!("\nFirst 10 examples:");
    for (i, &num) in examples.iter().take(10).enumerate() {
        println!("{}. {}", i + 1, num);
    }
}

fn is_palindrome(n: u64) -> bool {
    let s = n.to_string();
    let bytes = s.as_bytes();
    let len = bytes.len();
    
    for i in 0..len / 2 {
        if bytes[i] != bytes[len - 1 - i] {
            return false;
        }
    }
    
    true
}
