use std::time::Instant;

fn pow_mod(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
    if modulus == 1 { return 0; }
    let mut result = 1u64;
    base %= modulus;
    while exp > 0 {
        if exp & 1 == 1 { result = result * base % modulus; }
        exp >>= 1;
        base = base * base % modulus;
    }
    result
}

fn extended_gcd(a: i64, b: i64) -> (i64, i64) {
    if b == 0 { return (1, 0); }
    let (x, y) = extended_gcd(b, a % b);
    (y, x - (a / b) * y)
}

fn mod_inv(a: u64, m: u64) -> u64 {
    let (x, _) = extended_gcd(a as i64, m as i64);
    ((x % m as i64 + m as i64) as u64) % m
}

fn legendre(n: u64, p: u64) -> u64 {
    let mut count = 0u64;
    let mut pk = p;
    while pk <= n {
        count += n / pk;
        match pk.checked_mul(p) {
            Some(v) => pk = v,
            None => break,
        }
    }
    count
}

fn compute_wilson(p: u64, pa: u64) -> u64 {
    let mut prod = 1u64;
    for i in 1..pa { if i % p != 0 { prod = prod * i % pa; } }
    prod
}

fn fact_p_free(n: u64, p: u64, pa: u64) -> u64 {
    if n == 0 { return 1; }
    let wilson = compute_wilson(p, pa);
    let periods = n / pa;
    let period_contribution = pow_mod(wilson, periods, pa);
    let rem = n % pa;
    let mut partial = 1u64;
    for i in 1..=rem {
        if i % p != 0 { partial = partial * (i % pa) % pa; }
    }
    let rec = fact_p_free(n / p, p, pa);
    period_contribution * partial % pa * rec % pa
}

fn f(n: u64) -> u64 {
    let mod2: u64 = 32;
    let mod5: u64 = 3125;
    let v2 = legendre(n, 2);
    let v5 = legendre(n, 5);
    let excess2 = v2 - v5;
    let f_no2 = fact_p_free(n, 2, mod2);
    let x_mod32 = if excess2 >= 5 { 0 } else {
        pow_mod(2, excess2, mod2)
            * (f_no2 * mod_inv(pow_mod(5, v5, mod2), mod2) % mod2) % mod2
    };
    let f_no5 = fact_p_free(n, 5, mod5);
    let x_mod3125 = f_no5 * mod_inv(pow_mod(2, v5, mod5), mod5) % mod5;
    let inv32_mod3125 = mod_inv(32, mod5);
    let t = ((x_mod3125 + mod5 - x_mod32 % mod5) % mod5) * inv32_mod3125 % mod5;
    x_mod32 + 32 * t
}

fn main() {
    let n = 1_000_000_000_000u64;
    let start = Instant::now();
    let result = f(n);
    let elapsed = start.elapsed();
    println!("f({}) = {:05}", n, result);
    println!("computed in {}.{:09} seconds", elapsed.as_secs(), elapsed.subsec_nanos());
}
