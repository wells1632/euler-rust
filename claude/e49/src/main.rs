fn is_prime(n: u32) -> bool {
    if n < 2 { return false; }
    if n == 2 { return true; }
    if n % 2 == 0 { return false; }

    let mut i = 3;
    while i * i <= n {
	if n % i == 0 { return false; }
	i += 2;
    }
    true
}

fn get_sorted_digits(n: u32) -> String {
    let mut digits: Vec<char> = n.to_string().chars().collect();
    digits.sort();
    digits.into_iter().collect()
}

fn are_permutations(a: u32, b: u32, c: u32) -> bool {
    let sorted_a = get_sorted_digits(a);
    let sorted_b = get_sorted_digits(b);
    let sorted_c = get_sorted_digits(c);

    sorted_a == sorted_b && sorted_b == sorted_c
}

fn main() {
    println!("Finding arithmetic sequences of three 4-digit primes that are permutations of each other...");

    let mut found_sequences = Vec::new();

    // Generate all 4-digit primes
    let mut primes = Vec::new();
    for n in 1000..=9999 {
	if is_prime(n) {
	    primes.push(n);
	}
    }

    println!("Found {} 4-digit primes", primes.len());

    // Check all pairs of primes to find arithmetic sequences
    for i in 0..primes.len() {
	for j in i + 1..primes.len() {
	    let a = primes[i];
	    let b = primes[j];
	    let diff = b - a;
	    let c = b + diff;

	    // Check if c is a 4-digit prime
	    if c <= 9999 && is_prime(c) {
		// Check if all three are permutations of each other
		if are_permutations(a, b, c) {
		    found_sequences.push((a, b, c, diff));
		    println!("Found sequence: {} + {} = {}, {} + {} = {} (difference: {})",
			     a, diff, b, b, diff, c, diff);
		    println!("  Sorted digits: {} -> {} -> {}",
			     get_sorted_digits(a), get_sorted_digits(b), get_sorted_digits(c));
		}
	    }
	}
    }

    println!("\nTotal sequences found: {}", found_sequences.len());

    if found_sequences.is_empty() {
	println!("No arithmetic sequences of three 4-digit primes found that are permutations of each other.");
    } else {
	println!("\nAll found sequences:");
	for (i, (a, b, c, diff)) in found_sequences.iter().enumerate() {
	    println!("{}. {}, {}, {} (difference: {})", i + 1, a, b, c, diff);
	}
    }
}
