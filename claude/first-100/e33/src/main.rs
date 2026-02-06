fn gcd(a: u32, b: u32) -> u32 {
    if b == 0 {
	a
    } else {
	gcd(b, a % b)
    }
}

fn main() {
    let mut valid_fractions = Vec::new();

    // Check all two-digit fractions
    for num in 10..100 {
	for den in (num + 1)..100 {
	    let num_tens = num / 10;
	    let num_ones = num % 10;
	    let den_tens = den / 10;
	    let den_ones = den % 10;

	    // Skip if any digit is 0 (to avoid trivial cases)
	    if num_ones == 0 || den_ones == 0 {
		continue;
	    }

	    let original_fraction = (num as f64) / (den as f64);

	    // Check if we can cancel the tens digits
	    if num_tens == den_tens && num_tens != 0 {
		let simplified = (num_ones as f64) / (den_ones as f64);
		if (original_fraction - simplified).abs() < 1e-10 {
		    valid_fractions.push((num, den));
		}
	    }

	    // Check if we can cancel the ones digits
	    if num_ones == den_ones {
		let simplified = (num_tens as f64) / (den_tens as f64);
		if (original_fraction - simplified).abs() < 1e-10 {
		    valid_fractions.push((num, den));
		}
	    }

	    // Check cross-cancellation: num_tens with den_ones
	    if num_tens == den_ones && den_ones != 0 {
		let simplified = (num_ones as f64) / (den_tens as f64);
		if (original_fraction - simplified).abs() < 1e-10 {
		    valid_fractions.push((num, den));
		}
	    }

	    // Check cross-cancellation: num_ones with den_tens
	    if num_ones == den_tens && den_tens != 0 {
		let simplified = (num_tens as f64) / (den_ones as f64);
		if (original_fraction - simplified).abs() < 1e-10 {
		    valid_fractions.push((num, den));
		}
	    }
	}
    }

    println!("Valid digit-cancelling fractions:");
    for (num, den) in &valid_fractions {
	println!("{}/{}", num, den);
    }

    // Calculate the product of all valid fractions
    let mut product_num = 1u32;
    let mut product_den = 1u32;

    for (num, den) in &valid_fractions {
	product_num *= num;
	product_den *= den;
    }

    println!("\nProduct: {}/{}", product_num, product_den);

    // Reduce to lowest terms
    let common_factor = gcd(product_num, product_den);
    let reduced_num = product_num / common_factor;
    let reduced_den = product_den / common_factor;

    println!("Reduced: {}/{}", reduced_num, reduced_den);
    println!("Denominator: {}", reduced_den);
}

