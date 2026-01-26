fn main() {
    let limit = 1_000_000;
    
    let mut best_n = 0u64;
    let mut best_d = 1u64;
    
    for d in 2..=limit {
        // Find largest n such that n/d < 3/7
        // n/d < 3/7 => 7n < 3d => n < 3d/7
        let n = (3 * d - 1) / 7;
        
        // Check if gcd(n, d) == 1 and if this is closer to 3/7
        if gcd(n, d) == 1 {
            // Compare n/d with best_n/best_d
            // n/d > best_n/best_d iff n*best_d > best_n*d
            if n * best_d > best_n * d {
                best_n = n;
                best_d = d;
            }
        }
    }
    
    println!("Fraction immediately to the left of 3/7: {}/{}", best_n, best_d);
    println!("Numerator: {}", best_n);
    
    // Verify it's less than 3/7
    let diff = 3.0 / 7.0 - (best_n as f64 / best_d as f64);
    println!("Distance from 3/7: {:.15}", diff);
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}
