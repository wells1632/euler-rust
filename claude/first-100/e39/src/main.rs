use std::collections::HashMap;

fn gcd(a: u32, b: u32) -> u32 {
    if b == 0 { a } else { gcd(b, a % b) }
}

fn find_perimeter_with_max_solutions() -> (u32, u32, Vec<(u32, u32, u32)>) {
    let mut perimeter_solutions: HashMap<u32, Vec<(u32, u32, u32)>> = HashMap::new();

    // Generate all Pythagorean triples with perimeter < 1000
    for m in 2..32 {  // m^2 + n^2 < 1000, so m < ~32
	for n in 1..m {
	    if (m - n) % 2 == 1 && gcd(m, n) == 1 {
		// Generate primitive triple
		let a = m * m - n * n;
		let b = 2 * m * n;
		let c = m * m + n * n;

		// Scale up the primitive triple
		let mut k = 1;
		while k * (a + b + c) < 1000 {
		    let scaled_a = k * a;
		    let scaled_b = k * b;
		    let scaled_c = k * c;
		    let perimeter = scaled_a + scaled_b + scaled_c;

		    // Store triangle (sorted for consistency)
		    let mut triangle = [scaled_a, scaled_b, scaled_c];
		    triangle.sort();

		    perimeter_solutions
			.entry(perimeter)
			.or_insert_with(Vec::new)
			.push((triangle[0], triangle[1], triangle[2]));

		    k += 1;
		}
	    }
	}
    }

    // Find perimeter with maximum number of solutions
    let (max_perimeter, max_solutions) = perimeter_solutions
	.iter()
	.max_by_key(|(_, solutions)| solutions.len())
	.unwrap();

    (*max_perimeter, max_solutions.len().try_into().unwrap(), max_solutions.clone())
}

fn main() {
    let (perimeter, count, solutions) = find_perimeter_with_max_solutions();

    println!("Perimeter with maximum number of right triangle solutions: {}", perimeter);
    println!("Number of solutions: {}", count);
    println!("Solutions:");

    for (i, (a, b, c)) in solutions.iter().enumerate() {
	println!("  {}: ({}, {}, {}) - verification: {}² + {}² = {}, {}² = {}",
		 i + 1, a, b, c, a, b, a*a + b*b, c, c*c);
    }
}
