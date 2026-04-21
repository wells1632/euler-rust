use rug::{Float, Integer};
use rug::ops::Pow;
use std::io;

fn sum_of_fractional_digits(n: u32, d: u32) -> u32 {
    let precision_bits = (d as f64 * 3.33 + 64.0) as u32;

    let n_float = Float::with_val(precision_bits, n);
    let sqrt_n = n_float.sqrt();

    let n_int = Integer::from(n);
    let sqrt_int = n_int.clone().sqrt();
    if sqrt_int.clone() * sqrt_int.clone() == n_int {
        return 0;
    }

    let floor_val = sqrt_n.clone().floor();
    let frac = sqrt_n - floor_val;

    let ten_pow = Float::with_val(precision_bits, 10u32).pow(d);
    let shifted = frac * ten_pow;

    let s = format!("{:.0}", shifted.floor());
    let padded = format!("{:0>width$}", s, width = d as usize);
    let digits: &str = &padded[..d as usize];

    digits
        .chars()
        .map(|c| c.to_digit(10).unwrap_or(0))
        .sum()
}

fn main() {
    println!("Enter n (the number to square root):");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: u32 = input.trim().parse().expect("Please enter a valid integer");

    println!("Enter d (number of fractional digits to sum):");
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let d: u32 = input.trim().parse().expect("Please enter a valid integer");

    let result = sum_of_fractional_digits(n, d);
    println!("S({}, {}) = {}", n, d, result);
}
