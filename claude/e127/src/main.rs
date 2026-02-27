fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 { a } else { gcd(b, a % b) }
}

fn main() {
    let limit = 120_000u64;
    let mut sum = 0u64;
    let mut hit_count = 0u64;

    // Precompute rad for all numbers up to limit
    let mut rad_arr = vec![1u64; limit as usize];
    for i in 2..limit as usize {
        if rad_arr[i] == 1 {
            let mut j = i;
            while j < limit as usize {
                rad_arr[j] *= i as u64;
                j += i;
            }
        }
    }

    println!("Sieve complete, searching for abc-hits...");

    for c in 3..limit {
        if c % 10_000 == 0 {
            println!("Progress: c = {:6} / {} | hits so far: {} | sum so far: {}", 
                c, limit, hit_count, sum);
        }

        let rad_c = rad_arr[c as usize];
        for a in 1..=(c - 1) / 2 {
            let b = c - a;
            if gcd(a, b) != 1 { continue; }
            let rad_abc = rad_arr[a as usize] * rad_arr[b as usize] * rad_c;
            if rad_abc < c {
                sum += c;
                hit_count += 1;
            }
        }
    }

    println!("---");
    println!("Complete! abc-hits for c < {}: {}", limit, hit_count);
    println!("Sum of c: {}", sum);
}
