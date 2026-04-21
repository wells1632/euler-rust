use std::collections::HashSet;

// Represent capacitance as a fraction (numerator, denominator) in reduced form
fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 { a } else { gcd(b, a % b) }
}

fn reduce(n: u64, d: u64) -> (u64, u64) {
    let g = gcd(n, d);
    (n / g, d / g)
}

// Get all distinct capacitance values (as reduced fractions) using exactly k capacitors
fn exact_values(k: usize, memo: &mut Vec<Option<HashSet<(u64, u64)>>>) -> HashSet<(u64, u64)> {
    if let Some(ref s) = memo[k] {
        return s.clone();
    }

    let mut results = HashSet::new();

    if k == 1 {
        results.insert((1, 1)); // C = 1
        memo[k] = Some(results.clone());
        return results;
    }

    // Split k capacitors into two non-empty groups: left (j) and right (k-j)
    for j in 1..k {
        let left = exact_values(j, memo);
        let right = exact_values(k - j, memo);

        for &(n1, d1) in &left {
            for &(n2, d2) in &right {
                // Parallel: C = n1/d1 + n2/d2 = (n1*d2 + n2*d1) / (d1*d2)
                let pn = n1 * d2 + n2 * d1;
                let pd = d1 * d2;
                results.insert(reduce(pn, pd));

                // Series: 1/C = d1/n1 + d2/n2 = (d1*n2 + d2*n1) / (n1*n2)
                // So C = (n1*n2) / (d1*n2 + d2*n1)
                let sn = n1 * n2;
                let sd = d1 * n2 + d2 * n1;
                results.insert(reduce(sn, sd));
            }
        }
    }

    memo[k] = Some(results.clone());
    results
}

fn main() {
    let max_n = 18;
    let mut memo: Vec<Option<HashSet<(u64, u64)>>> = vec![None; max_n + 1];

    let mut all_values: HashSet<(u64, u64)> = HashSet::new();

    for k in 1..=max_n {
        let vals = exact_values(k, &mut memo);
        all_values.extend(vals);
        println!("D({}) = {}", k, all_values.len());
    }
}
