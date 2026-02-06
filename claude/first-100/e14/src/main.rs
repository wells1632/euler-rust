fn main() {
    let mut max_length = 0;
    let mut number_with_max = 0;

    for n in 1..1_000_000 {
	let length = collatz_length(n);
	if length > max_length {
	    max_length = length;
	    number_with_max = n;
	}
    }

    println!("Starting number with longest chain: {}", number_with_max);
    println!("Chain length: {}", max_length);
}

fn collatz_length(mut n: u64) -> u64 {
    let mut count = 1;

    while n != 1 {
	if n % 2 == 0 {
	    n = n / 2;
	} else {
	    n = 3 * n + 1;
	}
	count += 1;
    }

    count
}
