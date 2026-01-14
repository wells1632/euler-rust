use std::str::FromStr;

fn reverse_number(n: u128) -> u128 {
    let s: String = n.to_string().chars().rev().collect();
    u128::from_str(&s).unwrap()
}

fn is_palindrome(n: u128) -> bool {
    let s = n.to_string();
    let rev: String = s.chars().rev().collect();
    s == rev
}

fn is_lychrel(n: u128, max_iterations: usize) -> bool {
    let mut current = n;
    
    for _ in 0..max_iterations {
        let reversed = reverse_number(current);
        current = current + reversed;
        
        if is_palindrome(current) {
            return false; // Not a Lychrel number
        }
    }
    
    true // Likely a Lychrel number (no palindrome found)
}

fn main() {
    let limit = 10_000;
    let max_iterations = 50;
    let mut lychrel_numbers = Vec::new();
    
    for n in 1..limit {
        if is_lychrel(n, max_iterations) {
            lychrel_numbers.push(n);
        }
    }
    
    println!("Lychrel numbers below {}:", limit);
    println!("{:?}", lychrel_numbers);
    println!("\nTotal count: {}", lychrel_numbers.len());
}
