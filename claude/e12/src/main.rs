fn main() {
    let mut n = 1;
    let mut triangle = 1;

    loop {
	if count_divisors(triangle) > 500 {
	    println!("The first triangle number with over 500 divisors is: {}", triangle);
	    break;
	}
	n += 1;
	triangle += n;
    }
}

fn count_divisors(n: u64) -> u32 {
    let mut count = 0;
    let sqrt_n = (n as f64).sqrt() as u64;

    for i in 1..=sqrt_n {
	if n % i == 0 {
	    count += 1;
	    if i != n / i {
		count += 1;
	    }
	}
    }

    count
}
