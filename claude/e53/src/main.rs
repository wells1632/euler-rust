fn binomial_coefficient(n: u128, r: u128) -> u128 {
    if r > n {
        return 0;
    }
    if r == 0 || r == n {
        return 1;
    }
    
    // Use the smaller of r and n-r for efficiency
    let r = r.min(n - r);
    
    let mut result: u128 = 1;
    for i in 0..r {
        result = result * (n - i) / (i + 1);
    }
    
    result
}

fn main() {
    let mut count = 0;
    let threshold = 1_000_000u128;
    
    for n in 1..=100 {
        for r in 0..=n {
            let value = binomial_coefficient(n, r);
            if value > threshold {
                count += 1;
            }
        }
    }
    
    println!("Number of binomial coefficients C(n,r) with 1≤n≤100 that exceed 1,000,000: {}", count);
}
