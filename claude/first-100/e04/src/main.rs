use std::time::Instant;

fn main() {
    let mut largest_palindrome = 0;
    let start = Instant::now();
    
    for i in (100..1000).rev() {
        for j in (i..1000).rev() {
            let product = i * j;
            
            if product <= largest_palindrome {
                break;
            }
            
            if is_palindrome(product) {
                largest_palindrome = product;
            }
        }
    }
    
    println!("The largest palindrome from the product of two 3-digit numbers is: {}", largest_palindrome);
    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);
}

fn is_palindrome(n: u32) -> bool {
    let s = n.to_string();
    s == s.chars().rev().collect::<String>()
}

