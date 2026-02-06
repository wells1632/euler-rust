fn triangle(n: u64) -> u64 {
    n * (n + 1) / 2
}

fn pentagonal(n: u64) -> u64 {
    n * (3 * n - 1) / 2
}

fn hexagonal(n: u64) -> u64 {
    n * (2 * n - 1)
}

fn is_triangle(x: u64) -> bool {
    // For a number to be triangular: n = (-1 + sqrt(1 + 8x)) / 2
    let discriminant = 1 + 8 * x;
    let sqrt_disc = (discriminant as f64).sqrt();

    if sqrt_disc.fract() != 0.0 {
	return false;
    }

    let sqrt_disc = sqrt_disc as u64;
    sqrt_disc >= 1 && (sqrt_disc - 1) % 2 == 0
}

fn is_pentagonal(x: u64) -> bool {
    // For a number to be pentagonal: n = (1 + sqrt(1 + 24x)) / 6
    let discriminant = 1 + 24 * x;
    let sqrt_disc = (discriminant as f64).sqrt();

    if sqrt_disc.fract() != 0.0 {
	return false;
    }

    let sqrt_disc = sqrt_disc as u64;
    sqrt_disc >= 1 && (1 + sqrt_disc) % 6 == 0
}

fn main() {
    println!("Finding numbers that are triangle, pentagonal, and hexagonal...");

    let mut count = 0;
    let mut n = 1;

    loop {
	let hex = hexagonal(n);

	if is_triangle(hex) && is_pentagonal(hex) {
	    count += 1;
	    println!("Found #{}: {}", count, hex);

	    // Find which triangle and pentagonal numbers this corresponds to
	    let t_n = ((-1.0 + (1.0 + 8.0 * hex as f64).sqrt()) / 2.0) as u64;
	    let p_n = ((1.0 + (1.0 + 24.0 * hex as f64).sqrt()) / 6.0) as u64;

	    println!("  Triangle T{} = {}", t_n, triangle(t_n));
	    println!("  Pentagonal P{} = {}", p_n, pentagonal(p_n));
	    println!("  Hexagonal H{} = {}", n, hex);

	    // Verify
	    println!("  Verification: T{} = {}, P{} = {}, H{} = {}",
		     t_n, triangle(t_n),
		     p_n, pentagonal(p_n),
		     n, hexagonal(n));
	    println!();

	    if count == 3 {
		break;
	    }
	}

	n += 1;

	// Safety check to avoid infinite loop
	if n > 1000000 {
	    println!("Searched up to n = {}, stopping.", n);
	    break;
	}
    }

    if count < 3 {
	println!("Only found {} number(s) in the search range.", count);
    }
}
