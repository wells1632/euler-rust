fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 { a } else { gcd(b, a % b) }
}

// Find least k such that n | R(k), by direct iteration
fn a(n: u64) -> u64 {
    let mut r = 0u64;
    let mut k = 0u64;
    loop {
        r = (r * 10 + 1) % n;
        k += 1;
        if r == 0 { return k; }
    }
}

fn main() {
    use std::time::Instant;
    let start = Instant::now();

    // Verify known values
    println!("A(7)  = {} (expected 6)",  a(7));
    println!("A(41) = {} (expected 5)",  a(41));

    // Verify threshold=10 case
    for n in (1u64..).filter(|&n| gcd(n, 10) == 1) {
        if a(n) > 10 {
            println!("First n where A(n) > 10: {} with A(n)={} (expected 17)", n, a(n));
            break;
        }
    }

    // Now find least n where A(n) > 1,000,000
    // But A(n) could be huge, so direct iteration per n is too slow.
    // We need the order approach but computing it correctly.
    // Key insight: A(n) = ord_{n/gcd(n,9)}(10) adjusted for factor of 9.
    // Actually let's just verify: for small n, does direct iteration match
    // our multiplicative order approach?

    // The issue: for n with gcd(n,9)>1, the order of 10 mod n != A(n)
    // e.g. n=9: R(k) mod 9 = k mod 9, so A(9)=9, but ord_9(10)=1 since 10≡1 mod 9
    println!("\nA(9) = {} (direct)", a(9));
    println!("A(27) = {} (direct)", a(27));
    println!("A(99) = {} (direct)", a(99));
    println!("A(999) = {} (direct)", a(999));

    // So for multiples of 9, A(n) != ord_n(10).
    // Since gcd(n,10)=1, n has no factors of 2 or 5.
    // Write n = 3^a * m where gcd(m,30)=1.
    // R(k) = (10^k-1)/9 = (10^k-1)/(3^2)
    // For 3^a | R(k): need 3^(a+2) | 10^k - 1... 
    // This gets complicated. Since the search space likely avoids
    // multiples of 3 for large orders, let's just use direct iteration
    // but cap it and use the order method for non-multiples of 3:

    fn gcd2(a: u64, b: u64) -> u64 {
        if b == 0 { a } else { gcd2(b, a % b) }
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

    // Compute A(n) efficiently:
    // R(k) = (10^k - 1)/9
    // n | R(k) iff 9n | 9*R(k) = 10^k-1 iff 10^k ≡ 1 mod (9n/gcd(9n,9n))
    // More carefully: let g = gcd(n, 9n/n) ... 
    // Simplest: n | (10^k-1)/9 
    // Multiply both sides by 9: 9n | 9*(10^k-1)/9 * gcd... 
    // Let's just say: find ord of 10 mod (n * 9 / gcd(n*9, 10^k-1))
    // 
    // Actually cleanest: n | R(k) iff n*9/gcd(n,9) | 10^k - 1
    // i.e. lcm(n,9) | 10^k - 1
    // i.e. ord_{lcm(n,9)}(10) | k
    // So A(n) = ord_{lcm(n,9)}(10) !
    // Verify: A(9): lcm(9,9)=9, ord_9(10): 10≡1 mod 9, ord=1? But A(9)=9...
    // Hmm that's wrong too. Let me just verify directly:
    // R(1)=1, R(2)=11, ..., R(9)=111111111=9*12345679, so A(9)=9 ✓
    // 10 mod 9 = 1, so 10^k mod 9 = 1 for all k, meaning 10^k-1 ≡ 0 mod 9 always
    // So lcm(9,9)=9 divides 10^k-1 for all k>=1, giving ord=1, not 9. Wrong.
    //
    // The real formula: n | (10^k-1)/9
    // Case 1: gcd(n,3)=1. Then 9 and n coprime, so n|(10^k-1)/9 iff 9n|10^k-1
    //   (since gcd(9,n)=1 means we can multiply). So A(n) = ord_{9n}(10)... 
    //   wait no: 9n | 10^k-1 means 10^k ≡ 1 mod 9n, so A(n) = ord_{9n}(10).
    //   But ord_{9n}(10) = lcm(ord_9(10), ord_n(10)) = lcm(1, ord_n(10)) = ord_n(10)
    //   since ord_9(10)=1. So for gcd(n,3)=1: A(n) = ord_n(10). ✓
    //   Verify A(7)=ord_7(10)=6 ✓, A(41)=ord_41(10)=5 ✓
    //
    // Case 2: 3|n. Write n=3^a * m, gcd(m,3)=1.
    //   Need 3^a * m | (10^k-1)/9 = (10^k-1)/3^2
    //   So 3^(a+2) * m | 10^k - 1
    //   i.e. 10^k ≡ 1 mod 3^(a+2) AND 10^k ≡ 1 mod m
    //   So A(n) = lcm(ord_{3^(a+2)}(10), ord_m(10))
    //   ord_3(10)=1, ord_9(10)=1, ord_27(10)=3, ord_81(10)=9...
    //   For a=1 (n div by 3 not 9): need 3^3=27 | 10^k-1, ord_27(10)=3
    //   For a=2 (n div by 9): need 3^4=81 | 10^k-1, ord_81(10)=9

    fn a_fast(n: u64) -> u64 {
        // Factor out powers of 3 from n
        let mut temp = n;
        let mut pow3 = 0u32;
        while temp % 3 == 0 { temp /= 3; pow3 += 1; }
        // temp = m, gcd(m,3)=1, n = 3^pow3 * m

        // Need 3^(pow3+2) | 10^k - 1 AND m | 10^k - 1
        // Combined: need 10^k ≡ 1 mod lcm(3^(pow3+2), m)
        // But gcd(3,m)=1 so lcm = 3^(pow3+2) * m

        let modulus = if pow3 == 0 {
            // gcd(n,3)=1: A(n) = ord_n(10)... 
            // but wait, we showed A(n)=ord_{9n}(10)=ord_n(10) for gcd(n,3)=1
            // Actually let's just use modulus = n directly since ord_9(10)=1
            n
        } else {
            // modulus = 3^(pow3+2) * m
            let pow3_plus2 = 3u64.pow(pow3 + 2);
            pow3_plus2 * temp
        };

        // Now find ord_{modulus}(10)
        if gcd2(10, modulus) != 1 { return 0; } // shouldn't happen

        // phi(modulus)
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

    // Verify a_fast matches direct for all n up to 10000
    println!("\nCross-checking a_fast vs direct for n up to 10000:");
    let mut mismatches = 0;
    for n in (1u64..=10000).filter(|&n| gcd2(n, 10) == 1) {
        let fast = a_fast(n);
        let direct = a(n);
        if fast != direct {
            println!("  MISMATCH n={}: fast={}, direct={}", n, fast, direct);
            mismatches += 1;
            if mismatches > 20 { println!("  ...stopping after 20 mismatches"); break; }
        }
    }
    if mismatches == 0 { println!("  All match!"); }

    // Now search for least n where A(n) > 1,000,000
    println!("\nSearching for least n where A(n) > 1,000,000...");
    let mut last_report = Instant::now();
    for n in (1u64..).filter(|&n| gcd2(n, 10) == 1) {
        let an = a_fast(n);
        if an > 1_000_000 {
            println!("Least n where A(n) > 1,000,000: {}", n);
            println!("A(n) = {}", an);
            println!("Total time: {:.3}s", start.elapsed().as_secs_f64());
            break;
        }
        if last_report.elapsed().as_secs_f64() >= 5.0 {
            println!("  Searching... n={} | elapsed: {:.1}s", n, start.elapsed().as_secs_f64());
            last_report = Instant::now();
        }
    }
}
