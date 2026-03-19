fn sieve(limit: usize) -> Vec<bool> {
    let mut is_prime = vec![true; limit];
    is_prime[0] = false;
    is_prime[1] = false;
    let mut i = 2;
    while i * i < limit {
        if is_prime[i] {
            let mut j = i * i;
            while j < limit {
                is_prime[j] = false;
                j += i;
            }
        }
        i += 1;
    }
    is_prime
}

fn is_perfect_cube(n: i128) -> bool {
    if n == 0 { return true; }
    let cbrt = (n as f64).cbrt() as i128;
    for c in (cbrt - 2).max(0)..=cbrt + 2 {
        if c * c * c == n {
            return true;
        }
    }
    false
}

fn has_property(p: i128) -> bool {
    for d in 1..p {
        if 4 * p < 3 * d {
            break;
        }

        let discriminant = d * d * d * (4 * p - 3 * d);
        let sqrt_disc = (discriminant as f64).sqrt() as i128;

        let mut found_sqrt: Option<i128> = None;
        for s in (sqrt_disc - 2).max(0)..=sqrt_disc + 2 {
            if s * s == discriminant {
                found_sqrt = Some(s);
                break;
            }
        }

        let s = match found_sqrt {
            Some(s) => s,
            None => continue,
        };

        if p == 3 * d {
            continue;
        }

        let denom = 2 * (p - 3 * d);

        for numer in [3 * d * d + s, 3 * d * d - s] {
            if denom != 0 && numer > 0 && numer % denom == 0 {
                let n = numer / denom;
                if n > 0 && is_perfect_cube(n * n * (n + p)) {
                    return true;
                }
            }
        }
    }
    false
}

fn main() {
    let limit = 1_000_000usize;
    let is_prime = sieve(limit);

    let primes: Vec<usize> = (2..limit).filter(|&p| is_prime[p]).collect();
    let total_primes = primes.len();
    println!("Sieve complete. Checking {} primes...", total_primes);

    let mut count = 0;
    let mut checked = 0;
    let report_every = 10_000;

    for &p in &primes {
        if has_property(p as i128) {
            count += 1;
            println!("  Found: p = {} (running total: {})", p, count);
        }
        checked += 1;
        if checked % report_every == 0 {
            println!(
                "  Progress: {}/{} primes checked ({:.1}%), found {} so far",
                checked,
                total_primes,
                100.0 * checked as f64 / total_primes as f64,
                count
            );
        }
    }

    println!("\nFinal count: {}", count);
}
