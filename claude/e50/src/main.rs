fn main() {
    let limit = 1_000_000;
    let primes = sieve_of_eratosthenes(limit);

    // Create a set for fast prime lookup
    let prime_set: std::collections::HashSet<_> = primes.iter().copied().collect();

    let mut max_length = 0;
    let mut best_prime = 0;

    // Try all possible starting positions
    for start in 0..primes.len() {
	let mut sum = 0;

	// Try all possible lengths from this starting position
	for end in start..primes.len() {
	    sum += primes[end];

	    if sum >= limit {
		break;
	    }

	    let length = end - start + 1;

	    // Check if sum is prime and we have a longer sequence
	    if length > max_length && prime_set.contains(&sum) {
		max_length = length;
		best_prime = sum;
	    }
	}
    }

    println!("Prime: {}", best_prime);
    println!("Consecutive primes: {}", max_length);
}

fn sieve_of_eratosthenes(limit: usize) -> Vec<usize> {
    let mut is_prime = vec![true; limit];
    is_prime[0] = false;
    if limit > 1 {
	is_prime[1] = false;
    }

    for i in 2..((limit as f64).sqrt() as usize + 1) {
	if is_prime[i] {
	    for j in (i * i..limit).step_by(i) {
		is_prime[j] = false;
	    }
	}
    }

    is_prime.iter()
	.enumerate()
	.filter_map(|(num, &prime)| if prime { Some(num) } else { None })
	.collect()
}
