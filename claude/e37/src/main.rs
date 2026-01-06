fn is_prime(n: u32) -> bool {
    if n < 2 {
	return false;
    }
    if n == 2 {
	return true;
    }
    if n % 2 == 0 {
	return false;
    }

    let sqrt_n = (n as f64).sqrt() as u32;
    for i in (3..=sqrt_n).step_by(2) {
	if n % i == 0 {
	    return false;
	}
    }
    true
}

fn is_truncatable_prime(n: u32) -> bool {
    if n < 10 {
	return false; // Single digit primes are not considered truncatable
    }

    let s = n.to_string();

    // Check left-to-right truncation
    for i in 1..s.len() {
	let truncated: u32 = s[i..].parse().unwrap();
	if !is_prime(truncated) {
	    return false;
	}
    }

    // Check right-to-left truncation
    for i in 1..s.len() {
	let truncated: u32 = s[..s.len()-i].parse().unwrap();
	if !is_prime(truncated) {
	    return false;
	}
    }

    true
}

fn main() {
    let mut truncatable_primes = Vec::new();
    let mut n = 11; // Start from 11 (first two-digit prime)

    // There are exactly 11 truncatable primes
    while truncatable_primes.len() < 11 {
	if is_prime(n) && is_truncatable_prime(n) {
	    truncatable_primes.push(n);
	}
	n += 2; // Only check odd numbers after 11
    }

    let sum: u32 = truncatable_primes.iter().sum();

    println!("Truncatable primes: {:?}", truncatable_primes);
    println!("Sum of truncatable primes: {}", sum);
}
