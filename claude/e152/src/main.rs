use std::collections::HashSet;
use std::collections::HashMap;

fn primes_up_to(n: usize) -> Vec<usize> {
    let mut sieve = vec![true; n + 1];
    sieve[0] = false;
    sieve[1] = false;
    for i in 2..=n {
        if sieve[i] {
            let mut j = i * 2;
            while j <= n {
                sieve[j] = false;
                j += i;
            }
        }
    }
    (2..=n).filter(|&i| sieve[i]).collect()
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 { a } else { gcd(b, a % b) }
}

fn lcm(a: u64, b: u64) -> Option<u64> {
    let g = gcd(a, b);
    (a / g).checked_mul(b)
}

// For prime p, find which multiples of p can appear in valid subsets.
//
// Each multiple is x = k*p^a where gcd(k,p)=1.
// 1/x^2 = 1/(k^2 * p^(2a))
//
// For the total sum to equal 1/2 (denominator only has factor 2),
// the sum of terms involving p must have NO p in its denominator.
//
// Write each term as: 1/(k^2 * p^(2a))
// Group terms by their p-adic valuation 2a.
// The sum must have p-adic valuation >= 0 (no p in denominator).
//
// Equivalently: working over LCM of all x^2 values (= p^(2*max_a) * LCM(k^2)),
// the integer sum must be divisible by p^(2*max_a).
fn find_valid_multiples(p: usize, allowed: &HashSet<usize>) -> HashSet<usize> {
    let mut sorted: Vec<usize> = allowed.iter()
        .filter(|&&x| x % p == 0)
        .cloned()
        .collect();
    sorted.sort();
    let n = sorted.len();

    if n == 0 { return HashSet::new(); }
    if n == 1 { return HashSet::new(); }
    if n > 24 { return sorted.iter().cloned().collect(); }

    // For each multiple x, compute:
    //   pa = p-adic valuation of x (how many times p divides x)
    //   k  = x / p^pa  (the p-free part)
    let mut pa_vals: Vec<u32> = Vec::new();
    let mut k_vals: Vec<usize> = Vec::new();
    for &x in &sorted {
        let mut xx = x;
        let mut pa = 0u32;
        while xx % p == 0 { xx /= p; pa += 1; }
        pa_vals.push(pa);
        k_vals.push(xx);
    }

    let max_pa = *pa_vals.iter().max().unwrap();

    // Work over base = p^(2*max_pa) * lcm(k_i^2)
    // Each term in integer units = base / x^2
    //   = p^(2*max_pa) * lcm(k^2) / (k_i^2 * p^(2*pa_i))
    //   = p^(2*(max_pa - pa_i)) * lcm(k^2) / k_i^2

    // First compute lcm of k^2 values
    let k_lcm: u64 = match k_vals.iter().try_fold(1u64, |acc, &k| {
        lcm(acc, (k * k) as u64)
    }) {
        Some(v) => v,
        None => return sorted.iter().cloned().collect(),
    };

    // p^(2*max_pa)
    let p_part = match (p as u64).checked_pow(2 * max_pa) {
        Some(v) => v,
        None => return sorted.iter().cloned().collect(),
    };

    let base = match k_lcm.checked_mul(p_part) {
        Some(v) => v,
        None => return sorted.iter().cloned().collect(),
    };

    // Compute integer term for each multiple
    let terms: Vec<u64> = sorted.iter().enumerate().map(|(i, &x)| {
        base / (x as u64 * x as u64)
    }).collect();

    // For valid subset: sum of chosen terms must be divisible by p_part
    // (so that when divided by base, the fraction has no p in denominator)
    let mut valid_numbers: HashSet<usize> = HashSet::new();

    for mask in 1u32..(1u32 << n) {
        let sum: u64 = (0..n)
            .filter(|&i| mask & (1 << i) != 0)
            .map(|i| terms[i])
            .sum();

        if sum % p_part == 0 {
            for i in 0..n {
                if mask & (1 << i) != 0 {
                    valid_numbers.insert(sorted[i]);
                }
            }
        }
    }

    valid_numbers
}

