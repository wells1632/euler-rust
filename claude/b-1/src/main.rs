fn zeta_neg1() -> f64 {
    -1.0 / 12.0
}

fn main() {
    let sum_mult3  =  3.0 * zeta_neg1();
    let sum_mult5  =  5.0 * zeta_neg1();
    let sum_mult15 = 15.0 * zeta_neg1();

    let result = sum_mult3 + sum_mult5 - sum_mult15;

    println!("Sum of multiples of 3:  {:.6}", sum_mult3);
    println!("Sum of multiples of 5:  {:.6}", sum_mult5);
    println!("Sum of multiples of 15: {:.6}", sum_mult15);
    println!("Total (inclusion-exclusion): {:.6}", result);
    println!("As a fraction: 7/12 = {:.6}", 7.0_f64 / 12.0);
}
