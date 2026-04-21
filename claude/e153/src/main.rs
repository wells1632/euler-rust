fn gcd(a: usize, b: usize) -> usize {
    if b == 0 { a } else { gcd(b, a % b) }
}

fn main() {
    const LIMIT: usize = 100_000_000;

    let mut answer = vec![0i64; LIMIT + 1];

    // b = 0 case: a divides n iff a | n
    for a in 1..=LIMIT {
        let mut multiple = a;
        while multiple <= LIMIT {
            answer[multiple] += a as i64;
            multiple += a;
        }
    }

    // b != 0 case: primitive pairs (p, q) with p > 0, q > 0, gcd(p,q) = 1
    // contribution is 2*d*p to all multiples of d*(p^2+q^2)
    let max_pq = ((LIMIT as f64).sqrt() as usize) + 1;

    for pp in 1..=max_pq {
        for qq in 1..=max_pq {
            let norm = pp * pp + qq * qq;
            if norm > LIMIT { break; }
            if gcd(pp, qq) != 1 { continue; }

            let mut d = 1usize;
            while d * norm <= LIMIT {
                let step = d * norm;
                let contrib = 2 * (d * pp) as i64;
                let mut multiple = step;
                while multiple <= LIMIT {
                    answer[multiple] += contrib;
                    multiple += step;
                }
                d += 1;
            }
        }
    }

    let total: i64 = answer[1..=LIMIT].iter().sum();
    println!("{}", total);
}
