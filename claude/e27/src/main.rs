fn is_prime(n: i32) -> bool {
    if n < 2 {
	return false;
    }
    if n == 2 {
	return true;
    }
    if n % 2 == 0 {
	return false;
    }

    let limit = (n as f64).sqrt() as i32;
    for i in (3..=limit).step_by(2) {
	if n % i == 0 {
	    return false;
	}
    }
    true
}

fn count_consecutive_primes(a: i32, b: i32) -> i32 {
    let mut n = 0;
    loop {
	let value = n * n + a * n + b;
	if !is_prime(value.abs()) {
	    return n;
	}
	n += 1;
    }
}

fn main() {
    let mut max_count = 0;
    let mut best_a = 0;
    let mut best_b = 0;

    for a in -999..1000 {
	for b in -1000..=1000 {
	    let count = count_consecutive_primes(a, b);
	    if count > max_count {
		max_count = count;
		best_a = a;
		best_b = b;
	    }
	}
    }

    println!("Best a: {}", best_a);
    println!("Best b: {}", best_b);
    println!("Consecutive primes: {}", max_count);
    println!("Product a * b: {}", best_a * best_b);
}
