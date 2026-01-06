use std::collections::HashSet;

fn digits_to_set(mut n: u32) -> HashSet<u32> {
    let mut digits = HashSet::new();
    while n > 0 {
	digits.insert(n % 10);
	n /= 10;
    }
    digits
}

fn is_pandigital(a: u32, b: u32, product: u32) -> bool {
    let mut combined = format!("{}{}{}", a, b, product);

    // Must be exactly 9 digits
    if combined.len() != 9 {
	return false;
    }

    // Check each digit 1-9 appears exactly once
    combined.chars().all(|c| c >= '1' && c <= '9') &&
	combined.chars().collect::<HashSet<_>>().len() == 9
}

fn main() {
    let mut products = HashSet::new();

    // We need a * b = c where digits of a, b, c form a pandigital
    // Since we have 9 digits total, we can bound our search:
    // 1 digit * 4 digits = 4 digits (e.g., 2 * 1234 = 2468)
    // 2 digits * 3 digits = 4 digits (e.g., 12 * 345 = 4140)

    // Case 1: 1 digit * 4 digits = 4 digits
    for a in 1..=9 {
	for b in 1234..=9876 {
	    let product = a * b;
	    if is_pandigital(a, b, product) {
		products.insert(product);
		println!("{} × {} = {}", a, b, product);
	    }
	}
    }

    // Case 2: 2 digits * 3 digits = 4 digits
    for a in 12..=98 {
	for b in 123..=987 {
	    let product = a * b;
	    if product > 9876 {
		break; // Product too large
	    }
	    if is_pandigital(a, b, product) {
		products.insert(product);
		println!("{} × {} = {}", a, b, product);
	    }
	}
    }

    let sum: u32 = products.iter().sum();
    println!("\nSum of all unique products: {}", sum);
}
