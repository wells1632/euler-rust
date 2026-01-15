use num_bigint::BigUint;

fn count_digits(n: &BigUint) -> usize {
    n.to_string().len()
}

fn main() {
    // Initial values for sqrt(2) continued fraction
    // p_0 = 1, p_1 = 3
    // q_0 = 1, q_1 = 2
    let mut p_prev = BigUint::from(1u32);  // p_{n-2}
    let mut p_curr = BigUint::from(3u32);  // p_{n-1}
    let mut q_prev = BigUint::from(1u32);  // q_{n-2}
    let mut q_curr = BigUint::from(2u32);  // q_{n-1}
    
    let mut count = 0;
    
    // First expansion is 3/2
    if count_digits(&p_curr) > count_digits(&q_curr) {
        count += 1;
    }
    
    // Generate expansions 2 through 1000
    for i in 2..=1000 {
        // Calculate next convergent using recurrence relation:
        // p_n = 2*p_{n-1} + p_{n-2}
        // q_n = 2*q_{n-1} + q_{n-2}
        let p_next = &p_curr * 2u32 + &p_prev;
        let q_next = &q_curr * 2u32 + &q_prev;
        
        // Check if numerator has more digits than denominator
        if count_digits(&p_next) > count_digits(&q_next) {
            count += 1;
        }
        
        // Update for next iteration
        p_prev = p_curr;
        p_curr = p_next;
        q_prev = q_curr;
        q_curr = q_next;
    }
    
    println!("Number of fractions (out of first 1000) where numerator has more digits: {}", count);
}