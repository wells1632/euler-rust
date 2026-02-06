fn prime_factors(mut n: u64) -> Vec<u64> {
    let mut factors = Vec::new();
    let mut d = 2;

    while d * d <= n {
	while n % d == 0 {
	    if factors.is_empty() || factors.last() != Some(&d) {
		factors.push(d);
	    }
	    n /= d;
	}
	d += 1;
    }

    if n > 1 {
	factors.push(n);
    }

    factors
}

fn count_distinct_prime_factors(n: u64) -> usize {
    prime_factors(n).len()
}

fn main() {
    println!("Finding the first four consecutive integers with four distinct prime factors each...");

    let mut n = 2;

    loop {
	// Check if four consecutive numbers starting at n each have exactly 4 distinct prime factors
	let counts: Vec<usize> = (0..4)
	    .map(|i| count_distinct_prime_factors(n + i))
	    .collect();

	if counts.iter().all(|&count| count == 4) {
	    println!("Found four consecutive integers:");

	    for i in 0..4 {
		let num = n + i;
		let factors = prime_factors(num);
		println!("{}: prime factors = {:?} (count: {})",
			 num, factors, factors.len());
	    }

	    println!("\nThe first number in the sequence is: {}", n);
	    break;
	}

	n += 1;

	// Progress indicator for large searches
	if n % 10000 == 0 {
	    println!("Searching... current n = {}", n);
	}

	// Safety check
	if n > 1000000 {
	    println!("Searched up to {}, no solution found", n);
	    break;
	}
    }
}
