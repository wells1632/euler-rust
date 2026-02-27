fn main() {
    let target = 10_u64.pow(10);
    let mut n = 1;
    
    loop {
        let prime = nth_prime(n);
        let remainder = compute_remainder(prime, n);
        
        if remainder > target {
            println!("First n where remainder > 10^10: {}", n);
            println!("P({}) = {}", n, prime);
            println!("Remainder = {}", remainder);
            break;
        }
        
        // Verify example: n=3, P(3)=5
        if n == 3 {
            println!("Verification: n=3, P(3)={}, remainder={}", prime, remainder);
        }
        
        n += 1;
    }
}

fn compute_remainder(prime: u64, n: u64) -> u64 {
    let p_squared = prime * prime;
    
    let p_minus_1 = (prime - 1) % p_squared;
    let p_plus_1 = (prime + 1) % p_squared;
    
    let term1 = mod_pow(p_minus_1, n, p_squared);
    let term2 = mod_pow(p_plus_1, n, p_squared);
    
    (term1 + term2) % p_squared
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

fn nth_prime(n: u64) -> u64 {
    if n == 1 {
        return 2;
    }
    
    let mut count = 1;
    let mut candidate = 3;
    
    while count < n {
        if is_prime(candidate) {
            count += 1;
        }
        if count < n {
            candidate += 2;
        }
    }
    
    candidate
}

fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    
    let mut i = 3;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 2;
    }
    
    true
}
