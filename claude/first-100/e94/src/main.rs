fn main() {
    let limit = 1_000_000_000u64;
    
    println!("Finding almost equilateral triangles with:");
    println!("- Integral side lengths and area");
    println!("- Perimeter <= {}", limit);
    println!();
    
    let mut total_sum = 0u128;
    let mut count = 0;
    
    println!("Generating triangles...\n");
    
    // Check all possible values of a where (a,a,a-1) or (a,a,a+1) fit under limit
    let max_a = limit / 3 + 1;
    
    for a in 2..=max_a {
        // Type 1: (a, a, a-1)
        if a > 0 {
            let perimeter = 3 * a - 1;
            if perimeter <= limit && is_integer_area(a, a, a - 1) {
                println!("  ({}, {}, {}), perimeter = {}", a, a, a - 1, perimeter);
                total_sum += perimeter as u128;
                count += 1;
            }
        }
        
        // Type 2: (a, a, a+1)
        let perimeter = 3 * a + 1;
        if perimeter <= limit && is_integer_area(a, a, a + 1) {
            println!("  ({}, {}, {}), perimeter = {}", a, a, a + 1, perimeter);
            total_sum += perimeter as u128;
            count += 1;
        }
        
        // Progress indicator
        if a % 10_000_000 == 0 {
            println!("Progress: checked up to a = {}", a);
        }
    }
    
    println!("\n{}", "=".repeat(70));
    println!("RESULT:");
    println!("Total triangles found: {}", count);
    println!("Sum of all perimeters: {}", total_sum);
    println!("{}", "=".repeat(70));
}

fn is_integer_area(a: u64, b: u64, c: u64) -> bool {
    let a128 = a as u128;
    let b128 = b as u128;
    let c128 = c as u128;
    
    let perimeter = a128 + b128 + c128;
    
    if perimeter % 2 != 0 {
        let s2 = perimeter;
        let area_16_sq = s2 * (s2 - 2*a128) * (s2 - 2*b128) * (s2 - 2*c128);
        
        let sqrt_val = integer_sqrt(area_16_sq);
        sqrt_val * sqrt_val == area_16_sq
    } else {
        let s = perimeter / 2;
        if s < a128 || s < b128 || s < c128 {
            return false;
        }
        let area_sq = s * (s - a128) * (s - b128) * (s - c128);
        
        let sqrt_val = integer_sqrt(area_sq);
        sqrt_val * sqrt_val == area_sq
    }
}

fn integer_sqrt(n: u128) -> u128 {
    if n == 0 {
        return 0;
    }
    
    let mut x = n;
    let mut y = (x + 1) / 2;
    
    while y < x {
        x = y;
        y = (x + n / x) / 2;
    }
    
    x
}
