fn sieve_of_eratosthenes(limit: usize) -> Vec<bool> {
    let mut is_prime = vec![true; limit + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    
    for i in 2..=((limit as f64).sqrt() as usize) {
        if is_prime[i] {
            for j in (i * i..=limit).step_by(i) {
                is_prime[j] = false;
            }
        }
    }
    is_prime
}

fn count_family_size(prime: u32, is_prime: &[bool]) -> usize {
    let s = prime.to_string();
    let digits: Vec<char> = s.chars().collect();
    let n = digits.len();
    let mut max_family = 0;
    
    // Try replacing each unique digit that appears in the number
    for target_digit in '0'..='9' {
        // Find all positions where this digit appears
        let positions: Vec<usize> = digits.iter()
            .enumerate()
            .filter(|(_, &d)| d == target_digit)
            .map(|(i, _)| i)
            .collect();
        
        if positions.is_empty() {
            continue;
        }
        
        // Try replacing all occurrences of this digit with each digit 0-9
        let mut family_count = 0;
        for replacement in '0'..='9' {
            let mut new_digits = digits.clone();
            for &pos in &positions {
                new_digits[pos] = replacement;
            }
            
            // Skip if leading zero
            if new_digits[0] == '0' {
                continue;
            }
            
            if let Ok(num) = new_digits.iter().collect::<String>().parse::<usize>() {
                if num < is_prime.len() && is_prime[num] {
                    family_count += 1;
                }
            }
        }
        
        max_family = max_family.max(family_count);
    }
    
    max_family
}

fn main() {
    let limit = 1_000_000;
    let is_prime = sieve_of_eratosthenes(limit);
    
    for candidate in 2..limit {
        if !is_prime[candidate] {
            continue;
        }
        
        if count_family_size(candidate as u32, &is_prime) == 8 {
            println!("Answer: {}", candidate);
            break;
        }
    }
}
