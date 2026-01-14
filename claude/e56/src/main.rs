use num_bigint::BigUint;

fn digital_sum(n: &BigUint) -> u32 {
    n.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .sum()
}

fn main() {
    let mut max_sum = 0;
    let mut max_a = 0u32;
    let mut max_b = 0u32;
    
    for a in 1u32..100 {
        for b in 1u32..100 {
            let base = BigUint::from(a);
            let power = base.pow(b);
            let sum = digital_sum(&power);
            
            if sum > max_sum {
                max_sum = sum;
                max_a = a;
                max_b = b;
            }
        }
    }
    
    println!("Maximum digital sum: {}", max_sum);
    println!("Found at: {}^{}", max_a, max_b);
    println!();
    
    // Calculate and show the verification
    let result = BigUint::from(max_a).pow(max_b);
    let verification = digital_sum(&result);
    
    println!("99^95 = {}", result);
    println!();
    println!("Verification - Sum of digits: {}", verification);
}
