fn main() {
    for a in 1..1000 {
        for b in a..1000 {
            let c = 1000 - a - b;
            
            if c > b && a * a + b * b == c * c {
                println!("Found triplet: a={}, b={}, c={}", a, b, c);
                println!("Product abc = {}", a * b * c);
                return;
            }
        }
    }
}
