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

fn is_pandigital(n: u64) -> bool {
    let s = n.to_string();
    let len = s.len();

    // Check if all digits 1 to len are present exactly once
    let mut used = vec![false; len + 1];

    for c in s.chars() {
	let digit = c.to_digit(10).unwrap() as usize;
	if digit == 0 || digit > len || used[digit] {
	    return false;
	}
	used[digit] = true;
    }

    // Check that all digits from 1 to len are used
    for i in 1..=len {
	if !used[i] {
	    return false;
	}
    }

    true
}

fn generate_permutations(digits: &mut Vec<u32>, start: usize, results: &mut Vec<u64>) {
    if start == digits.len() {
	let num_str: String = digits.iter().map(|&d| d.to_string()).collect();
	if let Ok(num) = num_str.parse::<u64>() {
	    results.push(num);
	}
	return;
    }

    for i in start..digits.len() {
	digits.swap(start, i);
	generate_permutations(digits, start + 1, results);
	digits.swap(start, i);
    }
}

fn find_largest_pandigital_prime() -> Option<u64> {
    // Start from 9-digit pandigitals and work down
    for n_digits in (1..=9).rev() {
	let mut digits: Vec<u32> = (1..=n_digits).collect();
	let mut permutations = Vec::new();

	generate_permutations(&mut digits, 0, &mut permutations);

	// Sort in descending order to find largest first
	permutations.sort_by(|a, b| b.cmp(a));

	for num in permutations {
	    if is_prime(num) {
		return Some(num);
	    }
	}
    }

    None
}

fn main() {
    match find_largest_pandigital_prime() {
	Some(prime) => {
	    println!("Largest pandigital prime: {}", prime);
	    println!("Verification - is pandigital: {}", is_pandigital(prime));
	    println!("Verification - is prime: {}", is_prime(prime));
	}
	None => println!("No pandigital prime found"),
    }
}
