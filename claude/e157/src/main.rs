fn count_solutions(n: u32) -> u64 {
    let pow10: u64 = 10u64.pow(n);
    let mut count = 0u64;

    // a <= b, and 1/a + 1/b >= 1/10^n with a <= b means 2/a >= 1/10^n => a <= 2*10^n
    for a in 1..=(2 * pow10) {
        // 1/b = p/10^n - 1/a > 0 => p/10^n > 1/a => p >= 1, and b >= a
        // b >= a means 1/b <= 1/a means p/10^n - 1/a <= 1/a => p/10^n <= 2/a => p <= 2*10^n/a
        // Also b = 1/(p/10^n - 1/a) = a*10^n / (a*p - 10^n)
        // For b to be a positive integer: (a*p - 10^n) | a*10^n and a*p > 10^n

        let min_p = pow10 / a + 1; // a*p > 10^n => p > 10^n/a
        let max_p = 2 * pow10 / a; // p <= 2*10^n/a (from b >= a)

        for p in min_p..=max_p {
            let numerator = a * pow10;
            let denom = a * p - pow10;
            if denom > 0 && numerator % denom == 0 {
                count += 1;
            }
        }
    }
    count
}

fn main() {
    let mut total = 0u64;
    for n in 1..=9 {
        let c = count_solutions(n);
        println!("n={}: {} solutions", n, c);
        total += c;
    }
    println!("Total (1<=n<=9): {}", total);
}