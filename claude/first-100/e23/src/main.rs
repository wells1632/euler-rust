fn main() {
    const LIMIT: usize = 28123; // All integers greater than 28123 can be written as sum of two abundant numbers

    // Find all abundant numbers up to LIMIT
    let mut abundant = Vec::new();
    for n in 1..LIMIT {
	if sum_of_divisors(n) > n {
	    abundant.push(n);
	}
    }

    // Mark all numbers that can be written as sum of two abundant numbers
    let mut can_be_sum = vec![false; LIMIT];
    for i in 0..abundant.len() {
	for j in i..abundant.len() {
	    let sum = abundant[i] + abundant[j];
	    if sum < LIMIT {
		can_be_sum[sum] = true;
	    } else {
		break;
	    }
	}
    }

    // Sum all numbers that cannot be written as sum of two abundant numbers
    let mut total = 0;
    for n in 1..LIMIT {
	if !can_be_sum[n] {
	    total += n;
	}
    }

    println!("Sum of all positive integers that cannot be written as sum of two abundant numbers: {}", total);
}

fn sum_of_divisors(n: usize) -> usize {
    let mut sum = 1;
    let sqrt_n = (n as f64).sqrt() as usize;

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
