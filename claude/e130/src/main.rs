fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 { a } else { gcd(b, a % b) }
}

fn mod_pow(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
    if modulus == 1 { return 0; }
    let mut result = 1u64;
    base %= modulus;
    while exp > 0 {
        if exp & 1 == 1 {
            result = (result as u128 * base as u128 % modulus as u128) as u64;
        }
        exp >>= 1;
        base = (base as u128 * base as u128 % modulus as u128) as u64;
    }
    result
}

fn factorize(mut n: u64) -> Vec<u64> {
    let mut factors = Vec::new();
    let mut d = 2u64;
    while d * d <= n {
        if n % d == 0 {
            factors.push(d);
            while n % d == 0 { n /= d; }
        }
        d += 1;
    }
    if n > 1 { factors.push(n); }
    factors
}

fn is_prime(n: u64) -> bool {
    if n < 2 { return false; }
    if n == 2 || n == 3 { return true; }
    if n % 2 == 0 || n % 3 == 0 { return false; }
    let mut i = 5u64;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 { return false; }
        i += 6;
    }
    true
}

fn a_fast(n: u64) -> u64 {
    let mut temp = n;
    let mut pow3 = 0u32;
    while temp % 3 == 0 { temp /= 3; pow3 += 1; }

    let modulus = if pow3 == 0 {
        n
    } else {
        3u64.pow(pow3 + 2) * temp
    };

    if gcd(10, modulus) != 1 { return 0; }

    let phi = {
        let mut result = modulus;
        let mut t = modulus;
        let mut d = 2u64;
        while d * d <= t {
            if t % d == 0 {
                while t % d == 0 { t /= d; }
                result -= result / d;
            }
            d += 1;
        }
        if t > 1 { result -= result / t; }
        result
    };

    let factors = factorize(phi);
    let mut ord = phi;
    for p in &factors {
        while ord % p == 0 && mod_pow(10, ord / p, modulus) == 1 {
            ord /= p;
        }
    }
    ord
}

fn main() {
    use std::time::Instant;
    let start = Instant::now();

    // Verify the first five known composites
    println!("Verifying known composites:");
    for &n in &[91u64, 259, 451, 481, 703] {
        let an = a_fast(n);
        println!("  n={:4}, A(n)={:4}, (n-1)%A(n)={}, composite={}",
            n, an, (n-1) % an, !is_prime(n));
    }

    println!("\nSearching for first 25 composite values...");
    let mut composites: Vec<u64> = Vec::new();
    let mut n = 2u64;

    while composites.len() < 25 {
        // Must be composite, gcd(n,10)=1, and A(n) | n-1
        if !is_prime(n) && gcd(n, 10) == 1 {
            let an = a_fast(n);
            if an > 0 && (n - 1) % an == 0 {
                composites.push(n);
                println!("  Found {:2}/25: n={:10}, A(n)={:10}", 
                    composites.len(), n, an);
            }
        }
        n += 1;
    }

    let sum: u64 = composites.iter().sum();
    println!("\nFirst 25 composites: {:?}", composites);
    println!("Sum: {}", sum);
    println!("Time: {:.3}s", start.elapsed().as_secs_f64());
}
