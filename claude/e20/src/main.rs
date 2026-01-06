fn main() {
    let mut digits = vec![1]; // Start with 1! = 1

    // Multiply by each number from 2 to 100
    for n in 2..=100 {
	let mut carry = 0;

	for i in 0..digits.len() {
	    let product = digits[i] * n + carry;
	    digits[i] = product % 10;
	    carry = product / 10;
	}

	while carry > 0 {
	    digits.push(carry % 10);
	    carry /= 10;
	}
    }

    let sum: u32 = digits.iter().sum();

    println!("100! has {} digits", digits.len());
    println!("Sum of digits in 100!: {}", sum);
}
