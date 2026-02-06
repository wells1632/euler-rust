fn main() {
    let limit = 1_000_000;
    
    // Calculate totient values for all n up to limit
    let phi = compute_totients(limit);
    
    let mut max_ratio = 0.0;
    let mut max_n = 0;
    
    for n in 2..=limit {
        let ratio = n as f64 / phi[n] as f64;
        if ratio > max_ratio {
            max_ratio = ratio;
            max_n = n;
        }
    }
    
    println!("n with maximum n/phi(n): {}", max_n);
    println!("phi({}) = {}", max_n, phi[max_n]);
    println!("n/phi(n) = {:.10}", max_ratio);
    
    // Show prime factorization
    print!("Prime factorization: ");
    print_factorization(max_n);
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

fn print_factorization(mut n: usize) {
    let mut factors = Vec::new();
    let mut d = 2;
    
    while d * d <= n {
        while n % d == 0 {
            factors.push(d);
            n /= d;
        }
        d += 1;
    }
    
    if n > 1 {
        factors.push(n);
    }
    
    println!("{}", factors.iter()
        .map(|f| f.to_string())
        .collect::<Vec<_>>()
        .join(" × "));
}
