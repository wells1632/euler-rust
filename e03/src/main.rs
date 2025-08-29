use euler::check_prime_i64;

fn main() {
    let start = 600851475143_i64;
    let mut sqrt_start = (start as f64).sqrt() as i64;
    while sqrt_start > 2 {
        if start % sqrt_start == 0 {
            if check_prime_i64(sqrt_start) {
                println!("{}", sqrt_start);
                break;

            }
        }
        sqrt_start-=1;
    }
}
