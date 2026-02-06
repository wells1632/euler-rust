use num_bigint::BigUint;
use num_traits::{Zero, One};

fn main() {
    // Generate first 100 coefficients of continued fraction for e
    let coefficients = generate_e_coefficients(100);
    
    // Calculate the 100th convergent
    let (numerator, _denominator) = calculate_convergent(&coefficients);
    
    // Sum digits in numerator
    let digit_sum = sum_digits(&numerator);
    
    println!("100th convergent numerator: {}", numerator);
    println!("Sum of digits: {}", digit_sum);
}

fn generate_e_coefficients(n: usize) -> Vec<u64> {
    let mut coeffs = Vec::with_capacity(n);
    coeffs.push(2); // First coefficient is 2
    
    let mut k = 1u64;
    for i in 1..n {
        match i % 3 {
            1 => coeffs.push(1),
            2 => {
                coeffs.push(2 * k);
                k += 1;
            },
            0 => coeffs.push(1),
            _ => unreachable!(),
        }
    }
    
    coeffs
}

fn calculate_convergent(coeffs: &[u64]) -> (BigUint, BigUint) {
    if coeffs.is_empty() {
        return (Zero::zero(), One::one());
    }
    
    // Initialize: p_{-1} = 1, p_0 = a_0, q_{-1} = 0, q_0 = 1
    let mut p_prev2: BigUint = One::one();
    let mut p_prev1: BigUint = BigUint::from(coeffs[0]);
    let mut q_prev2: BigUint = Zero::zero();
    let mut q_prev1: BigUint = One::one();
    
    for &a in &coeffs[1..] {
        let a_big = BigUint::from(a);
        
        // p_n = a_n * p_{n-1} + p_{n-2}
        let p_n = &a_big * &p_prev1 + &p_prev2;
        
        // q_n = a_n * q_{n-1} + q_{n-2}
        let q_n = &a_big * &q_prev1 + &q_prev2;
        
        p_prev2 = p_prev1;
        p_prev1 = p_n;
        q_prev2 = q_prev1;
        q_prev1 = q_n;
    }
    
    (p_prev1, q_prev1)
}

fn sum_digits(n: &BigUint) -> u64 {
    n.to_string()
        .chars()
        .filter_map(|c| c.to_digit(10))
        .map(|d| d as u64)
        .sum()
}
