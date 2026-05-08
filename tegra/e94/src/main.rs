fn main() {
    let limit = 1_000_000_000u64;
    let mut total_sum = 0u128;
    let mut count = 0;

    println!("Finding almost equilateral triangles (Pell equation method)");
    println!("Perimeter <= {}\n", limit);

    // Solutions come in two families, seeded separately.
    // For (a, a, b) triangles with integer area, valid 'a' values satisfy
    // the recurrence: a_next = 4*a_prev - a_prev_prev
    // Family 1: (a, a, a-1) — seeds a=2 (triangle 2,2,1, area=sqrt(15)/4 — skip), a=4
    // Family 2: (a, a, a+1) — seeds start at a=3 (triangle 3,3,4, perimeter=10)

    // We track both families with their recurrences simultaneously.
    // Verified seed pairs (prev, curr) for each family:
    let families: &[(u64, u64, i64)] = &[
        (1, 4, -1),  // (a,a,a-1) family: next = 4*curr - prev
        (2, 3,  1),  // (a,a,a+1) family: next = 4*curr - prev
    ];

    let mut results = Vec::new();

    for &(seed_prev, seed_curr, delta) in families {
        let mut prev = seed_prev;
        let mut curr = seed_curr;

        loop {
            let b = (curr as i64 + delta) as u64;
            let perimeter = curr + curr + b;
            if perimeter > limit {
                break;
            }
            results.push((curr, b, perimeter));
            let next = 4 * curr - prev;
            prev = curr;
            curr = next;
        }
    }

    results.sort_by_key(|&(_, _, p)| p);

    for (a, b, perimeter) in &results {
        println!("  ({}, {}, {}), perimeter = {}", a, a, b, perimeter);
        total_sum += *perimeter as u128;
        count += 1;
    }

    println!("\n{}", "=".repeat(70));
    println!("RESULT:");
    println!("Total triangles found: {}", count);
    println!("Sum of all perimeters: {}", total_sum);
    println!("{}", "=".repeat(70));
}
