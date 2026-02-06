fn main() {
    // The probability of drawing 2 blue discs is: b(b-1) / (n(n-1)) = 1/2
    // This simplifies to: 2b(b-1) = n(n-1)
    // Which can be transformed into the Pell equation: x² - 2y² = -1
    // where x = 2n - 1 and y = 2b - 1
    
    let target = 1_000_000_000_000u64; // 10^12
    
    // Start with the fundamental solution (1, 1)
    let mut x = 1u128;
    let mut y = 1u128;
    
    loop {
        // Convert back to blue discs (b) and total discs (n)
        let n = (x + 1) / 2;
        let b = (y + 1) / 2;
        
        // Check if n > 10^12
        if n > target as u128 {
            println!("First arrangement with total discs > 10^12:");
            println!("Blue discs: {}", b);
            println!("Total discs: {}", n);
            println!("Red discs: {}", n - b);
            println!("\nThe Pell equation guarantees this is correct:");
            println!("P(two blue) = b(b-1) / n(n-1) = 1/2");
            break;
        }
        
        // Generate next solution using Pell equation recurrence
        // x_{k+1} = 3x_k + 4y_k
        // y_{k+1} = 2x_k + 3y_k
        let new_x = 3 * x + 4 * y;
        let new_y = 2 * x + 3 * y;
        
        x = new_x;
        y = new_y;
    }
}
