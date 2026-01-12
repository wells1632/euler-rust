fn mod_pow(base: u128, exp: u128, modulus: u128) -> u128 {
    let mut result = 1;
    let mut base = base % modulus;
    let mut exp = exp;

    while exp > 0 {
	if exp % 2 == 1 {
	    result = (result * base) % modulus;
	}
	exp = exp >> 1;
	base = (base * base) % modulus;
    }

    result
}

fn main() {
    println!("Calculating the last ten digits of 1^1 + 2^2 + 3^3 + ... + 1000^1000");

    const MODULUS: u128 = 10_000_000_000; // 10^10 for last 10 digits
    let mut sum = 0u128;

    for i in 1..=1000 {
	let term = mod_pow(i, i, MODULUS);
	sum = (sum + term) % MODULUS;

	// Progress indicator for every 100 terms
	if i % 100 == 0 {
	    println!("Processed {} terms, current sum (mod 10^10): {}", i, sum);
	}
    }

    println!("\nFinal result:");
    println!("Last ten digits of the series: {:010}", sum);
    println!("Sum modulo 10^10: {}", sum);
}
