/*
A permutation is an ordered arrangement of objects. For example, 3124 is one possible permutation
of the digits 1, 2, 3 and 4. If all of the permutations are listed numerically or alphabetically,
we call it lexicographic order. The lexicographic permutations of 0, 1 and 2 are:

012   021   102   120   201   210

What is the millionth lexicographic permutation of the digits 0, 1, 2, 3, 4, 5, 6, 7, 8 and 9?
 */

// This method takes a long time, but it does work.

fn main() {
    let mut count = 362880;
    for i in 987654322 as i64..=9876543210 {
        if uniq_count(i) == 10 {
            count+=1;
//            println!("{} - {}", count, i);
        }
        if count == 1000000 {
            println!("Count: {} Value: {}", count, i);
            break;
        }
    }
    

}

fn uniq_count(n: i64) -> i32 {
    let mut digits = Vec::new();
    let mut n = n;
    if n<999999999 {
        digits.push(0);
    }
    while n>9 {
        digits.push(n%10);
        n=n/10;
    }
    digits.push(n);
    digits.sort();
    digits.dedup();
    digits.len() as i32
}

