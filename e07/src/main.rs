fn main() {
    use std::io::{stdin,stdout,Write};
    let mut s=String::new();
    let mut prime = 2;

    print!("Enter which prime you would like: ");
    let _=stdout().flush();
    stdin().read_line(&mut s).expect("Did not enter a correct string");
    if let Some('\n')=s.chars().next_back() {
        s.pop();
    }
    if let Some('\r')=s.chars().next_back() {
        s.pop();
    }
    println!("Finding {}'th prime...",s);

    let nth_prime: i32 = s.parse().expect("Failed to parse string to integer");
    let mut count = 0;

    while count < nth_prime {
        if euler::check_prime(prime) {
            count+=1;
        }
        prime+=1;
    }
    println!("The {}'th prime is: {}", s, prime-1);
}
