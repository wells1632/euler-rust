fn find_cycle_length(d: usize) -> usize {
    // Remove factors of 2 and 5 (they don't contribute to cycles)
    let mut d_reduced = d;
    while d_reduced % 2 == 0 {
	d_reduced /= 2;
    }
    while d_reduced % 5 == 0 {
	d_reduced /= 5;
    }

    if d_reduced == 1 {
	return 0; // No cycle (terminating decimal)
    }

    // Find the period using modular arithmetic
    let mut remainder = 1;
    let mut seen = std::collections::HashMap::new();
    let mut position = 0usize;

    loop {
	if remainder == 0 {
	    return 0; // Terminating decimal
	}

	if let Some(&start_pos) = seen.get(&remainder) {
	    return position - start_pos; // Found the cycle length
	}

	seen.insert(remainder, position);
	remainder = (remainder * 10) % d_reduced;
	position += 1;
    }
}

fn main() {
    let mut max_cycle = 0;
    let mut best_d = 0;

    for d in 2..1000 {
	let cycle = find_cycle_length(d);
	if cycle > max_cycle {
	    max_cycle = cycle;
	    best_d = d;
	}
    }

    println!("d with longest cycle: {}", best_d);
    println!("Cycle length: {}", max_cycle);
}
