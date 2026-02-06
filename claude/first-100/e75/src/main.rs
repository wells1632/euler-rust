fn main() {
    let limit = 1_500_000;
    
    // Count solutions for each perimeter
    let mut solutions = vec![0u32; limit + 1];
    
    // Generate all Pythagorean triples with perimeter <= limit
    // Using the parametric form: a = m²-n², b = 2mn, c = m²+n²
    let m_limit = ((limit as f64).sqrt() as usize) + 1;
    
    for m in 2..m_limit {
        for n in 1..m {
            // Only primitive triples: gcd(m,n) = 1 and m-n odd
            if gcd(m, n) == 1 && (m - n) % 2 == 1 {
                let a = m * m - n * n;
                let b = 2 * m * n;
                let c = m * m + n * n;
                
                let p = a + b + c;
                
                // Generate all multiples of this primitive triple
                let mut k = 1;
                while k * p <= limit {
                    solutions[k * p] += 1;
                    k += 1;
                }
            }
        }
    }
    
    // Count perimeters with exactly one solution
    let count = solutions.iter().filter(|&&s| s == 1).count();
    
    println!("Number of values with exactly one solution: {}", count);
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}
