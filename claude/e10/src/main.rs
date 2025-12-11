fn main() {
    let limit = 2_000_000;
    let sum: u64 = sieve_of_eratosthenes(limit).iter().sum();

    println!("The sum of all primes below {} is: {}", limit, sum);
}

fn sieve_of_eratosthenes(limit: usize) -> Vec<u64> {
    let mut is_prime = vec![true; limit];
    is_prime[0] = false;
    is_prime[1] = false;

    for i in 2..((limit as f64).sqrt() as usize + 1) {
	if is_prime[i] {
	    for j in (i * i..limit).step_by(i) {
		is_prime[j] = false;
	    }
	}
    }

    is_prime.iter()
	.enumerate()
	.filter(|(_, &prime)| prime)
	.map(|(num, _)| num as u64)
	.collect()
}
