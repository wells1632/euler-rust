use std::time::Instant;
use rayon::prelude::*;
use std::collections::HashSet;

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 { a } else { gcd(b, a % b) }
}

fn reduce(n: u64, d: u64) -> (u64, u64) {
    let g = gcd(n, d);
    (n / g, d / g)
}

fn exact_values(k: usize, memo: &mut Vec<Option<HashSet<(u64, u64)>>>) -> HashSet<(u64, u64)> {
    if let Some(ref s) = memo[k] {
        return s.clone();
    }

    if k == 1 {
        let mut s = HashSet::new();
        s.insert((1u64, 1u64));
        memo[k] = Some(s.clone());
        return s;
    }

    // Pre-compute all dependencies first to avoid borrow conflicts
    for j in 1..k {
        if memo[j].is_none() { exact_values(j, memo); }
        if memo[k - j].is_none() { exact_values(k - j, memo); }
    }

    // Collect all (left, right) pair slices without holding a memo borrow
    let pairs: Vec<(Vec<(u64, u64)>, Vec<(u64, u64)>)> = (1..k)
        .map(|j| {
            let left  = memo[j].as_ref().unwrap().iter().copied().collect::<Vec<_>>();
            let right = memo[k - j].as_ref().unwrap().iter().copied().collect::<Vec<_>>();
            (left, right)
        })
        .collect();

    // Parallelise over the j splits
    let results: HashSet<(u64, u64)> = pairs
        .par_iter()
        .flat_map(|(left, right)| {
            let mut local = Vec::new();
            for &(n1, d1) in left {
                for &(n2, d2) in right {
                    let pn = n1 * d2 + n2 * d1;
                    let pd = d1 * d2;
                    local.push(reduce(pn, pd));

                    let sn = n1 * n2;
                    let sd = d1 * n2 + d2 * n1;
                    local.push(reduce(sn, sd));
                }
            }
            local
        })
        .collect();

    memo[k] = Some(results.clone());
    results
}

fn main() {
    let start = Instant::now();
    let max_n = 18;
    let mut memo: Vec<Option<HashSet<(u64, u64)>>> = vec![None; max_n + 1];
    let mut all_values: HashSet<(u64, u64)> = HashSet::new();

    for k in 1..=max_n {
        let vals = exact_values(k, &mut memo);
        all_values.extend(vals);
        println!("D({}) = {}", k, all_values.len());
    }

    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);
}
