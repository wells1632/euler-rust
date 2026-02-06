use num_bigint::BigUint;
use std::collections::HashSet;

fn main() {
    let distinct_count = count_distinct_powers(100);
    println!("Number of distinct terms: {}", distinct_count);
}

fn count_distinct_powers(max: u32) -> usize {
    let mut distinct_values: HashSet<BigUint> = HashSet::new();

    for a in 2..=max {
	for b in 2..=max {
	    let base = BigUint::from(a);
	    let value = base.pow(b);
	    distinct_values.insert(value);
	}
    }

    distinct_values.len()
}
