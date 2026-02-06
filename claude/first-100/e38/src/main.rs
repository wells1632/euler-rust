fn is_pandigital_9(n: u32) -> bool {
    let s = n.to_string();
    if s.len() != 9 {
	return false;
    }

    let mut digits = [false; 10];
    for c in s.chars() {
	let digit = c.to_digit(10).unwrap() as usize;
	if digit == 0 || digits[digit] {
	    return false;
	}
	digits[digit] = true;
    }
    true
}

fn get_concatenated_product(base: u32, max_multiplier: u32) -> Option<u32> {
    let mut result = String::new();

    for i in 1..=max_multiplier {
	result.push_str(&(base * i).to_string());
	if result.len() >= 9 {
	    break;
	}
    }

    if result.len() == 9 {
	result.parse().ok()
    } else {
	None
    }
}

fn find_largest_pandigital() -> u32 {
    let mut largest = 0;

    // Try different base numbers and multiplier ranges
    for base in 1..10000 {
	// Try different maximum multipliers
	for max_mult in 2..=9 {
	    if let Some(product) = get_concatenated_product(base, max_mult) {
		if is_pandigital_9(product) && product > largest {
		    largest = product;
		    println!("Found: {} (base: {}, multipliers: 1-{})", product, base, max_mult);
		}
	    }
	}
    }

    largest
}

fn main() {
    let result = find_largest_pandigital();
    println!("Largest 9-digit pandigital concatenated product: {}", result);
}
