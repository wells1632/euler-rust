/*
A perfect number is a number for which the sum of its proper divisors is exactly equal to the number.
For example, the sum of the proper divisors of 28 would be 1 + 2 + 4 + 7 + 14 = 28, which means that
28 is a perfect number.

A number n is called deficient if the sum of its proper divisors is less than n and it is called
abundant if this sum exceeds n.

As 12 is the smallest abundant number, 1 + 2 + 3 + 4 + 6 =16, the smallest number that can be written
as the sum of two abundant numbers is 24. By mathematical analysis, it can be shown that all integers
greater than 28123 can be written as the sum of two abundant numbers. However, this upper limit
cannot be reduced any further by analysis even though it is known that the greatest number that
cannot be expressed as the sum of two abundant numbers is less than this limit.

Find the sum of all the positive integers which cannot be written as the sum of two abundant numbers.
 */

use euler::get_factor_sum_i32;
use std::time::Instant;

fn main() {
    // First, let's generate a list of all of the abundant numbers less than 28123:
    let mut abundants = Vec::<i32>::new();
    let mut combined_abundants = Vec::<i32>::new();
    for i in 4..=28123 {
        if i < get_factor_sum_i32(i) {
//            println!("{} - {}", i, get_factor_sum_i32(i));
            abundants.push(i);
        }
    }
    // Now lets add all of the abundants together in all of the different combinations
    for i in abundants.iter() {
        for j in abundants.iter() {
            let k = i + j;
            if k < 28124 {

                combined_abundants.push(k);
                
            }
        }
    }
    combined_abundants.sort();
    combined_abundants.dedup();
    // Setup timing
    let mut now = Instant::now();
    // Using the contains method:
    let mut answer = 0;
    for i in 1..=28123 {
        if !combined_abundants.contains(&i) {
            answer+=i;
        }
    }
    let mut elapsed = now.elapsed();
    println!("This is the answer and time for using the Vector contains method:");
    println!("{} - {:.2?}", answer, elapsed);
    // Using the add/subtract method:
    now = Instant::now();
    let mut bigger = 0;
    let mut lesser = 0;
    for i in 1..=28123 {
        bigger += i;
    }
    for i in combined_abundants.iter() {
        lesser += i;
    }
    answer = bigger - lesser;
    elapsed = now.elapsed();
    println!("This is the answer and time for using a simple addition and subtraction iterative method:");
    println!("{} - {:.2?}", answer, elapsed);
    
}
