fn main() {
    let mut sum_square = 0;
    let mut square_sum = 0;

    for i in 1..=100 {
        square_sum+=i;
        sum_square=sum_square + i*i;
    }
    square_sum = square_sum * square_sum;
    
    println!("{}",square_sum - sum_square);
}
