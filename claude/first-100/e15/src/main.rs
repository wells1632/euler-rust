use std::time::Instant;
fn main() {
    let start = Instant::now();
    let grid_size = 20;

    // This is a combinatorial problem: C(40, 20)
    // We need to make 40 moves total (20 right, 20 down)
    // Choose which 20 of those moves are "right"

    let result = binomial_coefficient(grid_size * 2, grid_size);

    println!("Number of unique routes through a {}x{} grid: {}", grid_size, grid_size, result);
    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);
}

fn binomial_coefficient(n: u64, k: u64) -> u64 {
    let mut result: u64 = 1;

    for i in 0..k {
	result = result * (n - i) / (i + 1);
    }

    result
}
