fn main() {
    let number: u64 = 600851475143;
    let mut n = number;
    let mut largest_prime = 0;
    
    // Remove all factors of 2
    while n % 2 == 0 {
        largest_prime = 2;
        n /= 2;
    }
    
    // Check odd factors from 3 onwards
    let mut factor = 3;
    while factor * factor <= n {
        while n % factor == 0 {
            largest_prime = factor;
            n /= factor;
        }
        factor += 2;
    }
    
    // If n is still greater than 1, then it's a prime factor
    if n > 1 {
        largest_prime = n;
    }
    
    println!("The largest prime factor of {} is: {}", number, largest_prime);
}
