fn digit_sum(mut n: u64) -> u64 {
    let mut sum = 0;
    while n > 0 {
        sum += n % 10;
        n /= 10;
    }
    sum
}

fn count_digits(mut n: u64) -> u32 {
    if n == 0 { return 1; }
    let mut count = 0;
    while n > 0 {
        count += 1;
        n /= 10;
    }
    count
}

fn main() {
    let mut sequence = Vec::new();
    
    // For each power p, check all possible digit sums
    for power in 2..=20 {
        // Maximum possible digit sum for reasonable numbers
        // A 20-digit number has max digit sum of 180
        for sum in 1..=180 {
            // Use checked_pow to avoid overflow
            if let Some(candidate) = (sum as u64).checked_pow(power) {
                // Skip if number is too large
                if candidate > 10_000_000_000_000_000 {
                    continue;
                }
                
                let ds = digit_sum(candidate);
                if ds == sum as u64 {
                    if !sequence.contains(&candidate) {
                        sequence.push(candidate);
                    }
                }
            }
        }
    }
    
    sequence.sort();
    
    println!("Found {} numbers in sequence:", sequence.len());
    for (i, num) in sequence.iter().take(35).enumerate() {
        let ds = digit_sum(*num);
        // Find which power works
        for p in 2..=20 {
            if let Some(result) = ds.checked_pow(p) {
                if result == *num {
                    println!("a({}) = {} = {}^{}", i, num, ds, p);
                    break;
                }
            }
        }
    }
    
    if sequence.len() >= 30 {
        println!("\na(30) = {}", sequence[30]);
    } else {
        println!("\nOnly found {} numbers", sequence.len());
    }
}
