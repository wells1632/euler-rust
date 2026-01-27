use num_bigint::BigInt;
use num_traits::{ToPrimitive, FromPrimitive};

fn main() {
    let limit = 100;
    let digits = 100;
    
    let mut total_sum = 0u64;
    
    for n in 1..=limit {
        // Skip perfect squares
        if is_perfect_square(n) {
            continue;
        }
        
        // Calculate first 100 significant digits of sqrt(n)
        let significant_digits = sqrt_significant_digits(n, digits);
        
        // Sum the digits
        let digit_sum: u64 = significant_digits.iter().map(|&d| d as u64).sum();
        
        println!("sqrt({:3}) -> digit sum: {}", n, digit_sum);
        
        total_sum += digit_sum;
    }
    
    println!("\nTotal of all digital sums: {}", total_sum);
}

fn is_perfect_square(n: u64) -> bool {
    let sqrt = (n as f64).sqrt() as u64;
    sqrt * sqrt == n
}

fn sqrt_significant_digits(n: u64, num_digits: usize) -> Vec<u8> {
    // We need enough decimal precision
    // If integer part has k digits, we need num_digits - k decimal places
    let int_part = (n as f64).sqrt() as u64;
    let int_digits = int_part.to_string().len();
    let decimal_digits_needed = num_digits - int_digits;
    
    // Scale n by 10^(2 * decimal_digits_needed + extra)
    let extra = 5;
    let total_scale = decimal_digits_needed + extra;
    
    let mut scaled = BigInt::from(n);
    for _ in 0..total_scale {
        scaled *= 10;
        scaled *= 10;
    }
    
    // Compute integer square root
    let sqrt_scaled = isqrt(&scaled);
    let s = sqrt_scaled.to_string();
    
    // Extract all digits (integer + decimal)
    let mut result = Vec::new();
    for ch in s.chars().take(num_digits + int_digits) {
        if let Some(d) = ch.to_digit(10) {
            result.push(d as u8);
        }
    }
    
    // Take exactly num_digits
    result.truncate(num_digits);
    
    result
}

fn isqrt(n: &BigInt) -> BigInt {
    if n <= &BigInt::from(0) {
        return BigInt::from(0);
    }
    
    let mut x = n.clone();
    let two = BigInt::from(2);
    
    loop {
        let x_new = (&x + n / &x) / &two;
        if x_new >= x {
            return x;
        }
        x = x_new;
    }
}
