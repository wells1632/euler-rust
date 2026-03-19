use std::collections::HashSet;

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 { a } else { gcd(b, a % b) }
}

fn main() {
    let limit: u64 = 100_000_000;
    let mut count = 0u64;

    // Generate all Pythagorean triples (a,b,c) with a<b<c and perimeter < limit
    // Primitive triples: a = m^2-n^2, b = 2mn, c = m^2+n^2
    // Perimeter = 2m^2 + 2mn = 2m(m+n)
    // For scaled triples: perimeter = k * 2m(m+n) < limit

    let mut m: u64 = 2;
    loop {
        // Minimum perimeter for this m is when n=1, k=1: 2m(m+1)
        if 2 * m * (m + 1) >= limit {
            break;
        }

        for n in 1..m {
            if (m - n) % 2 == 0 { continue; }   // m-n must be odd
            if gcd(m, n) != 1 { continue; }       // must be coprime

            let a0 = m * m - n * n;
            let b0 = 2 * m * n;
            let c0 = m * m + n * n;
            let perim0 = a0 + b0 + c0;            // = 2m(m+n)

            // Scale by k
            let mut k: u64 = 1;
            loop {
                let perim = k * perim0;
                if perim >= limit { break; }

                let a = k * a0;
                let b = k * b0;
                let c = k * c0;

                // Condition: |b - a| divides c
                let diff = if b > a { b - a } else { a - b };
                if diff == 0 || c % diff == 0 {
                    count += 1;
                }

                k += 1;
            }
        }
        m += 1;
    }

    println!("Count of valid Pythagorean triangles: {}", count);
}
