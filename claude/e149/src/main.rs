fn kadane(seq: &[i64]) -> i64 {
    let mut best = seq[0];
    let mut current = seq[0];
    for &x in &seq[1..] {
        current = x.max(current + x);
        best = best.max(current);
    }
    best
}

fn main() {
    const N: usize = 2000;
    const MOD: i64 = 1_000_000;

    // Generate sequence (1-indexed, length N*N)
    let total = N * N;
    let mut s = vec![0i64; total + 1];

    for k in 1..=55 {
        let k = k as i64;
        s[k as usize] = (100003 - 200003 * k + 300007 * k * k * k).rem_euclid(MOD) - 500000;
    }

    for k in 56..=total {
        s[k] = (s[k - 24] + s[k - 55] + MOD).rem_euclid(MOD) - 500000;
    }

    // Fill grid (0-indexed)
    let grid: Vec<Vec<i64>> = (0..N)
        .map(|i| (0..N).map(|j| s[i * N + j + 1]).collect())
        .collect();

    let mut max_sum = i64::MIN;

    // Horizontal
    for i in 0..N {
        max_sum = max_sum.max(kadane(&grid[i]));
    }

    // Vertical
    for j in 0..N {
        let col: Vec<i64> = (0..N).map(|i| grid[i][j]).collect();
        max_sum = max_sum.max(kadane(&col));
    }

    // Diagonal (top-left to bottom-right)
    for start in 0..N {
        let diag: Vec<i64> = (0..N - start).map(|i| grid[i][start + i]).collect();
        max_sum = max_sum.max(kadane(&diag));
        if start != 0 {
            let diag: Vec<i64> = (0..N - start).map(|j| grid[start + j][j]).collect();
            max_sum = max_sum.max(kadane(&diag));
        }
    }

    // Anti-diagonal (top-right to bottom-left)
    for start in 0..N {
        let anti: Vec<i64> = (0..=start).map(|i| grid[i][start - i]).collect();
        max_sum = max_sum.max(kadane(&anti));
        if start != 0 {
            let anti: Vec<i64> = (0..N - start).map(|j| grid[start + j][N - 1 - j]).collect();
            max_sum = max_sum.max(kadane(&anti));
        }
    }

    println!("{}", max_sum);
}
