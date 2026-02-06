fn factorial(n: u32) -> u32 {
    match n {
	0 | 1 => 1,
	_ => n * factorial(n - 1)
    }
}

fn sum_of_digit_factorials(mut num: u32) -> u32 {
    let mut sum = 0;
    while num > 0 {
	let digit = num % 10;
	sum += factorial(digit);
	num /= 10;
    }
    sum
}

fn main() {
    // Precompute factorials for digits 0-9 for efficiency
    let factorials: [u32; 10] = [1, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880];

    let mut valid_numbers = Vec::new();

    // We need an upper bound. Since 9! = 362880, a 7-digit number has at most
    // 7 * 362880 = 2540160, so we don't need to check beyond that
    for num in 1..=2540160 {
	let mut temp = num;
	let mut sum = 0;

	// Calculate sum of factorial of digits using precomputed values
	while temp > 0 {
	    let digit = (temp % 10) as usize;
	    sum += factorials[digit];
	    temp /= 10;
	}

	if sum == num {
	    valid_numbers.push(num);
	    println!("{}", num);
	}
    }

    let total_sum: u32 = valid_numbers.iter().sum();
    println!("\nNumbers equal to sum of factorial of their digits:");
    for num in &valid_numbers {
	println!("{}", num);
    }

    println!("\nSum of all such numbers: {}", total_sum);
}
