fn main() {
    let sum = find_sum_of_fifth_power_numbers();
    println!("Sum of all numbers equal to sum of fifth powers of their digits: {}", sum);
}

fn find_sum_of_fifth_power_numbers() -> u32 {
    let mut total = 0;

    // Upper bound: 9^5 = 59049, so 6 digits max gives 6 * 59049 = 354294
    // 7 digits would give at most 7 * 59049 = 413343 (only 6 digits)
    // So we only need to check up to around 354294
    let upper_bound = 6 * 9u32.pow(5);

    // Start at 2 since we exclude 1 (single digit numbers don't count as sums)
    for n in 2..=upper_bound {
	if is_fifth_power_sum(n) {
	    println!("{} = sum of fifth powers of its digits", n);
	    total += n;
	}
    }

    total
}

fn is_fifth_power_sum(n: u32) -> bool {
    let mut sum = 0;
    let mut num = n;

    while num > 0 {
	let digit = num % 10;
	sum += digit.pow(5);
	num /= 10;
    }

    sum == n
}
