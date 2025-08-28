fn main() {
    let start = 600851475143_i64;
    let mut sqrt_start = (start as f64).sqrt() as i64;
    while sqrt_start > 2 {
        if start % sqrt_start == 0 {
            if check_prime(sqrt_start) {
                println!("{}", sqrt_start);
                break;

            }
        }
        sqrt_start-=1;
    }
}

fn check_prime(n: i64) -> bool {
    let mut i = 2;
    while i*i<=n {
        if n%i == 0 {
            return false;
        }
        i+=1;
    }
    return true;
}
