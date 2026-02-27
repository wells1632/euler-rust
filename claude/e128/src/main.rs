use std::time::Instant;

fn is_prime(n: u64) -> bool {
    if n < 2 { return false; }
    if n == 2 { return true; }
    if n % 2 == 0 { return false; }
    let mut i = 3u64;
    while i * i <= n {
        if n % i == 0 { return false; }
        i += 2;
    }
    true
}

fn ring_start(k: u64) -> u64 {
    if k == 0 { return 1; }
    3 * k * (k - 1) + 2
}

fn main() {
    let total_start = Instant::now();

    fn neighbors_brute(n: u64) -> Vec<u64> {
        let ring_of = |n: u64| -> u64 {
            if n == 1 { return 0; }
            ((-3.0 + (9.0 + 12.0 * (n as f64 - 1.0)).sqrt()) / 6.0).ceil() as u64
        };
        if n == 1 { return vec![2,3,4,5,6,7]; }
        let k = ring_of(n);
        let start = ring_start(k);
        let pos = n - start;
        let ring_len = 6 * k;
        let side = pos / k;
        let side_pos = pos % k;
        let mut nb = vec![];
        nb.push(if pos == 0 { start + ring_len - 1 } else { n - 1 });
        nb.push(if pos == ring_len - 1 { start } else { n + 1 });
        if k == 1 {
            nb.push(1);
        } else {
            let inner_start = ring_start(k - 1);
            let inner_len = 6 * (k - 1);
            if side_pos == 0 {
                nb.push(inner_start + (side * (k-1)) % inner_len);
            } else {
                let ip = side * (k-1) + side_pos - 1;
                nb.push(inner_start + ip);
                nb.push(inner_start + (ip + 1) % inner_len);
            }
        }
        let outer_start = ring_start(k + 1);
        let outer_len = 6 * (k + 1);
        if side_pos == 0 {
            let p = (side * (k+1)) % outer_len;
            nb.push(outer_start + (p + outer_len - 1) % outer_len);
            nb.push(outer_start + p);
            nb.push(outer_start + (p + 1) % outer_len);
        } else {
            let op = side * (k+1) + side_pos;
            nb.push(outer_start + op % outer_len);
            nb.push(outer_start + (op + 1) % outer_len);
        }
        nb
    }

    let pd_brute = |n: u64| -> u64 {
        neighbors_brute(n).iter()
            .filter(|&&nb| nb != 0 && is_prime(n.abs_diff(nb)))
            .count() as u64
    };

    let mut count = 0u64;
    let mut last_report_time = Instant::now();

    for n in 1..=7u64 {
        if pd_brute(n) == 3 {
            count += 1;
            if count == 10 { println!("10th tile with PD=3: {} (expected 271)", n); }
            if count == 2000 {
                println!("2000th tile with PD=3: {} | total time: {:.3}s", n, total_start.elapsed().as_secs_f64());
                return;
            }
        }
    }

    'outer: for k in 2u64.. {
        let start = ring_start(k);
        let ring_len = 6 * k;

        for s in 0..6u64 {
            let q = 6 * k + s;
            let corner_tile = start + s * k;

            let corner_pd = if s == 0 {
                is_prime(q - 1) as u64
                + is_prime(2 * q + 5) as u64
                + is_prime(q + 1) as u64
            } else {
                is_prime(q - 5) as u64
                + is_prime(q - 1) as u64
                + is_prime(q + 1) as u64
            };

            if corner_pd == 3 {
                count += 1;
                if count % 200 == 0 {
                    let elapsed = total_start.elapsed().as_secs_f64();
                    let interval = last_report_time.elapsed().as_secs_f64();
                    println!("  Hit {:4}/2000 at tile {:16} (ring {:6}, side {}, corner) | elapsed: {:.3}s (+{:.3}s)",
                        count, corner_tile, k, s, elapsed, interval);
                    last_report_time = Instant::now();
                }
                if count == 10 { println!("10th tile with PD=3: {} (expected 271)", corner_tile); }
                if count == 2000 {
                    println!("2000th tile with PD=3: {} | total time: {:.3}s", corner_tile, total_start.elapsed().as_secs_f64());
                    break 'outer;
                }
            }

            let nc_pd = is_prime(q - 5) as u64 + is_prime(q + 1) as u64;
            let nc_end = if s == 5 { k - 2 } else { k - 1 };

            if nc_pd == 3 {
                for sp in 1..=nc_end {
                    let tile = start + s * k + sp;
                    count += 1;
                    if count % 200 == 0 {
                        let elapsed = total_start.elapsed().as_secs_f64();
                        let interval = last_report_time.elapsed().as_secs_f64();
                        println!("  Hit {:4}/2000 at tile {:16} (ring {:6}, side {}, sp={:6}) | elapsed: {:.3}s (+{:.3}s)",
                            count, tile, k, s, sp, elapsed, interval);
                        last_report_time = Instant::now();
                    }
                    if count == 10 { println!("10th tile with PD=3: {} (expected 271)", tile); }
                    if count == 2000 {
                        println!("2000th tile with PD=3: {} | total time: {:.3}s", tile, total_start.elapsed().as_secs_f64());
                        break 'outer;
                    }
                }
            }

            if s == 5 {
                let last_tile = start + ring_len - 1;
                if pd_brute(last_tile) == 3 {
                    count += 1;
                    if count % 200 == 0 {
                        let elapsed = total_start.elapsed().as_secs_f64();
                        let interval = last_report_time.elapsed().as_secs_f64();
                        println!("  Hit {:4}/2000 at tile {:16} (ring {:6}, last tile)   | elapsed: {:.3}s (+{:.3}s)",
                            count, last_tile, k, elapsed, interval);
                        last_report_time = Instant::now();
                    }
                    if count == 10 { println!("10th tile with PD=3: {} (expected 271)", last_tile); }
                    if count == 2000 {
                        println!("2000th tile with PD=3: {} | total time: {:.3}s", last_tile, total_start.elapsed().as_secs_f64());
                        break 'outer;
                    }
                }
            }
        }
    }
}
