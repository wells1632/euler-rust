fn main() {
    // C(26, n) * (2^n - n - 1)
    // Eulerian number A(n,1) = 2^n - n - 1

    let mut max_p: u64 = 0;
    let mut max_n: usize = 0;

    for n in 2..=26usize {
        let eulerian = (1u64 << n) - n as u64 - 1;
        let binom = binomial(26, n);
        let p = binom * eulerian;
        println!("n={:2}, C(26,n)={:10}, A(n,1)={:10}, p(n)={}", n, binom, eulerian, p);
        if p > max_p {
            max_p = p;
            max_n = n;
        }
    }

    println!("\nMaximum p(n) = {} at n = {}", max_p, max_n);
}

fn binomial(n: usize, k: usize) -> u64 {
    if k > n { return 0; }
    let k = k.min(n - k);
    let mut result = 1u64;
    for i in 0..k {
        result = result * (n - i) as u64 / (i + 1) as u64;
    }
    result
}