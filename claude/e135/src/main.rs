fn main() {
    let limit = 1_000_000usize;
    let mut counts = vec![0u32; limit];

    // x, y, z are positive integers in AP.
    // Let the first term be a and common difference be d (d can be 0? No, they should be distinct)
    // So x=a, y=a+d, z=a+2d for positive integer a>=1, integer d (nonzero)
    // But d could be negative! If d < 0, let's substitute d = -e where e > 0:
    // x=a, y=a-e, z=a-2e, need all positive so a > 2e
    //
    // Case 1: d > 0
    // n = a^2 - (a+d)^2 - (a+2d)^2 = -a^2 - 6ad - 5d^2 < 0, not valid
    //
    // Case 2: d < 0, let e = -d > 0
    // x=a, y=a-e, z=a-2e
    // n = a^2 - (a-e)^2 - (a-2e)^2
    // n = a^2 - a^2 + 2ae - e^2 - a^2 + 4ae - 4e^2
    // n = -a^2 + 6ae - 5e^2
    // n = -(a^2 - 6ae + 5e^2)
    // n = -(a - e)(a - 5e)
    // For n > 0: (a-e)(a-5e) < 0
    // Since a > 2e > e, we have (a-e) > 0, so we need (a-5e) < 0
    // So e < a < 5e, combined with a > 2e: 2e < a < 5e
    // Also need z = a-2e >= 1, so a >= 2e+1
    // So: 2e < a < 5e, i.e. a from 2e+1 to 5e-1
    //
    // n = -(a-e)(a-5e) = (a-e)(5e-a)
    //
    // This is a different formula from before! Let's check both give same results
    // and see if we were double counting or missing solutions.
    //
    // Previous formula used z as base with positive d:
    // z=z, y=z+d, x=z+2d => same as case 2 with a=z+2d=x, e=d
    // So a = z+2e, substituting: a ranges from 2e+1 to 5e-1
    // => z+2e ranges from 2e+1 to 5e-1
    // => z ranges from 1 to 3e-1  <-- same as before!
    //
    // So the formula IS the same. The issue must be the upper bound on d.
    // n = (a-e)(5e-a), maximum when a = 3e (midpoint): n_max = 2e * 2e = 4e^2
    // So e_max = sqrt(limit/4) = sqrt(250000) = 500
    // BUT when a is near 2e or 5e, n can be small even for large e
    // We need e such that ANY n < limit is reachable
    // The constraint is 4e^2 >= smallest possible n, but we need ALL solutions
    // for n < limit, so we need e_max where minimum possible n for that e < limit
    // Min n for given e: a=2e+1 or a=5e-1 => n=(e+1)(1) or n=(4e-1)(1) 
    // So even large e can produce small n, meaning we need a much larger max_e!
    //
    // For a = 5e-1: n = (4e-1)(1) = 4e-1
    // So 4e-1 < limit => e < (limit+1)/4 = 250000
    // That's our true upper bound!

    let max_e = (limit + 1) / 4;  // ~250000

    for e in 1..=max_e {
        // a ranges from 2e+1 to 5e-1
        for a in (2 * e + 1)..(5 * e) {
            let n = (a - e) * (5 * e - a);
            if n == 0 || n >= limit {
                continue;
            }
            counts[n] += 1;
        }
    }

    // Verify known values
    println!("Solutions for n=27: {}", counts[27]);
    println!("Solutions for n=1155: {}", counts[1155]);

    let result = counts.iter().filter(|&&c| c == 10).count();
    println!("Count of n < 1,000,000 with exactly 10 solutions: {}", result);
}
