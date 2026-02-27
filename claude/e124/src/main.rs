fn main() {
    let limit = 100000;
    
    // Calculate rad(n) for all n and store as (n, rad(n))
    let mut pairs: Vec<(u32, u32)> = Vec::new();
    
    for n in 1..=limit {
        let rad = radical(n);
        pairs.push((n, rad));
    }
    
    // Sort by rad(n) first, then by n when rad values are equal
    pairs.sort_by(|a, b| {
        if a.1 == b.1 {
            a.0.cmp(&b.0)  // Sort by n when rad(n) is equal
        } else {
            a.1.cmp(&b.1)  // Sort by rad(n)
        }
    });
    
    // E(k) is the k-th element (1-indexed)
    let e_10000 = pairs[9999].0;  // 0-indexed array
    
    println!("E(10000) = {}", e_10000);
    
    // Verify with small example (n<=10)
    println!("\nVerification with n<=10:");
    let mut small_pairs: Vec<(u32, u32)> = Vec::new();
    for n in 1..=10 {
        let rad = radical(n);
        small_pairs.push((n, rad));
        println!("n={}, rad(n)={}", n, rad);
    }
    small_pairs.sort_by(|a, b| {
        if a.1 == b.1 {
            a.0.cmp(&b.0)
        } else {
            a.1.cmp(&b.1)
        }
    });
    println!("\nSorted for n<=10:");
    for i in 0..10 {
        println!("E({}) = n={}, rad(n)={}", i+1, small_pairs[i].0, small_pairs[i].1);
    }
}

fn radical(n: u32) -> u32 {
    if n == 1 {
        return 1;
    }
    
    let mut rad = 1;
    let mut num = n;
    let mut factor = 2;
    
    // Find all distinct prime factors
    while factor * factor <= num {
        if num % factor == 0 {
            rad *= factor;  // Multiply rad by this prime factor once
            // Remove all occurrences of this factor
            while num % factor == 0 {
                num /= factor;
            }
        }
        factor += 1;
    }
    
    // If num > 1, then it's a prime factor
    if num > 1 {
        rad *= num;
    }
    
    rad
}
