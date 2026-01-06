fn main() {
    let limit = 1_000_000;
    let primes = sieve_of_eratosthenes(limit);
    let circular_primes = count_circular_primes(&primes, limit);

    println!("Number of circular primes below {}: {}", limit, circular_primes);
}

fn sieve_of_eratosthenes(limit: usize) -> Vec<bool> {
    let mut is_prime = vec![true; limit];
    is_prime[0] = false;
    if limit > 1 {
	is_prime[1] = false;
    }

    for i in 2..((limit as f64).sqrt() as usize + 1) {
	if is_prime[i] {
	    for j in ((i * i)..limit).step_by(i) {
		is_prime[j] = false;
	    }
	}
    }

    is_prime
}

fn count_circular_primes(primes: &[bool], limit: usize) -> usize {
    let mut count = 0;

    for i in 2..limit {
	if primes[i] && is_circular_prime(i, primes) {
	    count += 1;
	}
    }

    count
}

fn is_circular_prime(n: usize, primes: &[bool]) -> bool {
    let digits: Vec<u8> = n.to_string().chars()
	.map(|c| c.to_digit(10).unwrap() as u8)
	.collect();

    let len = digits.len();

    for i in 0..len {
	let rotated = rotate_digits(&digits, i);
	let num = digits_to_number(&rotated);

	if num >= primes.len() || !primes[num] {
	    return false;
	}
    }

    true
}

fn rotate_digits(digits: &[u8], rotation: usize) -> Vec<u8> {
    let len = digits.len();
    let rotation = rotation % len;

    let mut rotated = Vec::with_capacity(len);
    rotated.extend_from_slice(&digits[rotation..]);
    rotated.extend_from_slice(&digits[..rotation]);

    rotated
}

fn digits_to_number(digits: &[u8]) -> usize {
    digits.iter().fold(0, |acc, &digit| acc * 10 + digit as usize)
}
