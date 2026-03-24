use std::collections::HashSet;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    println!("Searching for Torricelli triangles with p+q+r <= 120000...\n");

    // At the Torricelli point, all angles are 120 degrees, so:
    //   c^2 = p^2 + q^2 + p*q
    //   a^2 = q^2 + r^2 + q*r
    //   b^2 = p^2 + r^2 + p*r
    //
    // Strategy: iterate over integer triples (p, q, r) with p >= q >= r >= 1,
    // compute a, b, c, check if they are integers and form a valid triangle
    // with all angles < 120 degrees.

    let max_pqr: u64 = 120_000;

    // p+q+r <= 120000, and p >= q >= r >= 1
    // so p <= 120000, q <= p, r <= q

    let mut distinct_sums: HashSet<u64> = HashSet::new();
    let mut count = 0u64;

    // We need c^2 = p^2+q^2+pq, a^2 = q^2+r^2+qr, b^2 = p^2+r^2+pr
    // to all be perfect squares.

    // Precompute: for a given pair (m,n), m^2+n^2+m*n is a perfect square iff...
    // We just check directly.

    let is_perfect_square = |n: u64| -> Option<u64> {
        if n == 0 { return Some(0); }
        let s = (n as f64).sqrt() as u64;
        for candidate in s.saturating_sub(2)..=s+2 {
            if candidate * candidate == n {
                return Some(candidate);
            }
        }
        None
    };

    // Triangle validity: all angles < 120 degrees
    // angle A < 120 => cos A > -1/2 => b^2+c^2-a^2 > -bc => 2(b^2+c^2-a^2) > -2bc
    // i.e. b^2 + c^2 + bc > a^2  (and same for B, C)
    let valid_triangle = |a: u64, b: u64, c: u64| -> bool {
        // Triangle inequality
        if a + b <= c || a + c <= b || b + c <= a { return false; }
        // All angles < 120 degrees:
        // For angle opposite side a: b^2 + c^2 + b*c > a^2
        let a2 = a*a; let b2 = b*b; let c2 = c*c;
        let bc = b*c; let ac = a*c; let ab = a*b;
        (b2 + c2 + bc > a2) && (a2 + c2 + ac > b2) && (a2 + b2 + ab > c2)
    };

    // Iterate p >= q >= r >= 1, p+q+r <= max_pqr
    for r in 1..=max_pqr / 3 {
        if r % 1000 == 0 {
            println!("  Status: r={}, distinct sums found so far: {}, elapsed: {:.2?}",
                r, distinct_sums.len(), start.elapsed());
        }

        for q in r..=(max_pqr - r) / 2 {
            // Check q^2 + r^2 + q*r early — need a^2 = q^2+r^2+qr to be perfect square
            // Actually a corresponds to the side opposite the vertex A where BT=q, CT=r
            // Let's compute all three only when each step passes

            let qr_val = q*q + r*r + q*r;
            let a_opt = is_perfect_square(qr_val);
            if a_opt.is_none() { continue; }
            let a = a_opt.unwrap();

            let p_max = max_pqr - q - r;

            for p in q..=p_max {
                let pq_val = p*p + q*q + p*q;
                let c_opt = is_perfect_square(pq_val);
                if c_opt.is_none() { continue; }
                let c = c_opt.unwrap();

                let pr_val = p*p + r*r + p*r;
                let b_opt = is_perfect_square(pr_val);
                if b_opt.is_none() { continue; }
                let b = b_opt.unwrap();

                // All three sides are integers — check triangle validity
                if !valid_triangle(a, b, c) { continue; }

                let sum = p + q + r;
                count += 1;

                let is_new = distinct_sums.insert(sum);
                if is_new {
                    println!("  *** New distinct sum: p+q+r={} (p={},q={},r={}, a={},b={},c={}) elapsed: {:.2?}",
                        sum, p, q, r, a, b, c, start.elapsed());
                }
            }
        }
    }

    println!("\n=== RESULT ===");
    println!("  Total Torricelli triangles found: {}", count);
    println!("  Distinct values of p+q+r: {}", distinct_sums.len());

    let mut sorted_sums: Vec<u64> = distinct_sums.into_iter().collect();
    sorted_sums.sort();

    println!("  Sum of all distinct p+q+r values: {}", sorted_sums.iter().sum::<u64>());
    println!("  First few sums: {:?}", &sorted_sums[..sorted_sums.len().min(10)]);
    println!("\nTotal time: {:.2?}", start.elapsed());
}
