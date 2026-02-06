fn mod_pow(base: u64, exp: u64, modulus: u64) -> u64 {
    let mut result = 1u64;
    let mut base = base % modulus;
    let mut exp = exp;
    
    while exp > 0 {
        if exp % 2 == 1 {
            result = ((result as u128 * base as u128) % modulus as u128) as u64;
        }
        exp >>= 1;
        base = ((base as u128 * base as u128) % modulus as u128) as u64;
    }
    
    result
}

fn main() {
    let modulus = 10_000_000_000u64; // 10^10 for last 10 digits
    let base = 2u64;
    let exponent = 7830457u64;
    let multiplier = 28433u64;
    
    // Calculate (28433 × 2^7830457 + 1) mod 10^10
    let power_result = mod_pow(base, exponent, modulus);
    let result = (((multiplier as u128 * power_result as u128) % modulus as u128) as u64 + 1) % modulus;
    
    println!("The last 10 digits of 28433 × 2^7830457 + 1 are: {:010}", result);
}
