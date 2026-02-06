fn generate_pandigital_permutations() -> Vec<String> {
    let mut digits = vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    let mut permutations = Vec::new();

    fn permute(digits: &mut Vec<char>, start: usize, result: &mut Vec<String>) {
	if start == digits.len() {
	    result.push(digits.iter().collect());
	    return;
	}

	for i in start..digits.len() {
	    digits.swap(start, i);
	    permute(digits, start + 1, result);
	    digits.swap(start, i);
	}
    }

    permute(&mut digits, 0, &mut permutations);
    permutations
}

fn check_divisibility_property(number: &str) -> bool {
    let primes = [2, 3, 5, 7, 11, 13, 17];

    // Check each 3-digit substring starting from position 1
    for i in 0..7 {
	if number.len() < i + 4 {
	    return false;
	}

	let substring = &number[i+1..i+4];
	let value = substring.parse::<u32>().unwrap_or(0);

	if value % primes[i] != 0 {
	    return false;
	}
    }

    true
}

fn main() {
    println!("Searching for pandigital numbers with special divisibility property...");

    let permutations = generate_pandigital_permutations();
    let mut valid_numbers = Vec::new();

    for number in permutations {
	if check_divisibility_property(&number) {
	    valid_numbers.push(number.clone());

	    println!("Found: {}", number);
	    println!("  d2d3d4 = {} (÷2 = {})", &number[1..4], number[1..4].parse::<u32>().unwrap() / 2);
	    println!("  d3d4d5 = {} (÷3 = {})", &number[2..5], number[2..5].parse::<u32>().unwrap() / 3);
	    println!("  d4d5d6 = {} (÷5 = {})", &number[3..6], number[3..6].parse::<u32>().unwrap() / 5);
	    println!("  d5d6d7 = {} (÷7 = {})", &number[4..7], number[4..7].parse::<u32>().unwrap() / 7);
	    println!("  d6d7d8 = {} (÷11 = {})", &number[5..8], number[5..8].parse::<u32>().unwrap() / 11);
	    println!("  d7d8d9 = {} (÷13 = {})", &number[6..9], number[6..9].parse::<u32>().unwrap() / 13);
	    println!("  d8d9d10 = {} (÷17 = {})", &number[7..10], number[7..10].parse::<u32>().unwrap() / 17);
	    println!();
	}
    }

    if valid_numbers.is_empty() {
	println!("No pandigital numbers found with the required property.");
    } else {
	let sum: u64 = valid_numbers.iter()
	    .map(|s| s.parse::<u64>().unwrap())
	    .sum();

	println!("Total valid numbers found: {}", valid_numbers.len());
	println!("Sum of all valid numbers: {}", sum);
    }
}
