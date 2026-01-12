use std::f64::consts::PI;

fn is_perfect_square(n: i32) -> bool {
    if n <= 0 {
        return false;
    }
    let sqrt = (n as f64).sqrt();
    let int_sqrt = sqrt.round() as i32;
    int_sqrt * int_sqrt == n
}

fn distance_to_nearest_integer(x: f64) -> f64 {
    (x - x.round()).abs()
}

fn main() {
    let mut best_n = 0;
    let mut best_distance = f64::INFINITY;
    let mut best_value = 0.0;
    
    // Search through all positive integers up to 1000
    for n in 1..=1000 {
        // Skip perfect squares
        if is_perfect_square(n) {
            continue;
        }
        
        // Calculate cos(π * sqrt(n))
        let value = (PI * (n as f64).sqrt()).cos();
        
        // Find distance to nearest integer
        let dist = distance_to_nearest_integer(value);
        
        // Track the best (smallest distance)
        if dist < best_distance {
            best_distance = dist;
            best_n = n;
            best_value = value;
        }
    }
    
    println!("Best n: {}", best_n);
    println!("cos(π√{}) = {}", best_n, best_value);
    println!("Distance to nearest integer: {:.10}", best_distance);
    println!("Nearest integer: {}", best_value.round() as i32);
}
