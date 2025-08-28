fn main() {
    let mut max = 1;
    let mut prev = 0;
    let mut ans = 0;
    while max < 40000000 {
        if max == 1 {
            prev = 1;
        }
        let new = max + prev;
        if new % 2 == 0 && new < 4000000 {
            ans += new;
        }
        prev = max;
        max = new;
    }
    println!("The sum of even-valued terms in the Fibonacci sequence less than four million");
    println!("Total: {}", ans);
}
