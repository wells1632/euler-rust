use std::time::Instant;

fn main() {
    let limit = 10000;
    let mut odd_period_count = 0;
    
    let start = Instant::now();
    
    for n in 2..=limit {
        if is_perfect_square(n) {
            continue;
        }
        
        let period = continued_fraction_period(n);
        if period % 2 == 1 {
            odd_period_count += 1;
        }
    }
    
    let duration = start.elapsed();
    
    println!("Numbers with odd period for N <= {}: {}", limit, odd_period_count);
    println!("Time elapsed: {:?}", duration);
    println!("Time elapsed: {:.3} ms", duration.as_secs_f64() * 1000.0);
}

fn is_perfect_square(n: u64) -> bool {
    let sqrt = (n as f64).sqrt() as u64;
    sqrt * sqrt == n
}

fn continued_fraction_period(n: u64) -> u64 {
    let a0 = (n as f64).sqrt() as u64;
    
    let mut m = 0i64;
    let mut d = 1i64;
    let mut a = a0 as i64;
    
    let mut period = 0u64;
    
    loop {
        m = d * a - m;
        d = (n as i64 - m * m) / d;
        a = (a0 as i64 + m) / d;
        period += 1;
        
        // The period ends when a = 2 * a0
        if a == 2 * a0 as i64 {
            break;
        }
    }
    
    period
}
