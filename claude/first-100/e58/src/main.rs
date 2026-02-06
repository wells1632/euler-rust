fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    
    let limit = (n as f64).sqrt() as u64;
    for i in (3..=limit).step_by(2) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn main() {
    let mut side_length = 1;
    let mut diagonal_count = 1; // Start with center (1)
    let mut prime_count = 0;    // 1 is not prime
    
    loop {
        side_length += 2; // Next odd side length (3, 5, 7, ...)
        
        // Calculate the four corner values for this square
        // Bottom-right corner is side_length^2
        let square = side_length * side_length;
        let step = side_length - 1;
        
        // Four diagonal numbers: n², n²-(n-1), n²-2(n-1), n²-3(n-1)
        let corners = [
            square,
            square - step,
            square - 2 * step,
            square - 3 * step,
        ];
        
        // Check each corner for primality
        for &corner in &corners {
            diagonal_count += 1;
            if is_prime(corner) {
                prime_count += 1;
            }
        }
        
        // Calculate ratio
        let ratio = prime_count as f64 / diagonal_count as f64;
        
        // Optional: Print progress every 100 iterations
        if side_length % 100 == 1 {
            println!("Side length: {}, Primes: {}/{}, Ratio: {:.4}", 
                     side_length, prime_count, diagonal_count, ratio);
        }
        
        // Check if ratio falls below 10%
        if ratio < 0.10 {
            println!("\nAnswer:");
            println!("Side length: {}", side_length);
            println!("Primes on diagonals: {}/{}", prime_count, diagonal_count);
            println!("Ratio: {:.6} ({:.2}%)", ratio, ratio * 100.0);
            break;
        }
    }
}