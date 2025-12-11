fn main() {
    let mut count = 0;
    let mut num = 2;

    while count < 10_001 {
	if is_prime(num) {
	    count += 1;
	    if count == 10_001 {
		println!("The 10,001st prime number is: {}", num);
		break;
	    }
	}
	num += 1;
    }
}

fn is_prime(n: u64) -> bool {
    if n < 2 {
	return false;
    }
    if n == 2 {
	return true;
    }
    if n % 2 == 0 {
	return false;
    }

    let mut i = 3;
    while i * i <= n {
	if n % i == 0 {
	    return false;
	}
	i += 2;
    }
    true
}
