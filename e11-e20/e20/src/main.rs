use num_traits::One;
use num_bigint::BigUint;

fn main() {

    let result = factorial(100);
    
    println!("factorial(100) = {}", result);
    println!("");
    let mut sum = 0;
    for val in result.to_string().chars() {
        sum += val as i32 - 0x30;
    }
    println!("Sum of digits: {}", sum);
}
    
fn factorial(n: usize) -> BigUint {
    let mut total = BigUint::one();
    for i in 1..=n {
        total = total * i;
    }
    return total;
}
