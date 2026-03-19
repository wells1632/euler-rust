fn main() {
    let limit = 50_000_000usize;
    let mut counts = vec![0u32; limit];

    // n = u*v where:
    // u+v ≡ 0 (mod 4)
    // v < 3u  (ensures z >= 1)
    // u >= 1, v >= 1

    for u in 1..limit {
        let r = u % 4;
        let target_v_mod4 = (4 - r) % 4;
        let v_start = if target_v_mod4 == 0 { 4 } else { target_v_mod4 };

        // No early break - just skip if smallest v already exceeds limit
        if u * v_start >= limit {
            continue;
        }

        let mut v = v_start;
        while v < 3 * u && u * v < limit {
            counts[u * v] += 1;
            v += 4;
        }
    }

    // Verify
    println!("Solutions for n=20: {}", counts[20]);
    println!("Unique solutions below 100: {}", counts[1..100].iter().filter(|&&c| c == 1).count());

    let result = counts.iter().filter(|&&c| c == 1).count();
    println!("Count of n < 50,000,000 with exactly 1 solution: {}", result);
}
