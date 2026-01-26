fn main() {
    let limit = 10_000_000;
    
    // Generate primes up to sqrt(limit) * 2 to be safe
    let primes = sieve_of_eratosthenes((limit as f64).sqrt() as usize * 2);
    
    let mut min_ratio = f64::INFINITY;
    let mut best_n = 0;
    let mut best_phi = 0;
    
    // Check products of two primes (semiprimes have smallest n/phi(n) after primes)
    for i in 0..primes.len() {
        for j in i..primes.len() {
            let p = primes[i] as u64;
            let q = primes[j] as u64;
            let n = p * q;
            
            if n >= limit {
                break;
            }
            
            // For semiprime n = p*q, phi(n) = (p-1)(q-1)
            let phi = (p - 1) * (q - 1);
            
            if is_permutation(n, phi) {
                let ratio = n as f64 / phi as f64;
                if ratio < min_ratio {
                    min_ratio = ratio;
                    best_n = n;
                    best_phi = phi;
                }
            }
        }
    }
    
    println!("n = {}", best_n);
    println!("phi(n) = {}", best_phi);
    println!("n/phi(n) = {:.10}", min_ratio);
}

fn sieve_of_eratosthenes(limit: usize) -> Vec<usize> {
    let mut is_prime = vec![true; limit + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    
    for i in 2..=((limit as f64).sqrt() as usize) {
        if is_prime[i] {
            for j in (i * i..=limit).step_by(i) {
                is_prime[j] = false;
            }
        }
    }
    
    is_prime.iter()
        .enumerate()
        .filter_map(|(i, &prime)| if prime { Some(i) } else { None })
        .collect()
}

fn is_permutation(a: u64, b: u64) -> bool {
    let mut digits_a = digit_counts(a);
    let mut digits_b = digit_counts(b);
    
    digits_a == digits_b
}

fn digit_counts(mut n: u64) -> [u8; 10] {
    let mut counts = [0u8; 10];
    
    while n > 0 {
        counts[(n % 10) as usize] += 1;
        n /= 10;
    }
    
    counts
}
