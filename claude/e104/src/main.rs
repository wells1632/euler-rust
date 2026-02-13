use std::time::Instant;

fn main() {
    println!("Finding first Fibonacci number with pandigital first and last 9 digits...\n");
    
    let start = Instant::now();
    let result = find_pandigital_fibonacci();
    let duration = start.elapsed();
    
    println!("\n{}", "=".repeat(70));
    println!("RESULT:");
    println!("Fibonacci index: {}", result);
    println!("Time taken: {:.3} seconds", duration.as_secs_f64());
    println!("{}", "=".repeat(70));
}

fn find_pandigital_fibonacci() -> u64 {
    let mut f_prev = 1u128;
    let mut f_curr = 1u128;
    let modulo = 1_000_000_000u128; // 10^9 for last 9 digits
    
    // For first 9 digits, use logarithms
    let phi = (1.0 + 5.0f64.sqrt()) / 2.0;
    let log_phi = phi.log10();
    let log_sqrt5 = 5.0f64.sqrt().log10();
    
    let mut index = 2u64;
    let mut last_checked = 0u64;
    let mut pandigital_last_count = 0u64;
    
    loop {
        index += 1;
        
        // Calculate next Fibonacci (last 9 digits only)
        let f_next = (f_prev + f_curr) % modulo;
        f_prev = f_curr;
        f_curr = f_next;
        
        // Progress indicator
        if index % 100000 == 0 {
            println!("Progress: index {}, found {} with pandigital last 9", 
                     index, pandigital_last_count);
            last_checked = index;
        }
        
        // Check last 9 digits first (cheaper test)
        if is_pandigital(f_curr as u64) {
            pandigital_last_count += 1;
            
            // Calculate first 9 digits using logarithms
            let log_fib = (index as f64) * log_phi - log_sqrt5;
            
            // Need at least 9 digits
            if log_fib < 8.0 {
                continue;
            }
            
            let log_fib_int = log_fib.floor();
            let log_fib_frac = log_fib - log_fib_int;
            
            // First 9 digits
            let first_digits = (10.0f64.powf(log_fib_frac + 8.0)) as u64;
            
            println!("  Candidate at index {}: first={}, last={}", 
                     index, first_digits, f_curr);
            
            if is_pandigital(first_digits) {
                println!("\n*** FOUND! ***");
                println!("Index: {}", index);
                println!("First 9 digits: {}", first_digits);
                println!("Last 9 digits: {}", f_curr);
                return index;
            }
        }
    }
}

fn is_pandigital(n: u64) -> bool {
    let mut digits = [false; 10];
    let mut num = n;
    let mut count = 0;
    
    while num > 0 {
        let digit = (num % 10) as usize;
        if digit == 0 || digits[digit] {
            return false; // Contains 0 or duplicate
        }
        digits[digit] = true;
        num /= 10;
        count += 1;
    }
    
    // Must have exactly 9 digits, all from 1-9
    count == 9 && digits[1..=9].iter().all(|&d| d)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_is_pandigital() {
        assert!(is_pandigital(123456789));
        assert!(is_pandigital(987654321));
        assert!(is_pandigital(192837465));
        assert!(!is_pandigital(123456788)); // Duplicate
        assert!(!is_pandigital(12345678)); // Only 8 digits
        assert!(!is_pandigital(1234567890)); // Contains 0
        assert!(!is_pandigital(123456780)); // Contains 0
    }
}
