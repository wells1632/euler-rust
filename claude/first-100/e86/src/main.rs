use std::collections::HashSet;
use std::collections::HashMap;

type Pair = (usize, usize);

// Theory: each valid cuboid maps onto a Pythagorean triple, as can be easily
// seen if the cuboid is drawn as a net. The longest corner-to-corner distance
// of the cuboid is the hypotenuse, with the other two triangle edges being the
// largest cuboid edge and the sum of the smaller two cuboid edges. We generate
// all triples up to a limit (or more accurately doubles; we don't need the
// hypotenuse) and count the cuboids they generate.

fn find_smallest_fac(n: usize) -> usize {
    for i in 2..=(n as f64).sqrt() as usize {
        if (n / i)*i == n {
            return i;
        }
    }
    n
}

fn prime_facs(mut n: usize, smallest_fac: &[usize]) -> HashSet<usize> {
    let mut facs: HashSet<usize> = HashSet::new();
    while n > 1 {
        facs.insert(smallest_fac[n]);
        n /= smallest_fac[n];
    }
    facs
}

fn find_coprimes(n: usize, smallest_fac: &[usize]) -> HashSet<usize> {
    let mut not_coprime: HashSet<usize> = HashSet::new();
    for p in prime_facs(n, smallest_fac) {
        for k in 1..(n / p) {
            not_coprime.insert(p*k);
        }
    }
    (1..n)
        .collect::<HashSet<usize>>()
        .difference(&not_coprime)
        .map(|&x| x)
        .collect::<HashSet<usize>>()
}

// Count the cuboids mapping to a triangle with the given lengths (adj and opp,
// in no particular order). The cuboids' max edge length will always be s1.
// Cuboids with a greater max edge length are explicitly not counted.
// Geometrically, s2 is the x+y lengths of the cuboid and s1 is the z length.
fn cuboids_from_triangle(s1: usize, s2: usize) -> usize {
    if s1 + 1 >= s2 {
        return s2 / 2;
    }
    if 2*s1 < s2 {
        return 0;
    }
    (s2 / 2) + s1 + 1 - s2
}

fn pairs_from_seed(m: usize, k: usize, coprimes: &HashSet<usize>) -> Vec<Pair> {
    let mut pairs: Vec<Pair> = Vec::new();
    for n in coprimes {
        if (n + m) % 2 == 1 {
            let a: usize = m*m - n*n;
            let b: usize = 2*m*n;
            pairs.push((k*a, k*b));
            pairs.push((k*b, k*a));
        }
    }
    pairs
}

fn solve(target: usize) -> usize {
    let mut tally: usize = 0;
    let mut lim: usize = 0;
    let mut coprimes: Vec<HashSet<usize>> = Vec::new();
    let mut k_cache: Vec<usize> = Vec::new();
    let mut pair_cache: HashMap<usize, usize> = HashMap::new();
    let smallest_fac: &mut Vec<usize> = &mut Vec::new();

    while tally < target {
        lim += 1;
        let m_max: usize = (lim + 1) / 2;
        if m_max >= smallest_fac.len() {
            smallest_fac.push(find_smallest_fac(m_max));
        }
        if m_max >= coprimes.len() {
            coprimes.push(find_coprimes(m_max, smallest_fac));
        }
        if m_max >= k_cache.len() {
            k_cache.push(0);
        }
        for m in 2..=m_max {
            let k_new: usize = lim / (2*m - 1);
            if k_new == k_cache[m] {
                continue;
            }
            k_cache[m] = k_new;
            for pair in pairs_from_seed(m, k_new, &coprimes[m]) {
                let cube_count: usize = cuboids_from_triangle(pair.0, pair.1);
                if cube_count > 0 {
                    *pair_cache.entry(pair.0).or_insert(0) += cube_count;
                }
            }
        }
        tally += *pair_cache.get(&lim).unwrap_or(&0);
        pair_cache.remove(&lim);
    }
    lim
}

fn main() {
    println!("{}", solve(1_000_000));

}
