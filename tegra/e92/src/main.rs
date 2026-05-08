use std::time::Instant;
extern "C" {
    fn count_reaching_89_cuda(limit: u32) -> u32;
}

fn main() {
    let start = Instant::now();
    let limit = 10_000_000u32;
    let count = unsafe { count_reaching_89_cuda(limit) };
    println!("Numbers below {} that reach 89: {}", limit, count);
    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);
}
