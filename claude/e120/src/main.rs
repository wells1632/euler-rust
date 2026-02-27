fn main() {
    let mut total_sum: u64 = 0;
    
    for a in 3..=1000 {
        let max_r = find_max_remainder(a);
        total_sum += max_r;
        
        // Let's verify a=7 gives 42
        if a == 7 {
            println!("For a=7, max_r = {}", max_r);
        }
    }
    
    println!("Sum of all maximum remainders: {}", total_sum);
}

fn find_max_remainder(a: u64) -> u64 {
    let a_squared = a * a;
    let mut max_remainder = 0u64;
    
    // Check many more values of n to ensure we find the true maximum
    // The pattern might have a longer cycle
    for n in 1..=10000 {
        let remainder = compute_remainder(a, n, a_squared);
        if remainder > max_remainder {
            max_remainder = remainder;
        }
        
        // Early termination: if we've seen a² - 1, that's the theoretical max
        if max_remainder == a_squared - 1 {
            break;
        }
    }
    
    max_remainder
}

fn compute_remainder(a: u64, n: u64, a_squared: u64) -> u64 {
    // Compute ((a-1)^n + (a+1)^n) mod a²
    let a_minus_1 = (a - 1) % a_squared;
    let a_plus_1 = (a + 1) % a_squared;
    
    let term1 = mod_pow(a_minus_1, n, a_squared);
    let term2 = mod_pow(a_plus_1, n, a_squared);
    
    (term1 + term2) % a_squared
}

fn mod_pow(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
    if modulus == 1 {
        return 0;
    }
    
    let mut result = 1u64;
    base %= modulus;
    
    while exp > 0 {
        if exp % 2 == 1 {
            result = mod_mul(result, base, modulus);
        }
        exp >>= 1;
        base = mod_mul(base, base, modulus);
    }
    
    result
}

fn mod_mul(a: u64, b: u64, modulus: u64) -> u64 {
    ((a as u128 * b as u128) % modulus as u128) as u64
}
