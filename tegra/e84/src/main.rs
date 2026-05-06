use std::time::Instant;
// main.rs

const BOARD_SIZE: usize = 40;
// 65536 threads × 1000 sims each = 65,536,000 simulations
const SIMULATIONS_PER_THREAD: u64 = 1000;
const TOTAL_THREADS: u64 = 256 * 256;

extern "C" {
    fn run_simulation(counts_out: *mut u64);
}

fn main() {
    let start = Instant::now();
    let mut counts = vec![0u64; BOARD_SIZE];

    unsafe {
        run_simulation(counts.as_mut_ptr());
    }

    let total_sims = SIMULATIONS_PER_THREAD * TOTAL_THREADS;

    let mut squares: Vec<(usize, u64)> = counts
        .iter()
        .enumerate()
        .map(|(i, &c)| (i, c))
        .collect();

    squares.sort_by(|a, b| b.1.cmp(&a.1));

    println!("Top 10 most visited squares:");
    for (i, (square, count)) in squares.iter().take(10).enumerate() {
        println!(
            "{}. Square {:02}: {} visits ({:.4}%)",
            i + 1,
            square,
            count,
            (*count as f64 / total_sims as f64) * 100.0
        );
    }

    let modal_string = format!(
        "{:02}{:02}{:02}",
        squares[0].0, squares[1].0, squares[2].0
    );
    println!("\nModal string (top 3 squares): {}", modal_string);
    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);
}
