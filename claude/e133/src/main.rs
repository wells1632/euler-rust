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

fn multiplicative_order(base: u64, p: u64) -> u64 {
    // Find the smallest k such that base^k ≡ 1 (mod p)
    // Order must divide p-1 (Fermat's little theorem)
    // So we find divisors of p-1 in increasing order
    let pm1 = p - 1;

    // Factorize p-1
    let mut factors = std::collections::HashSet::new();
    let mut n = pm1;
    let mut d = 2u64;
    while d * d <= n {
        if n % d == 0 {
            factors.insert(d);
            while n % d == 0 { n /= d; }
        }
        d += 1;
    }
    if n > 1 { factors.insert(n); }

    // Start with ord = p-1, then try dividing out prime factors
    // This finds the true minimal order
    let mut ord = pm1;
    for &f in &factors {
        while ord % f == 0 && mod_pow(base, ord / f, p) == 1 {
            ord /= f;
        }
    }
    ord
}

fn is_10power_smooth(mut n: u64) -> bool {
    // Returns true if n's only prime factors are 2 and 5
    while n % 2 == 0 { n /= 2; }
    while n % 5 == 0 { n /= 5; }
    n == 1
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

fn can_divide_repunit_power_of_10(p: u64) -> bool {
    if p == 2 || p == 5 { return false; }
    if p == 3 { return false; } // 3 divides 9 in denominator of R(k)=(10^k-1)/9
    
    // p divides R(10^n) = (10^(10^n) - 1)/9
    // If p != 3: p divides R(k) iff 10^k ≡ 1 (mod p)
    // So we need 10^(10^n) ≡ 1 (mod p) for some n
    // This means ord_p(10) must divide 10^n for some n
    // i.e. ord_p(10) must be {2,5}-smooth
    let ord = multiplicative_order(10, p);
    is_10power_smooth(ord)
}

fn main() {
    let limit = 100_000u64;
    let mut never_sum = 0u64;
    let mut never_count = 0u64;

    for p in 2..limit {
        if !is_prime(p) { continue; }
        if !can_divide_repunit_power_of_10(p) {
            never_count += 1;
            never_sum += p;
        }
    }

    // Sanity check: primes below 100 that CAN be factors
    println!("Primes below 100 that CAN divide R(10^n):");
    for p in 2..100u64 {
        if is_prime(p) && can_divide_repunit_power_of_10(p) {
            print!("{} ", p);
        }
    }
    println!();
    println!("\nPrimes below {} that NEVER divide R(10^n):", limit);
    println!("Count: {}", never_count);
    println!("Sum: {}", never_sum);
}