fn main() {
    let max_val = 80;
    let primes = primes_up_to(max_val);

    let target_primes: Vec<usize> = primes.iter()
        .filter(|&&p| p > 3 && p <= max_val)
        .cloned()
        .collect();

    let mut allowed: HashSet<usize> = (2..=max_val).collect();

    // Iterative prime filtering until stable
    let mut iteration = 0;
    loop {
        iteration += 1;
        let mut changed = false;

        for &p in &target_primes {
            let multiples: Vec<usize> = allowed.iter()
                .filter(|&&x| x % p == 0)
                .cloned()
                .collect();

            if multiples.is_empty() { continue; }

            let valid = find_valid_multiples(p, &allowed);

            for &m in &multiples {
                if !valid.contains(&m) {
                    allowed.remove(&m);
                    println!("Iter {}: Removing {} (prime {})", iteration, m, p);
                    changed = true;
                }
            }
        }

        println!("--- Iteration {} done: {} candidates remaining ---",
            iteration, allowed.len());

        if !changed { break; }
    }

    let mut candidates: Vec<usize> = allowed.into_iter().collect();
    candidates.sort();
    println!("\nCandidates ({} total): {:?}", candidates.len(), candidates);

    let expected: Vec<usize> = vec![
        2,3,4,5,6,7,8,9,10,12,13,14,15,16,18,20,21,24,27,28,
        30,32,35,36,39,40,42,45,48,52,54,56,60,63,64,70,72,80
    ];
    println!("Expected  ({} total): {:?}", expected.len(), expected);

    if candidates != expected {
        let extra: Vec<usize> = candidates.iter()
            .filter(|x| !expected.contains(x))
            .cloned()
            .collect();
        let missing: Vec<usize> = expected.iter()
            .filter(|x| !candidates.contains(x))
            .cloned()
            .collect();
        if !extra.is_empty() { println!("Extra: {:?}", extra); }
        if !missing.is_empty() { println!("Missing: {:?}", missing); }
        println!("Proceeding with current candidates...");
    } else {
        println!("Candidate set matches expected!");
    }

    // Compute LCM using u128
    let overall_lcm: u128 = candidates.iter().fold(1u128, |acc, &x| {
        let sq = x as u128 * x as u128;
        let g = { let mut a = acc; let mut b = sq;
            while b != 0 { let t = b; b = a % b; a = t; } a };
        acc / g * sq
    });

    println!("\nOverall LCM: {}", overall_lcm);

    if overall_lcm % 2 != 0 {
        println!("ERROR: LCM is odd!");
        return;
    }

    let target = overall_lcm / 2;
    println!("Target: {}", target);

    // Verify all terms are exact
    for &x in &candidates {
        let sq = x as u128 * x as u128;
        if overall_lcm % sq != 0 {
            println!("ERROR: LCM not divisible by {}^2", x);
            return;
        }
    }

    let terms: Vec<u128> = candidates.iter()
        .map(|&x| overall_lcm / (x as u128 * x as u128))
        .collect();

    let n = candidates.len();
    let half = n / 2;
    let left_terms = &terms[..half];
    let right_terms = &terms[half..];

    println!("\nMeet in the middle: left={} elements, right={} elements",
        left_terms.len(), right_terms.len());
    println!("Left subsets:  {}", 1u64 << left_terms.len());
    println!("Right subsets: {}", 1u64 << right_terms.len());

    // Build right half HashMap
    println!("Building right half subset sums...");
    let right_n = right_terms.len();
    let mut right_sums: HashMap<u128, u64> =
        HashMap::with_capacity(1 << (right_n - 1));

    for mask in 0u64..(1u64 << right_n) {
        let sum: u128 = (0..right_n)
            .filter(|&i| mask & (1 << i) != 0)
            .map(|i| right_terms[i])
            .sum();
        if sum <= target {
            *right_sums.entry(sum).or_insert(0) += 1;
        }
    }
    println!("Right half: {} distinct sums", right_sums.len());

    // Enumerate left half and match
    println!("Enumerating left half...");
    let left_n = left_terms.len();
    let mut total_count = 0u64;

    for mask in 0u64..(1u64 << left_n) {
        let sum: u128 = (0..left_n)
            .filter(|&i| mask & (1 << i) != 0)
            .map(|i| left_terms[i])
            .sum();
        if sum <= target {
            let complement = target - sum;
            if let Some(&cnt) = right_sums.get(&complement) {
                total_count += cnt;
            }
        }
    }

    println!("\nTotal number of ways: {}", total_count);
}
