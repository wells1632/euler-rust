use std::time::Instant;

fn main() {
    let start = Instant::now();

    let sum: u32 = (1..1000)
        .filter(|n| n % 3 == 0 || n % 5 == 0)
        .sum();
    println!("The sum is: {}", sum);

    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);
}
