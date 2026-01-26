fn main() {
    let limit = 1_000_000;
    
    // Compute Euler's totient function for all d
    let phi = compute_totients(limit);
    
    // Sum phi(d) for d from 2 to limit
    let total: u64 = (2..=limit).map(|d| phi[d] as u64).sum();
    
    println!("Number of reduced proper fractions for d <= {}: {}", limit, total);
}

fn compute_totients(limit: usize) -> Vec<usize> {
    let mut phi: Vec<usize> = (0..=limit).collect();
    
    for i in 2..=limit {
        if phi[i] == i {
            // i is prime
            for j in (i..=limit).step_by(i) {
                phi[j] -= phi[j] / i;
            }
        }
    }
    
    phi
}
