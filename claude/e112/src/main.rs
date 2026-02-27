fn main() {
    let target_proportion = 0.99;
    
    println!("Finding the least number where bouncy proportion = {}%\n", target_proportion * 100.0);
    
    let mut bouncy_count = 0u64;
    let mut n = 1u64;
    
    loop {
        if is_bouncy_fast(n) {
            bouncy_count += 1;
        }
        
        // Check if we've hit exactly 99%
        // bouncy_count / n = 0.99
        // bouncy_count = 0.99 * n
        // 100 * bouncy_count = 99 * n
        if 100 * bouncy_count == 99 * n {
            println!("\n{}", "=".repeat(70));
            println!("ANSWER: {}", n);
            println!("Bouncy count: {}", bouncy_count);
            println!("Proportion: {:.6} ({:.2}%)", 
                     bouncy_count as f64 / n as f64, 
                     (bouncy_count as f64 / n as f64) * 100.0);
            println!("{}", "=".repeat(70));
            break;
        }
        
        n += 1;
        
        if n % 100000 == 0 {
            let proportion = bouncy_count as f64 / n as f64;
            println!("Progress: n = {:>10}, bouncy = {:>10}, proportion = {:.6}", 
                     n, bouncy_count, proportion);
        }
    }
}

fn is_bouncy_fast(mut n: u64) -> bool {
    if n < 10 {
        return false;
    }
    
    let mut last_digit = (n % 10) as u8;
    n /= 10;
    
    let mut has_increase = false;
    let mut has_decrease = false;
    
    while n > 0 {
        let digit = (n % 10) as u8;
        
        if digit < last_digit {
            has_increase = true;
        } else if digit > last_digit {
            has_decrease = true;
        }
        
        // Early exit
        if has_increase && has_decrease {
            return true;
        }
        
        last_digit = digit;
        n /= 10;
    }
    
    false
}
