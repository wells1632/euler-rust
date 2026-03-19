fn main() {
    // Golden nuggets are products of consecutive even-indexed Fibonacci pairs:
    // GN(k) = F(2k) * F(2k+1)
    // We need the 15th, so we need F(30) * F(31)

    // Generate Fibonacci numbers up to F(31)
    let mut fibs: Vec<u128> = vec![0, 1];
    for i in 2..=32 {
        let next = fibs[i - 1] + fibs[i - 2];
        fibs.push(next);
    }

    // Golden nuggets: F(2)*F(3), F(4)*F(5), F(6)*F(7), ...
    // kth golden nugget = F(2k) * F(2k+1)
    let k: usize = 15;
    let golden_nugget = fibs[2 * k] * fibs[2 * k + 1];

    println!("Fibonacci numbers used: F({}) = {}, F({}) = {}",
        2 * k, fibs[2 * k],
        2 * k + 1, fibs[2 * k + 1]
    );
    println!("The 15th golden nugget is: {}", golden_nugget);
}
