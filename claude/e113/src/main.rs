fn main() {
    println!("Counting non-bouncy numbers below 10^100\n");
    
    let max_digits = 100;
    
    let mut total_increasing = 0u128;
    let mut total_decreasing = 0u128;
    
    for k in 1..=max_digits {
        // For k-digit increasing numbers (digits d1 <= d2 <= ... <= dk, d1 >= 1):
        // Choose k digits from {1,2,...,9} with repetition: C(k+8, k)
        // This equals C(k+8, 8)
        let increasing = binomial(k + 8, 8);
        total_increasing += increasing;
        
        // For k-digit decreasing numbers (digits d1 >= d2 >= ... >= dk, d1 >= 1):
        // Choose k digits from {0,1,...,9} with repetition: C(k+9, k)
        // Subtract those where all digits are 0: 1
        // This equals C(k+9, 9) - 1
        let decreasing = binomial(k + 9, 9) - 1;
        total_decreasing += decreasing;
        
        if k <= 5 {
            println!("k={}: increasing={}, decreasing={}", k, increasing, decreasing);
        }
    }
    
    // Constant digit numbers (111, 222, etc.) are counted in both
    // There are 9 such numbers for each length
    let both = 9 * max_digits;
    
    let total = total_increasing + total_decreasing - both;
    
    println!("\nIncreasing: {}", total_increasing);
    println!("Decreasing: {}", total_decreasing);
    println!("Both (constant): {}", both);
    
    println!("\n{}", "=".repeat(70));
    println!("ANSWER: {} non-bouncy numbers below 10^100", total);
    println!("{}", "=".repeat(70));
}

fn binomial(n: u128, k: u128) -> u128 {
    if k > n {
        return 0;
    }
    if k == 0 || k == n {
        return 1;
    }
    
    let k = k.min(n - k);
    
    let mut result = 1u128;
    for i in 0..k {
        result = result * (n - i) / (i + 1);
    }
    
    result
}
