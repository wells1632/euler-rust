// Cargo.toml:
// [dependencies]
// rayon = "1.10"

use rayon::prelude::*;

fn digit_sum(mut n: u64, base: u64) -> u64 {
    let mut s = 0u64;
    while n > 0 { s += n % base; n /= base; }
    s
}

fn base_digits_lsb(mut n: u64, base: u64) -> Vec<u64> {
    if n == 0 { return vec![0]; }
    let mut d = Vec::new();
    while n > 0 { d.push(n % base); n /= base; }
    d
}

fn digit_sum_distribution(n: u64, p: u64) -> std::collections::HashMap<u64, u128> {
    let digits = base_digits_lsb(n, p);
    let max_carry = 3usize;
    let mut dp: Vec<std::collections::HashMap<u64, u128>> =
        (0..=max_carry).map(|_| std::collections::HashMap::new()).collect();
    dp[0].insert(0, 1);
    for &d in &digits {
        let mut new_dp: Vec<std::collections::HashMap<u64, u128>> =
            (0..=max_carry).map(|_| std::collections::HashMap::new()).collect();
        for carry_in in 0..=max_carry {
            for (&dsum, &count) in &dp[carry_in] {
                for a in 0..p {
                    for b in 0..p {
                        for c in 0..p {
                            let sum = a + b + c + carry_in as u64;
                            if sum % p == d {
                                let co = (sum / p) as usize;
                                if co <= max_carry {
                                    *new_dp[co].entry(dsum + a + b + c).or_insert(0) += count;
                                }
                            }
                        }
                    }
                }
            }
        }
        dp = new_dp;
    }
    dp[0].clone()
}

fn count_ge(dist: &std::collections::HashMap<u64, u128>, threshold: u64) -> u128 {
    dist.iter().filter(|&(&k, _)| k >= threshold).map(|(_, &v)| v).sum()
}

fn main() {
    let n: usize = 200_000;
    let n64 = n as u64;

    let s2n = digit_sum(n64, 2);
    let s5n = digit_sum(n64, 5);
    let t2 = (s2n + 12) as u8;
    let t5 = (s5n + 48) as u8;

    println!("n={n}, s2={s2n}, s5={s5n}, t2={t2}, t5={t5}");

    // Single-prime counts for reference
    let dist2 = digit_sum_distribution(n64, 2);
    let dist5 = digit_sum_distribution(n64, 5);
    let c2 = count_ge(&dist2, t2 as u64);
    let c5 = count_ge(&dist5, t5 as u64);
    let total = (n as u128 + 1) * (n as u128 + 2) / 2;
    println!("c2={c2}, c5={c5}, total={total}");
    println!("Bounds: [{}, {}]",
             if c2 + c5 > total { c2 + c5 - total } else { 0 },
             c2.min(c5));

    // Precompute digit sums as u8 arrays for cache efficiency
    println!("Precomputing digit sum tables...");
    let ds2: Vec<u8> = (0..=n).map(|v| digit_sum(v as u64, 2) as u8).collect();
    let ds5: Vec<u8> = (0..=n).map(|v| digit_sum(v as u64, 5) as u8).collect();

    println!("Running parallel O(n^2) computation...");
    let start = std::time::Instant::now();

    // KEY OPTIMIZATION: For each a, we precompute how much "budget" remains
    // for (b,c), then do an optimized inner scan.
    //
    // FURTHER OPTIMIZATION: For fixed a with da2 and da5,
    // we need ds2[b] + ds2[c] >= t2 - da2 AND ds5[b] + ds5[c] >= t5 - da5
    // where c = (n-a) - b.
    //
    // Inner loop optimization: Build a prefix count array for the inner sum.
    // For each m = n-a:
    //   count_valid[b] = [ds2[b] + ds2[m-b] >= r2 AND ds5[b] + ds5[m-b] >= r5]
    //   sum this over b = 0..m.
    //
    // This is still O(m) per a. But the inner loop body is extremely tight:
    // two array lookups + two additions + two comparisons + branch.
    // With SIMD and cache-friendly access, this can do ~10^9 iterations/sec.
    // Total: n^2/2 = 2*10^10 iterations / 10^9/sec = ~20 seconds with good SIMD.
    //
    // Rayon parallelizes the outer loop perfectly.

    let answer: u128 = (0..=n)
        .into_par_iter()
        .map(|a| {
            let da2 = ds2[a];
            let da5 = ds5[a];
            // saturating subtraction for thresholds
            let r2 = t2.saturating_sub(da2);
            let r5 = t5.saturating_sub(da5);
            let m = n - a;
            let mut local = 0u64;
            // OPTIMIZATION: early exit if even maximum possible ds2+ds5 is insufficient.
            // Max ds2 for any value in [0,n]: 17. Max ds5: 28.
            // So max ds2(b)+ds2(c) = 34, max ds5(b)+ds5(c) = 56.
            if r2 > 34 || r5 > 56 {
                return 0u128;
            }
            for b in 0..=m {
                let c = m - b;
                // SIMD-friendly: all additions and comparisons on u8
                if ds2[b].wrapping_add(ds2[c]) >= r2
                    && ds5[b].wrapping_add(ds5[c]) >= r5
                {
                    local += 1;
                }
            }
            local as u128
        })
        .sum();

    let elapsed = start.elapsed();
    println!("Computation took: {:.2?}", elapsed);
    println!("\nAnswer: {answer} coefficients of (x+y+z)^200000 are divisible by 10^12");
}
