fn count_rectangles(m: i64, n: i64) -> i64 {
    (m * (m + 1) / 2) * (n * (n + 1) / 2)
}

fn main() {
    let target = 2_000_000;
    let mut best_diff = i64::MAX;
    let mut best_m = 0;
    let mut best_n = 0;
    let mut best_count = 0;
    
    // Search reasonable range
    // Since we want ~2 million rectangles, we can estimate the range
    // If m ≈ n, then roughly (m²/2)² ≈ 2,000,000, so m² ≈ 2828, m ≈ 53
    // Let's search a bit wider to be safe
    for m in 1..=100 {
        for n in 1..=100 {
            let count = count_rectangles(m, n);
            let diff = (count - target).abs();
            
            if diff < best_diff {
                best_diff = diff;
                best_m = m;
                best_n = n;
                best_count = count;
            }
        }
    }
    
    println!("Target: {} rectangles", target);
    println!("Best grid: {}×{}", best_m, best_n);
    println!("Rectangles: {}", best_count);
    println!("Difference: {}", best_diff);
    println!("Area: {}", best_m * best_n);
}
