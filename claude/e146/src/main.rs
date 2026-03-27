use std::time::Instant;

// ---------------------------------------------------------------------------
// Modular arithmetic helpers
// ---------------------------------------------------------------------------

fn mod_mul(mut a: u64, mut b: u64, m: u64) -> u64 {
    let mut result = 0u64;
    a %= m;
    while b > 0 {
        if b & 1 == 1 {
            result = result.wrapping_add(a);
            if result >= m { result -= m; }
        }
        a = a.wrapping_add(a);
        if a >= m { a -= m; }
        b >>= 1;
    }
    result
}

fn mod_pow(mut base: u64, mut exp: u64, m: u64) -> u64 {
    let mut result = 1u64;
    base %= m;
    while exp > 0 {
        if exp & 1 == 1 {
            result = mod_mul(result, base, m);
        }
        base = mod_mul(base, base, m);
        exp >>= 1;
    }
    result
}

// ---------------------------------------------------------------------------
// Deterministic Miller-Rabin for n < 3.3e24
// ---------------------------------------------------------------------------

fn is_prime_miller_rabin(n: u64) -> bool {
    if n < 2 { return false; }
    if n == 2 || n == 3 || n == 5 || n == 7 { return true; }
    if n % 2 == 0 || n % 3 == 0 || n % 5 == 0 { return false; }

    let mut d = n - 1;
    let mut r = 0u32;
    while d % 2 == 0 { d /= 2; r += 1; }

    const WITNESSES: [u64; 12] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];

    'outer: for &a in &WITNESSES {
        if a >= n { continue; }
        let mut x = mod_pow(a, d, n);
        if x == 1 || x == n - 1 { continue; }
        for _ in 0..r - 1 {
            x = mod_mul(x, x, n);
            if x == n - 1 { continue 'outer; }
        }
        return false;
    }
    true
}

// ---------------------------------------------------------------------------
// CRT residue sieve
// ---------------------------------------------------------------------------

const PRIMES: [u64; 9] = [2, 3, 5, 7, 11, 13, 17, 19, 23];
const MODULUS: u64 = 2 * 3 * 5 * 7 * 11 * 13 * 17 * 19 * 23; // 223,092,870
const OFFSETS: [u64; 6] = [1, 3, 7, 9, 13, 27];

// The gap interior offsets we need to ensure are NOT prime
const GAP_OFFSETS: [u64; 21] = [
    2, 4, 5, 6, 8, 10, 11, 12,
    14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26,
];

/// For a given residue r mod p, returns true if any of n²+offset ≡ 0 (mod p)
/// for the candidate offsets, OR if any gap interior value ≡ 0 (mod p).
/// We want to KEEP residues where none of the candidates are 0 mod p,
/// and at least we don't prematurely eliminate based on gap interiors
/// (those are handled separately).
fn is_valid_residue_mod_p(r: u64, p: u64) -> bool {
    let r2 = (r * r) % p;
    // None of the six candidate offsets can make n²+offset ≡ 0 mod p
    for &off in &OFFSETS {
        if (r2 + off) % p == 0 {
            return false;
        }
    }
    true
}

/// Build the list of valid residues mod MODULUS using a sieve approach:
/// iterate over all residues mod MODULUS and keep those that pass
/// every prime in PRIMES.
/// 
/// More efficiently, we build valid residues prime by prime and combine.
fn build_valid_residues() -> Vec<u64> {
    // Start with all residues mod 2, keep valid ones,
    // then lift to mod 2*3, then mod 2*3*5, etc.
    // This is the CRT lifting / sieving approach.

    let mut valid: Vec<u64> = (0..PRIMES[0]).filter(|&r| is_valid_residue_mod_p(r, PRIMES[0])).collect();
    let mut current_mod = PRIMES[0];

    for &p in &PRIMES[1..] {
        let mut next_valid = Vec::new();
        let next_mod = current_mod * p;

        // For each existing valid residue mod current_mod,
        // try all lifts r, r+current_mod, r+2*current_mod, ... mod next_mod
        for &r in &valid {
            for k in 0..p {
                let candidate = r + k * current_mod;
                // Check validity mod p for this lifted candidate
                if is_valid_residue_mod_p(candidate % p, p) {
                    next_valid.push(candidate);
                }
            }
        }

        valid = next_valid;
        current_mod = next_mod;
    }

    valid.sort_unstable();
    valid
}

// ---------------------------------------------------------------------------
// Consecutive prime check (Miller-Rabin only, residue filter already passed)
// ---------------------------------------------------------------------------

fn are_consecutive_primes(n: u64) -> bool {
    let base = n * n;

    // All six candidates must be prime
    for &off in &OFFSETS {
        if !is_prime_miller_rabin(base + off) {
            return false;
        }
    }

    // No primes in the gap interiors
    for &off in &GAP_OFFSETS {
        if is_prime_miller_rabin(base + off) {
            return false;
        }
    }

    true
}

// ---------------------------------------------------------------------------
// Main
// ---------------------------------------------------------------------------

fn main() {
    let limit = 150_000_000u64;

    // Build valid residues mod MODULUS
    print!("Computing valid residues mod {MODULUS}... ");
    let t0 = Instant::now();
    let valid_residues = build_valid_residues();
    println!(
        "{} residues found in {:.2?}  ({:.4}% of all residues)",
        valid_residues.len(),
        t0.elapsed(),
        valid_residues.len() as f64 / MODULUS as f64 * 100.0
    );

    let start = Instant::now();
    let mut last_report = Instant::now();
    let report_secs = 10u64;

    let mut sum = 0u64;
    let mut count = 0u64;

    println!("Searching for all valid n below {limit}...\n");

    // Iterate over complete cycles of MODULUS below limit,
    // then handle the partial final cycle
    let full_cycles = limit / MODULUS;
    let remainder   = limit % MODULUS;

    'outer: for cycle in 0..=full_cycles {
        let base_offset = cycle * MODULUS;

        for &r in &valid_residues {
            let n = base_offset + r;

            // Skip n=0, and stop when we exceed the limit
            if n == 0 { continue; }
            if n >= limit { break 'outer; }

            if are_consecutive_primes(n) {
                sum += n;
                count += 1;
                println!(
                    "[{:>8.1}s] Found n = {:<12} | count: {:<6} | running sum: {}",
                    start.elapsed().as_secs_f64(),
                    n, count, sum
                );
            }

            // Progress report
            if last_report.elapsed().as_secs() >= report_secs {
                let elapsed = start.elapsed().as_secs_f64();
                let progress = n as f64 / limit as f64;
                let eta = (elapsed / progress) - elapsed;
                println!(
                    "  [progress] n = {n:<12} | {:.2}% | elapsed: {:.1}s | ETA: {:.1}s",
                    progress * 100.0, elapsed, eta
                );
                last_report = Instant::now();
            }
        }

        // On the last cycle, we only care about residues below remainder
        // (already handled by the n >= limit check above)
        let _ = remainder;
    }

    println!("\n===== RESULTS =====");
    println!("Sum of all valid n below {limit}: {sum}");
    println!("Total count: {count}");
    println!("Total time:  {:.2?}", start.elapsed());
}
