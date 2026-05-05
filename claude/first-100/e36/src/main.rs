use std::time::Instant;
fn main() {
    let start = Instant::now();
    let mut sum = 0u64;
    let mut palindromes = Vec::new();

    // Check numbers up to a reasonable limit (1 million should be sufficient)
    for n in 1..1_000_000 {
	if is_decimal_palindrome(n) && is_binary_palindrome(n) {
	    palindromes.push(n);
	    sum += n as u64;
	}
    }

    println!("Double palindromes found: {:?}", palindromes);
    println!("Sum of all double palindromes: {}", sum);
    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);
}

fn is_decimal_palindrome(n: u32) -> bool {
    let s = n.to_string();
    s == s.chars().rev().collect::<String>()
}

fn is_binary_palindrome(n: u32) -> bool {
    let binary = format!("{:b}", n);
    binary == binary.chars().rev().collect::<String>()
}
