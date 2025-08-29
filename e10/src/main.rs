use euler::check_prime_i32;

fn main() {
    let mut total = 0_i64;
    for i in 2..2000000 {
        if check_prime_i32(i) {
            total+=i as i64;
        }
    }

    println!("Answer: {}", total);
}
