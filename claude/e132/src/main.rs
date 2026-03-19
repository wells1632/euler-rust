fn mod_pow(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
    if modulus == 1 { return 0; }
    let mut result = 1u64;
    base %= modulus;
    while exp > 0 {
        if exp % 2 == 1 {
            result = (result as u128 * base as u128 % modulus as u128) as u64;
        }
        exp >>= 1;
        base = (base as u128 * base as u128 % modulus as u128) as u64;
    }
    result
}

fn is_prime(n: u64) -> bool {
    if n < 2 { return false; }
    if n == 2 || n == 3 || n == 5 { return true; }
    if n % 2 == 0 || n % 3 == 0 || n % 5 == 0 { return false; }
    let mut i = 7u64;
    while i * i <= n {
        if n % i == 0 { return false; }
        i += if i % 6 == 1 { 4 } else { 2 };
    }
    true
}

fn divides_repunit(p: u64, k: u64) -> bool {
    // p divides R(k) iff 10^k ≡ 1 (mod 9p) ... actually:
    // R(k) = (10^k - 1) / 9
    // p divides R(k) iff p divides (10^k - 1)/9
    // If p != 3: iff 10^k ≡ 1 (mod p)
    // p = 3 divides R(k) always (since R(k) = 111...1 and digit sum = k, div by 3 when 3|k)
    if p == 3 {
        return k % 3 == 0;
    }
    mod_pow(10, k, p) == 1
}

fn main() {
    let k = 1_000_000_000u64;
    let target = 40;
    let mut found = Vec::new();
    let mut sum = 0u64;

    // Skip p=2,5 as they don't divide any repunit (repunits are odd and not div by 5)
    // Start from 3
    let mut p = 3u64;
    while found.len() < target {
        if is_prime(p) && divides_repunit(p, k) {
            found.push(p);
            sum += p;
            println!("Found prime #{}: {}", found.len(), p);
        }
        p += if p == 2 { 1 } else { 2 };
    }

    println!("\nFirst {} prime factors of R(10^9):", target);
    println!("{:?}", found);
    println!("Sum: {}", sum);
}
