fn main() {
    let mut total = 0;
    
    for n in 1..=100 {
        let count = count_n_digit_nth_powers(n);
        if count > 0 {
            println!("n = {}: {} number(s)", n, count);
            total += count;
        } else {
            // No more solutions for larger n
            break;
        }
    }
    
    println!("\nTotal: {} numbers", total);
}

fn count_n_digit_nth_powers(n: u32) -> u32 {
    let mut count = 0;
    let mut k: u128 = 1;
    
    loop {
        // Compute k^n, checking for overflow
        let power = match k.checked_pow(n) {
            Some(p) => p,
            None => break, // Overflow means we've gone too far
        };
        
        let num_digits = count_digits(power);
        
        // If we've exceeded n digits, stop searching
        if num_digits > n {
            break;
        }
        
        // Count if this has exactly n digits
        if num_digits == n {
            count += 1;
        }
        
        k += 1;
    }
    
    count
}

fn count_digits(mut n: u128) -> u32 {
    if n == 0 {
        return 1;
    }
    
    let mut count = 0;
    while n > 0 {
        n /= 10;
        count += 1;
    }
    count
}
