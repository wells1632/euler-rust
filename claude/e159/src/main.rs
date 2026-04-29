use std::time::Instant;

fn digital_root(mut n: u64) -> u64 {
    if n == 0 {
        return 0;
    }
    let r = n % 9;
    if r == 0 { 9 } else { r }
}

fn max_drs(n: u64) -> u64 {
    if n < 4 {
        return digital_root(n);
    }

    let mut max = digital_root(n); // factorization: n itself

    // Try all factors from 2 up to sqrt(n)
    let mut i = 2;
    while i * i <= n {
        if n % i == 0 {
            let other = n / i;
            // Recursively get max DRS of the other factor, plus DR of i
            let candidate = digital_root(i) + max_drs(other);
            if candidate > max {
                max = candidate;
            }
        }
        i += 1;
    }

    max
}

fn main() {
    let start = Instant::now();

    let limit: u64 = 1_000_000;
    let mut sum: u64 = 0;

    for n in 2..limit {
        sum += max_drs(n);
    }

    let duration = start.elapsed();

    println!("Sum of mdrs(n) for 1 < n < 1,000,000: {}", sum);
    println!("Time elapsed: {:.3?}", duration);
}
