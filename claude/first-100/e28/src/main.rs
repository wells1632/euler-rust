fn main() {
    let size = 1001;
    let sum = calculate_diagonal_sum(size);
    println!("Sum of diagonals for {}x{} spiral: {}", size, size, sum);
}

fn calculate_diagonal_sum(size: usize) -> u64 {
    if size == 1 {
	return 1;
    }

    let mut sum: u64 = 1; // Start with center value
    let mut current = 1u64;

    // Process each layer of the spiral
    // Layer 1 has side length 3, layer 2 has side length 5, etc.
    for layer in 1..=(size / 2) {
	let step = (layer * 2) as u64;

	// Each layer has 4 corners (diagonal values)
	// Move to each corner by adding the step size
	for _ in 0..4 {
	    current += step;
	    sum += current;
	}
    }

    sum
}
