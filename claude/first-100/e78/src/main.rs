fn main() {
    let modulo = 1_000_000;
    let limit = 100000; // Should be enough
    
    // Compute partition function modulo 1,000,000
    let mut p = vec![0i64; limit];
    p[0] = 1;
    
    for n in 1..limit {
        let mut sum = 0i64;
        let mut k = 1;
        
        loop {
            // Generalized pentagonal numbers: k(3k-1)/2 for k = 1, -1, 2, -2, 3, -3, ...
            let pent1 = k * (3 * k - 1) / 2;
            let pent2 = k * (3 * k + 1) / 2;
            
            if pent1 > n as i64 {
                break;
            }
            
            // Signs alternate in pairs: +, +, -, -, +, +, -, -, ...
            let sign = if (k - 1) % 2 == 0 { 1 } else { -1 };
            
            sum += sign * p[n - pent1 as usize];
            
            if pent2 <= n as i64 {
                sum += sign * p[n - pent2 as usize];
            }
            
            k += 1;
        }
        
        p[n] = ((sum % modulo) + modulo) % modulo;
        
        if p[n] == 0 {
            println!("Least value of n where p(n) is divisible by {}: {}", modulo, n);
            return;
        }
    }
    
    println!("Need to increase limit!");
}
