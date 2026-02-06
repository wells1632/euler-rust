fn get_digit_at_position(position: usize) -> u32 {
    let mut current_pos = 0;
    let mut num = 1;

    loop {
	let num_str = num.to_string();
	let num_length = num_str.len();

	if current_pos + num_length >= position {
	    // The digit is within this number
	    let digit_index = position - current_pos - 1;
	    return num_str.chars().nth(digit_index).unwrap().to_digit(10).unwrap();
	}

	current_pos += num_length;
	num += 1;
    }
}

fn main() {
    let positions = [1, 10, 100, 1000, 10000, 100000, 1000000];
    let mut digits = Vec::new();

    println!("Finding digits at positions:");
    for &pos in &positions {
	let digit = get_digit_at_position(pos);
	digits.push(digit);
	println!("Position {}: {}", pos, digit);
    }

    let product: u32 = digits.iter().product();
    println!("\nProduct of all digits: {}", product);
}
