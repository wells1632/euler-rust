fn main() {
    let mut sum = 0;

    for a in 2..10000 {
	let b = sum_of_divisors(a);

	// Check if a and b are amicable (and not equal)
	if a != b && sum_of_divisors(b) == a {
	    sum += a;
	}
    }

    println!("Sum of all amicable numbers under 10000: {}", sum);
}

fn sum_of_divisors(n: u32) -> u32 {
    let mut sum = 1; // 1 is always a divisor
    let sqrt_n = (n as f64).sqrt() as u32;

    for i in 2..=sqrt_n {
	if n % i == 0 {
	    sum += i;
	    if i != n / i {
		sum += n / i;
	    }
	}
    }

    sum
}
