fn count_diagonal(m: usize, n: usize) -> u64 {
    let m2 = 2 * m as i64;
    let n2 = 2 * n as i64;
    let mut count = 0u64;
    for a in 1..=m2 {
        for b in 1..=n2 {
            let x_max = m2 - a - b;
            let y_min = b;
            let y_max = n2 - a;
            if x_max < 0 || y_min > y_max { continue; }
            // Count (x,y) with 0<=x<=x_max, y_min<=y<=y_max, x%2==y%2
            for x in 0..=x_max {
                let par = ((x % 2) + 2) % 2;
                let ys = if ((y_min % 2) + 2) % 2 == par { y_min } else { y_min + 1 };
                if ys > y_max { continue; }
                count += ((y_max - ys) / 2 + 1) as u64;
            }
        }
    }
    count
}

fn count_axis_aligned(m: usize, n: usize) -> u64 {
    let m = m as u64;
    let n = n as u64;
    (m * (m + 1) / 2) * (n * (n + 1) / 2)
}

fn count_total(m: usize, n: usize) -> u64 {
    count_axis_aligned(m, n) + count_diagonal(m, n)
}

fn main() {
    println!("Verifying:");
    for (m, n, exp) in [(1usize,1usize,1u64),(2,1,4),(3,1,8),(1,2,4),(2,2,18),(3,2,37)] {
        let r = count_total(m, n);
        let ok = if r == exp { "✓" } else { "✗" };
        println!("{}x{} | {} | expected {} {}", m, n, r, exp, ok);
    }

    let mut total = 0u64;
    for w in 1..=47usize {
        for h in 1..=43usize {
            total += count_total(w, h);
        }
    }
    println!("\nTotal for 47x43 and all smaller grids: {}", total);
}
