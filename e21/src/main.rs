/*
Let d(n) be defined as the sum of proper divisors of n (numbers less than n which divide
evenly into n). if d(a) =b and d(b)=a, where a!=b, then a and b are an amicable pair and
each of a and b are called amicable numbers.

For example, the proper divisors of 220 are 1, 2, 4, 5, 10, 11, 20, 22, 44, 55 and 110;
therefore d(220) = 284. The proper divisors of 284 are 1, 2, 4, 71, and 142; so d(284) = 220.

Evaluate the sum of all the amicable numbers under 10,000.
 */


use std::time::Instant;

fn main() {
    let now = Instant::now();
    let mut total = 0;
    // We'll just go through all 10000, not including 10000
    for i in 1..10000 {
        let a = get_factor_sum(i);
        let b = get_factor_sum(a);
        if i == b && a != b {
            total += i;
        }
    }
    let elapsed= now.elapsed();
    println!("Total: {}\nElapsed: {:.2?}", total, elapsed);
}

fn get_factor_sum(n:i32) -> i32 {
    let mut sum = 0;
    for i in 1..=(n/2) {
        if n % i == 0 {
            sum += i;
        }
    }
    return sum;
}
