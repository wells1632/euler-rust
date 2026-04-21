fn count_digit_occurrences(n: i64, d: u8) -> i64 {
    if n < 0 { return 0; }
    let d = d as i64;
    let mut count = 0i64;
    let mut m = 1i64;

    while m <= n {
        let low = n % m;
        let cur = (n / m) % 10;
        let high = n / m / 10;

        if cur < d {
            count += high * m;
        } else if cur == d {
            count += high * m + low + 1;
        } else {
            count += (high + 1) * m;
        }

        if d == 0 {
            count -= m;
        }

        m *= 10;
    }

    count
}

fn find_fixed_points(d: u8, lo: i64, hi: i64, solutions: &mut Vec<i64>) {
    if lo > hi { return; }

    let f_lo = count_digit_occurrences(lo, d);
    let f_hi = count_digit_occurrences(hi, d);

    if f_lo > hi || f_hi < lo {
        return;
    }

    if hi - lo <= 1000 {
        for n in lo..=hi {
            if count_digit_occurrences(n, d) == n {
                solutions.push(n);
            }
        }
        return;
    }

    let mid = lo + (hi - lo) / 2;
    find_fixed_points(d, lo, mid, solutions);
    find_fixed_points(d, mid + 1, hi, solutions);
}

fn main() {
    // Verify test cases
    println!("f(0,1)={}", count_digit_occurrences(0, 1));
    println!("f(1,1)={}", count_digit_occurrences(1, 1));
    println!("f(9,1)={}", count_digit_occurrences(9, 1));
    println!("f(10,1)={}", count_digit_occurrences(10, 1));
    println!("f(11,1)={}", count_digit_occurrences(11, 1));
    println!("f(12,1)={}", count_digit_occurrences(12, 1));

    // First, let's investigate boundary behaviour around 10^10
    println!("\nChecking boundary behaviour:");
    for d in 1u8..=9 {
        let n = 10_000_000_000i64;
        let f = count_digit_occurrences(n, d);
        println!("  f({}, {}) = {}  diff = {}", n, d, f, f - n);
    }

    // Check a wider range to see where f(n,d) - n changes sign
    println!("\nChecking where f(n,d) crosses n for large n:");
    for d in 1u8..=9 {
        // Sample at key points to understand the trajectory
        for exp in 9..=12 {
            let n = 10i64.pow(exp);
            let f = count_digit_occurrences(n, d);
            println!("  d={} n=10^{}: f={}, n={}, diff={}", d, exp, f, n, f - n);
        }
    }

    // Find true upper bound: find where f(n,d) < n for ALL n beyond this point
    // f(n,d) is roughly n * num_digits(n) / 10
    // For n = 10^11, num_digits = 12, f ~ 1.2 * 10^11 > 10^11 still possible
    // For n = 10^13, num_digits = 14, f ~ 1.4 * 10^13 > 10^13
    // Hmm, we need to find where f(n,d) definitively drops below n permanently
    
    // Actually for very large n:
    // The number of times digit d appears in [1..n] ~ n * log10(n) / 10
    // We need n * log10(n) / 10 = n => log10(n) = 10 => n = 10^10
    // But this is approximate. Let's check up to 10^13.

    // Also verify: is 10^10 a true solution or a false positive?
    println!("\nVerifying suspicious boundary solutions:");
    for d in 1u8..=9 {
        let n = 10_000_000_000i64;
        let f = count_digit_occurrences(n, d);
        println!("  d={}: f(10^10, {}) = {}, is_solution={}", d, d, f, f == n);
    }

    println!("\nSearching up to 10^13:");
    let upper = 100_000_000_000_000i64; // 10^14 to be very safe

    let mut total_sum: i64 = 0;

    for d in 1u8..=9 {
        let mut solutions: Vec<i64> = Vec::new();
        find_fixed_points(d, 0, upper, &mut solutions);
        solutions.sort();
        let s: i64 = solutions.iter().sum();
        println!("s({}) = {}  ({} solutions, largest: {})", 
            d, s, solutions.len(), solutions.last().unwrap_or(&0));
        total_sum += s;
    }

    println!("\nAnswer: sum of s(d) for d=1..=9 = {}", total_sum);
}