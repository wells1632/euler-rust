fn main() {
    let mut total = 0;
    for i in 1..1000 {
        if i % 3 == 0 {
            total += i;
        }
        if i % 5 == 0 {
            total += i;
        }
        if i % 15 == 0 {
            total -= i;
        }
    }
    println!("The sum of all multiples of three and five under one thousand");
    println!("Total: {}", total);
}
