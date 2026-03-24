use std::time::Instant;
use rayon::prelude::*;

fn reverse_number(n: u64) -> Option<u64> {
    if n % 10 == 0 {
        return None;
    }
    let mut reversed = 0u64;
    let mut tmp = n;
    while tmp > 0 {
        reversed = reversed * 10 + tmp % 10;
        tmp /= 10;
    }
    Some(reversed)
}

fn all_odd_digits(n: u64) -> bool {
    let mut tmp = n;
    while tmp > 0 {
        if (tmp % 10) % 2 == 0 {
            return false;
        }
        tmp /= 10;
    }
    true
}

fn main() {
    let start = Instant::now();

    // From the brute force run, count stops changing at 100_000_000
    // so we only need to search up to there.
    let limit = 100_000_000u64;
    let chunk_size = 1_000_000u64;

    let count: u64 = (0..limit / chunk_size)
        .into_par_iter()
        .map(|chunk| {
            let lo = chunk * chunk_size + 1;
            let hi = lo + chunk_size;
            let mut local = 0u64;
            for n in lo..hi {
                if let Some(rev) = reverse_number(n) {
                    if all_odd_digits(n + rev) {
                        local += 1;
                    }
                }
            }
            local
        })
        .sum();

    println!("\n=== RESULT ===");
    println!("  Reversible numbers below 1,000,000,000: {}", count);
    println!("  Total time: {:.2?}", start.elapsed());
}
