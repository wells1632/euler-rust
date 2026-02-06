fn main() {
    let limit = 12000;
    let mut count = 0;
    
    for d in 2..=limit {
        // Find range: 1/3 < n/d < 1/2
        // n/d > 1/3 => 3n > d => n > d/3
        // n/d < 1/2 => 2n < d => n < d/2
        
        let min_n = d / 3 + 1;  // smallest n such that n > d/3
        let max_n = (d - 1) / 2; // largest n such that n < d/2
        
        for n in min_n..=max_n {
            if gcd(n, d) == 1 {
                count += 1;
            }
        }
    }
    
    println!("Number of reduced proper fractions between 1/3 and 1/2 for d <= {}: {}", limit, count);
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}
