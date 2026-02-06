fn main() {
    let n = 100;
    
    let sum_of_squares: u64 = (1..=n).map(|x| x * x).sum();
    let sum: u64 = (1..=n).sum();
    let square_of_sum = sum * sum;
    
    let difference = square_of_sum - sum_of_squares;
    
    println!("Sum of squares: {}", sum_of_squares);
    println!("Square of sum: {}", square_of_sum);
    println!("Difference: {}", difference);
}
