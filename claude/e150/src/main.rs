fn main() {
    let n: usize = 1000;
    let total = n * (n + 1) / 2;

    let mut s = vec![0i64; total + 1];
    let mut t: u64 = 0;
    for k in 1..=total {
        t = (615949u64 * t + 797807u64) % (1u64 << 20);
        s[k] = t as i64 - (1i64 << 19);
    }

    println!("s(1)={}, s(2)={}, s(3)={}", s[1], s[2], s[3]);

    let mut prefix: Vec<Vec<i64>> = vec![vec![]; n + 1];
    for r in 1..=n {
        let mut p = vec![0i64; r + 1];
        for c in 0..r {
            let idx = r * (r - 1) / 2 + c + 1;
            p[c + 1] = p[c] + s[idx];
        }
        prefix[r] = p;
    }

    let mut min_sum = i64::MAX;

    for r in 1..=n {
        for c in 0..r {
            let mut sub_sum: i64 = 0;
            for d in 0..(n + 1 - r) {
                let row = r + d;
                sub_sum += prefix[row][c + d + 1] - prefix[row][c];
                if sub_sum < min_sum {
                    min_sum = sub_sum;
                }
            }
        }
    }

    println!("Minimum sub-triangle sum: {}", min_sum);
}
