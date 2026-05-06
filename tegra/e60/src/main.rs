// main.rs
use std::time::Instant;
use std::sync::{Arc, Mutex};
use rayon::prelude::*;

// ─── FFI binding to the CUDA kernel ───────────────────────────────────────────
extern "C" {
    fn launch_search(primes: *const u64, n_primes: i32, result: *mut u64);
}

// ─── CPU helpers ──────────────────────────────────────────────────────────────
fn is_prime(n: u64) -> bool {
    if n < 2 { return false; }
    if n == 2 { return true; }
    if n % 2 == 0 { return false; }
    let sqrt_n = (n as f64).sqrt() as u64;
    for i in (3..=sqrt_n).step_by(2) {
        if n % i == 0 { return false; }
    }
    true
}

fn concatenate(a: u64, b: u64) -> u64 {
    let b_digits = b.to_string().len() as u32;
    a * 10u64.pow(b_digits) + b
}

fn are_pair_compatible(a: u64, b: u64) -> bool {
    is_prime(concatenate(a, b)) && is_prime(concatenate(b, a))
}

// ─── Sequential search ────────────────────────────────────────────────────────
fn find_sequential(primes: &[u64]) -> Option<(u64, Vec<u64>)> {
    for i in 0..primes.len() {
        for j in i+1..primes.len() {
            if !are_pair_compatible(primes[i], primes[j]) { continue; }
            for k in j+1..primes.len() {
                if !are_pair_compatible(primes[i], primes[k])
                    || !are_pair_compatible(primes[j], primes[k]) { continue; }
                for l in k+1..primes.len() {
                    if !are_pair_compatible(primes[i], primes[l])
                        || !are_pair_compatible(primes[j], primes[l])
                        || !are_pair_compatible(primes[k], primes[l]) { continue; }
                    for m in l+1..primes.len() {
                        if !are_pair_compatible(primes[i], primes[m])
                            || !are_pair_compatible(primes[j], primes[m])
                            || !are_pair_compatible(primes[k], primes[m])
                            || !are_pair_compatible(primes[l], primes[m]) { continue; }
                        let sum = primes[i] + primes[j] + primes[k] + primes[l] + primes[m];
                        let set = vec![primes[i], primes[j], primes[k], primes[l], primes[m]];
                        return Some((sum, set));
                    }
                }
            }
        }
    }
    None
}

// ─── Parallel (Rayon) search ──────────────────────────────────────────────────
use std::sync::atomic::{AtomicBool, Ordering};

fn find_parallel(primes: &[u64]) -> Option<(u64, Vec<u64>)> {
    let found = Arc::new(AtomicBool::new(false));
    let pairs: Vec<(usize, usize)> = (0..primes.len())
        .flat_map(|i| (i+1..primes.len()).map(move |j| (i, j)))
        .collect();

    pairs.into_par_iter().find_map_any(|(i, j)| {
        if found.load(Ordering::Relaxed) { return None; }
        if !are_pair_compatible(primes[i], primes[j]) { return None; }
        for k in j+1..primes.len() {
            if found.load(Ordering::Relaxed) { return None; }
            if !are_pair_compatible(primes[i], primes[k]) { continue; }
            if !are_pair_compatible(primes[j], primes[k]) { continue; }
            for l in k+1..primes.len() {
                if found.load(Ordering::Relaxed) { return None; }
                if !are_pair_compatible(primes[i], primes[l]) { continue; }
                if !are_pair_compatible(primes[j], primes[l]) { continue; }
                if !are_pair_compatible(primes[k], primes[l]) { continue; }
                for m in l+1..primes.len() {
                    if found.load(Ordering::Relaxed) { return None; }
                    if !are_pair_compatible(primes[i], primes[m]) { continue; }
                    if !are_pair_compatible(primes[j], primes[m]) { continue; }
                    if !are_pair_compatible(primes[k], primes[m]) { continue; }
                    if !are_pair_compatible(primes[l], primes[m]) { continue; }
                    let sum = primes[i]+primes[j]+primes[k]+primes[l]+primes[m];
                    let set = vec![primes[i], primes[j], primes[k], primes[l], primes[m]];
                    found.store(true, Ordering::Relaxed);
                    return Some((sum, set));
                }
            }
        }
        None
    })
}


// ─── GPU search ───────────────────────────────────────────────────────────────
fn find_gpu(primes: &[u64]) -> Option<(u64, Vec<u64>)> {
    let mut result = vec![0u64; 7];
    unsafe {
        launch_search(primes.as_ptr(), primes.len() as i32, result.as_mut_ptr());
    }
    if result[6] == 1 {
        Some((result[5], result[0..5].to_vec()))
    } else {
        None
    }
}

// ─── Main ─────────────────────────────────────────────────────────────────────
fn main() {
    let limit = 10000;
    let mut primes = Vec::new();

    println!("Generating primes up to {}...\n", limit);
    for n in 2..limit {
        if is_prime(n) {
            primes.push(n);
        }
    }
    println!("Generated {} primes\n", primes.len());

    // Sequential
    println!("=== SEQUENTIAL VERSION ===");
    let start = Instant::now();
    match find_sequential(&primes) {
        Some((sum, set)) => {
            let d = start.elapsed();
            println!("Found set: {:?}", set);
            println!("Sum: {}", sum);
            println!("Time: {:.3}s ({} ms)\n", d.as_secs_f64(), d.as_millis());
        }
        None => {
            let d = start.elapsed();
            println!("No set found within the limit.");
            println!("Time: {:.3}s ({} ms)\n", d.as_secs_f64(), d.as_millis());
        }
    }

    // Parallel (Rayon)
    println!("=== PARALLEL VERSION ===");
    let start = Instant::now();
    match find_parallel(&primes) {
        Some((sum, set)) => {
            let d = start.elapsed();
            println!("Found set: {:?}", set);
            println!("Sum: {}", sum);
            println!("Time: {:.3}s ({} ms)\n", d.as_secs_f64(), d.as_millis());
        }
        None => {
            let d = start.elapsed();
            println!("No set found within the limit.");
            println!("Time: {:.3}s ({} ms)\n", d.as_secs_f64(), d.as_millis());
        }
    }

    // GPU
    println!("=== GPU VERSION ===");
    let start = Instant::now();
    match find_gpu(&primes) {
        Some((sum, set)) => {
            let d = start.elapsed();
            println!("Found set: {:?}", set);
            println!("Sum: {}", sum);
            println!("Time: {:.3}s ({} ms)\n", d.as_secs_f64(), d.as_millis());
        }
        None => {
            let d = start.elapsed();
            println!("No set found within the limit.");
            println!("Time: {:.3}s ({} ms)\n", d.as_secs_f64(), d.as_millis());
        }
    }
}
