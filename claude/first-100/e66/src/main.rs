use num_bigint::BigUint;
use num_traits::{Zero, One};

fn main() {
    let limit = 1000;
    let mut max_x = BigUint::zero();
    let mut max_d = 0;
    
    for d in 2..=limit {
        if is_perfect_square(d) {
            continue;
        }
        
        let (x, _y) = solve_pell(d);
        
        if x > max_x {
            max_x = x.clone();
            max_d = d;
        }
    }
    
    println!("D with largest minimal x: {}", max_d);
    println!("x = {}", max_x);
}

fn is_perfect_square(n: u64) -> bool {
    let sqrt = (n as f64).sqrt() as u64;
    sqrt * sqrt == n
}

fn solve_pell(d: u64) -> (BigUint, BigUint) {
    let a0 = (d as f64).sqrt() as u64;
    
    // Generate continued fraction coefficients and convergents
    let mut m = 0i64;
    let mut d_val = 1i64;
    let mut a = a0 as i64;
    
    let mut p_prev2 = BigUint::one();
    let mut p_prev1 = BigUint::from(a0);
    let mut q_prev2 = BigUint::zero();
    let mut q_prev1 = BigUint::one();
    
    loop {
        // Next coefficient
        m = d_val * a - m;
        d_val = (d as i64 - m * m) / d_val;
        a = (a0 as i64 + m) / d_val;
        
        // Next convergent
        let a_big = BigUint::from(a as u64);
        let p_n = &a_big * &p_prev1 + &p_prev2;
        let q_n = &a_big * &q_prev1 + &q_prev2;
        
        // Check if this convergent solves Pell's equation
        let x_squared = &p_n * &p_n;
        let dy_squared = BigUint::from(d) * &q_n * &q_n;
        
        if x_squared == dy_squared + BigUint::one() {
            return (p_n, q_n);
        }
        
        p_prev2 = p_prev1;
        p_prev1 = p_n;
        q_prev2 = q_prev1;
        q_prev1 = q_n;
    }
}
