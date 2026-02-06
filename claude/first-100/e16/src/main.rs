fn main() {
    let mut digits = vec![1]; // Start with 2^0 = 1

    // Multiply by 2 a thousand times
    for _ in 0..1000 {
	let mut carry = 0;

	for i in 0..digits.len() {
	    let product = digits[i] * 2 + carry;
	    digits[i] = product % 10;
	    carry = product / 10;
	}

	while carry > 0 {
	    digits.push(carry % 10);
	    carry /= 10;
	}
    }

    let sum: u32 = digits.iter().sum();

    println!("2^1000 has {} digits", digits.len());
    println!("Sum of digits: {}", sum);
}
