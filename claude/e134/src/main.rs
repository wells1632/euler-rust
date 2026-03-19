fn mod_inverse(a: u64, m: u64) -> Option<u64> {
    let (mut old_r, mut r) = (a as i64, m as i64);
    let (mut old_s, mut s) = (1i64, 0i64);

    while r != 0 {
        let q = old_r / r;
        (old_r, r) = (r, old_r - q * r);
        (old_s, s) = (s, old_s - q * s);
    }

    if old_r != 1 {
        None
    } else {
        Some(((old_s % m as i64 + m as i64) % m as i64) as u64)
    }
}

fn sieve(limit: usize) -> Vec<usize> {
    let mut is_prime = vec![true; limit + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    let mut i = 2;
    while i * i <= limit {
        if is_prime[i] {
            let mut j = i * i;
            while j <= limit {
                is_prime[j] = false;
                j += i;
            }
        }
        i += 1;
    }
    (2..=limit).filter(|&x| is_prime[x]).collect()
}

fn num_digits(n: u64) -> u32 {
    let mut count = 0;
    let mut x = n;
    while x > 0 {
        count += 1;
        x /= 10;
    }
    count
}

fn main() {
    // Sieve beyond 1,000,000 to ensure we have the successor of every prime <= 1,000,000
    let primes = sieve(1_100_000);
    let mut total: u128 = 0;

    for window in primes.windows(2) {
        let p = window[0] as u64;
        let q = window[1] as u64;

        // p must be in range [5, 1_000_000]
        if p < 5 || p > 1_000_000 {
            continue;
        }

        let d = num_digits(p);
        let modulus = 10u64.pow(d);

        // Find smallest multiple of q ending in digits of p
        // q*k ≡ p (mod modulus) => k ≡ p * q_inv (mod modulus)
        if let Some(q_inv) = mod_inverse(q % modulus, modulus) {
            let k = (p % modulus * q_inv) % modulus;
            let k = if k == 0 { modulus } else { k };
            let n = q * k;
            total += n as u128;
        }
    }

    println!("Sum = {}", total);
}
