fn main() {
    let result = (1..=20).fold(1, |acc, n| lcm(acc, n));
    
    println!("The smallest number evenly divisible by all numbers from 1 to 20 is: {}", result);
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}
