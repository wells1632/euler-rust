fn pentagonal(n: u64) -> u64 {
    n * (3 * n - 1) / 2
}

fn is_pentagonal(x: u64) -> bool {
    // For a number to be pentagonal: n = (1 + sqrt(1 + 24x)) / 6
    // We check if this gives an integer n
    let discriminant = 1 + 24 * x;
    let sqrt_disc = (discriminant as f64).sqrt();

    if sqrt_disc.fract() != 0.0 {
	return false;
    }

    let sqrt_disc = sqrt_disc as u64;
    (1 + sqrt_disc) % 6 == 0
}

fn main() {
    println!("Searching for pentagonal number pairs...");

    let mut found = false;
    let mut min_difference = u64::MAX;
    let mut result_pair = (0, 0);

    // Generate pentagonal numbers and check pairs
    for j in 1..3000 {
	let pj = pentagonal(j);

	for k in j + 1..3000 {
	    let pk = pentagonal(k);
	    let sum = pj + pk;
	    let diff = pk - pj;

	    // Check if both sum and difference are pentagonal
	    if is_pentagonal(sum) && is_pentagonal(diff) {
		println!("Found pair: P{} = {}, P{} = {}", j, pj, k, pk);
		println!("Sum: {} (pentagonal: {})", sum, is_pentagonal(sum));
		println!("Difference: {} (pentagonal: {})", diff, is_pentagonal(diff));

		if diff < min_difference {
		    min_difference = diff;
		    result_pair = (pj, pk);
		    found = true;
		}

		println!("Current minimum difference: {}", min_difference);
		println!();
	    }
	}
    }

    if found {
	println!("Minimum difference found: {}", min_difference);
	println!("Pair: {} and {}", result_pair.0, result_pair.1);
    } else {
	println!("No such pair found in the given range.");
    }
}
