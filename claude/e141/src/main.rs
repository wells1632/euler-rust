use std::collections::HashSet;

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 { a } else { gcd(b, a % b) }
}

fn isqrt(n: u64) -> u64 {
    if n == 0 { return 0; }
    let mut s = (n as f64).sqrt() as u64;
    while s > 0 && s * s > n { s -= 1; }
    while (s + 1).checked_mul(s + 1).map_or(false, |v| v <= n) { s += 1; }
    s
}

fn is_perfect_square(n: u64) -> bool {
    let s = isqrt(n);
    s * s == n
}

/// Find the fundamental solution (x, y) to x^2 - D*y^2 = 1 using continued fractions.
/// Returns None if D is a perfect square.
fn pell_fundamental(d: u64) -> Option<(u128, u128)> {
    let sq = isqrt(d);
    if sq * sq == d { return None; }

    let mut m = 0u128;
    let mut dd = 1u128;
    let mut a = sq as u128;
    let a0 = a;

    let mut p_prev = 1u128;
    let mut p_curr = a0;
    let mut q_prev = 0u128;
    let mut q_curr = 1u128;

    loop {
        m = dd * a - m;
        dd = (d as u128 - m * m) / dd;
        a = (a0 + m) / dd;

        let p_next = a * p_curr + p_prev;
        let q_next = a * q_curr + q_prev;

        if p_next * p_next - d as u128 * q_next * q_next == 1 {
            return Some((p_next, q_next));
        }

        p_prev = p_curr; p_curr = p_next;
        q_prev = q_curr; q_curr = q_next;
    }
}

/// Given a fundamental solution (x1,y1) to x^2 - D*y^2 = 1,
/// and a base solution (x0,y0) to x^2 - D*y^2 = N,
/// generate all solutions with y > 0 and y <= y_max.
fn generate_pell_solutions(
    x0: i128, y0: i128,
    x1: i128, y1: i128,
    d: i128,
    y_max: i128,
    solutions: &mut Vec<(i128, i128)>,
) {
    let mut x = x0;
    let mut y = y0;
    // Normalize to positive y
    if y < 0 { x = -x; y = -y; }
    if y == 0 { 
        // trivial solution, skip
        return; 
    }
    while y <= y_max && y > 0 {
        if x > 0 {
            solutions.push((x, y));
        }
        let xn = x1 * x + d * y1 * y;
        let yn = y1 * x + x1 * y;
        x = xn;
        y = yn;
    }
}

/// Find all base solutions to x^2 - D*y^2 = N with 1 <= y <= y_fund * sqrt(|N|)
/// using a brute force search (this is valid by theory).
fn base_solutions_pell(d: u128, n_rhs: u128, x1: u128, y1: u128) -> Vec<(i128, i128)> {
    // For x^2 - D*y^2 = N > 0, base solutions have y in [0, y_fund * sqrt(N) / ??? ]
    // Theory: |y| <= y1 * sqrt(N / (x1 - 1)) ... approximate.
    // Simpler: search y from 0 to y1 * isqrt(n_rhs) + y1
    let y_bound = y1 * (isqrt(n_rhs as u64) as u128 + 1) + y1;
    let mut sols = vec![];
    for y in 0..=y_bound {
        let x2 = n_rhs + d * y * y;
        let x = isqrt(x2 as u64) as u128; // careful with large numbers
        // need to handle u128 sqrt
        let x_val = {
            let mut s = (x2 as f64).sqrt() as u128;
            while s > 0 && s * s > x2 { s -= 1; }
            while (s+1)*(s+1) <= x2 { s += 1; }
            s
        };
        if x_val * x_val == x2 {
            sols.push((x_val as i128, y as i128));
            if y > 0 {
                sols.push((x_val as i128, -(y as i128)));
            }
        }
    }
    sols
}

fn main() {
    let limit: u64 = 1_000_000_000_000u64;
    let mut results: HashSet<u64> = HashSet::new();

    let max_p = {
        // p^3 <= limit (for a=1, s=1: n ~ p^3)
        let mut p = 1u64;
        while (p+1)*(p+1)*(p+1) < limit { p += 1; }
        p
    };

    println!("max_p = {}", max_p);
    println!("Starting search over {} candidate p values...", max_p);

    let direct_threshold: u64 = 2_000_000;

    for p in 2..=max_p {
        if p % 1000 == 0 {
            println!("  p = {} / {}, solutions so far: {}", p, max_p, results.len());
        }

        for s in 1..p {
            if gcd(p, s) != 1 { continue; }

            // n = a*s*(a*p^3 + s), need n < limit
            // a^2 * s * p^3 < limit => max_a ~ sqrt(limit / (s*p^3))
            let sp3 = match s.checked_mul(p).and_then(|v| v.checked_mul(p)).and_then(|v| v.checked_mul(p)) {
                Some(v) => v,
                None => continue,
            };
            if sp3 >= limit { break; }

            let max_a = isqrt(limit / sp3) + 2;

            if max_a <= direct_threshold {
                // Direct search
                for a in 1..=max_a {
                    let ap3 = match a.checked_mul(p).and_then(|v| v.checked_mul(p)).and_then(|v| v.checked_mul(p)) {
                        Some(v) => v, None => break,
                    };
                    let inner = match ap3.checked_add(s) { Some(v) => v, None => break };
                    let n = match (a * s).checked_mul(inner) { Some(v) => v, None => break };
                    if n >= limit { break; }
                    if is_perfect_square(n) {
                        results.insert(n);
                    }
                }
            } else {
                // Pell equation approach
                // X^2 - D*Y^2 = s^4, D = 4*s*p^3
                // X = 2*s*p^3*a + s^2, Y = sqrt(n) = k
                // max Y: k^2 < limit => Y < sqrt(limit) ~ 10^6
                let d_pell = 4 * sp3; // 4*s*p^3
                let n_rhs = s * s * s * s; // s^4

                let y_max = isqrt(limit - 1);

                if let Some((x1, y1)) = pell_fundamental(d_pell) {
                    // Find base solutions to X^2 - d_pell * Y^2 = n_rhs
                    let bases = base_solutions_pell(d_pell as u128, n_rhs as u128, x1, y1);

                    for (x0, y0) in bases {
                        let mut x = x0;
                        let mut y = y0;
                        if y < 0 { x = -x; y = -y; }
                        let x1i = x1 as i128;
                        let y1i = y1 as i128;
                        let di = d_pell as i128;

                        while y > 0 && y <= y_max as i128 {
                            if x > 0 {
                                // Recover a: X = 2*s*p^3*a + s^2 => a = (X - s^2) / (2*s*p^3)
                                let x_u = x as u64;
                                let s2 = s * s;
                                let denom = 2 * sp3;
                                if x_u > s2 && (x_u - s2) % denom == 0 {
                                    let a = (x_u - s2) / denom;
                                    if a >= 1 {
                                        let n = (y as u64) * (y as u64);
                                        if n < limit && is_perfect_square(n) {
                                            // double-check
                                            results.insert(n);
                                        }
                                    }
                                }
                            }
                            let xn = x1i * x + di * y1i * y;
                            let yn = y1i * x + x1i * y;
                            x = xn;
                            y = yn;
                        }
                    }
                }
            }
        }
    }

    let mut sorted: Vec<u64> = results.into_iter().collect();
    sorted.sort();
    println!("\nProgressive perfect squares below 10^12:");
    for &n in &sorted {
        println!("  {}", n);
    }
    let sum: u64 = sorted.iter().sum();
    println!("\nSum: {}", sum);
    println!("Count: {}", sorted.len());
}
