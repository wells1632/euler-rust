fn is_prime(n: u64) -> bool {
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

fn can_be_written_as_prime_plus_twice_square(n: u64) -> bool {
    // Check if n can be written as p + 2*k^2 where p is prime
    let mut k = 1;
    while 2 * k * k < n {
	let remainder = n - 2 * k * k;
	if is_prime(remainder) {
	    return true;
	}
	k += 1;
    }
    false
}

fn main() {
    println!("Finding the smallest odd composite that cannot be written as prime + 2*square...");

    let mut n = 9; // Start with first odd composite number

    loop {
	if !is_prime(n) { // Check if it's composite
	    if !can_be_written_as_prime_plus_twice_square(n) {
		println!("Found: {}", n);

		// Verify by showing what we tried
		println!("Verification - checking all possible decompositions:");
		let mut k = 1;
		while 2 * k * k < n {
		    let remainder = n - 2 * k * k;
		    println!("  {} - 2*{}² = {} - {} = {} (prime: {})",
			     n, k, n, 2*k*k, remainder, is_prime(remainder));
		    k += 1;
		}
		break;
	    }
	}
	n += 2; // Only check odd numbers

	// Safety check
	if n > 10000 {
	    println!("Searched up to {}, no solution found", n);
	    break;
	}
    }
}
