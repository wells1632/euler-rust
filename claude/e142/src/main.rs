use std::time::Instant;
use std::collections::HashSet;

fn main() {
    let start = Instant::now();
    println!("Searching for smallest x+y+z where x>y>z>0 and x+y, x-y, x+z, x-z, y+z, y-z are all perfect squares...\n");

    let limit: i64 = 1_000_000;

    // Precompute all perfect squares up to 2*limit in a HashSet for O(1) lookup
    let max_val = 2 * limit;
    let mut squares: HashSet<i64> = HashSet::new();
    let mut i = 1i64;
    while i * i <= max_val {
        squares.insert(i * i);
        i += 1;
    }
    println!("  Precomputed {} perfect squares up to {} (elapsed: {:.2?})\n", squares.len(), max_val, start.elapsed());

    let is_sq = |n: i64| -> bool { n > 0 && squares.contains(&n) };

    let mut best: Option<(i64, i64, i64)> = None;

    for x in 2..=limit {
        if x % 10_000 == 0 {
            println!("  Status: checking x = {} / {} (elapsed: {:.2?})", x, limit, start.elapsed());
        }

        if let Some((bx, by, bz)) = best {
            if x >= bx + by + bz {
                println!("  Early exit at x={} since x alone >= best sum {}", x, bx + by + bz);
                break;
            }
        }

        // x must be expressible as average of two squares: x+y and x-y are squares
        // so iterate only over valid y by finding square pairs that sum to 2x
        // i.e. a^2 + b^2 = 2x where y = (a^2 - b^2)/2, a > b, a^2 > x > b^2
        let mut valid_ys: Vec<i64> = Vec::new();
        let mut a = (x as f64).sqrt() as i64 + 1;
        loop {
            let a2 = a * a;
            let b2 = 2 * x - a2;
            if b2 <= 0 { break; }
            if is_sq(b2) {
                let y = (a2 - b2) / 2;
                if y > 0 && y < x && (a2 - b2) % 2 == 0 {
                    valid_ys.push(y);
                }
            }
            a += 1;
            if a2 > 2 * x { break; }
        }

        for y in valid_ys {
            if let Some((bx, by, bz)) = best {
                if x + y >= bx + by + bz {
                    continue;
                }
            }

            // Similarly find valid z values: x+z and x-z are squares, z < y
            let mut a = (x as f64).sqrt() as i64 + 1;
            loop {
                let a2 = a * a;
                let b2 = 2 * x - a2;
                if b2 <= 0 { break; }
                if is_sq(b2) {
                    let z = (a2 - b2) / 2;
                    if z > 0 && z < y && (a2 - b2) % 2 == 0 {
                        // Check remaining conditions
                        if is_sq(y + z) && is_sq(y - z) {
                            let sum = x + y + z;
                            if best.is_none() || sum < best.map(|(a,b,c)| a+b+c).unwrap() {
                                best = Some((x, y, z));
                                println!("  *** New best: x={}, y={}, z={}, sum={} (elapsed: {:.2?})", x, y, z, sum, start.elapsed());
                            }
                        }
                    }
                }
                a += 1;
                if a2 > 2 * x { break; }
            }
        }
    }

    let elapsed = start.elapsed();

    match best {
        Some((x, y, z)) => {
            println!("\n=== RESULT ===");
            println!("  x = {}", x);
            println!("  y = {}", y);
            println!("  z = {}", z);
            println!("  x+y = {} (sqrt: {})", x+y, ((x+y) as f64).sqrt() as i64);
            println!("  x-y = {} (sqrt: {})", x-y, ((x-y) as f64).sqrt() as i64);
            println!("  x+z = {} (sqrt: {})", x+z, ((x+z) as f64).sqrt() as i64);
            println!("  x-z = {} (sqrt: {})", x-z, ((x-z) as f64).sqrt() as i64);
            println!("  y+z = {} (sqrt: {})", y+z, ((y+z) as f64).sqrt() as i64);
            println!("  y-z = {} (sqrt: {})", y-z, ((y-z) as f64).sqrt() as i64);
            println!("  x+y+z = {}", x+y+z);
        }
        None => println!("No solution found within limit."),
    }

    println!("\nTotal time: {:.2?}", elapsed);
}
