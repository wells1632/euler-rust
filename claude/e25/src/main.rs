use num_bigint::BigUint;
use num_traits::{Zero, One};

fn main() {
    let mut a = BigUint::zero();
    let mut b = BigUint::one();
    let mut index = 1;

    loop {
	let next = &a + &b;
	a = b;
	b = next;
	index += 1;

	// Check if we've reached 1000 digits
	if b.to_string().len() >= 1000 {
	    println!("The first Fibonacci term with 1000 digits is at index: {}", index);
	    break;
	}
    }
}
