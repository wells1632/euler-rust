fn main() {
    let test = false;
    let mut answer = 20;
    while test == false {
        let mut bong = false;
        for i in 1..=20 {
            if answer % i != 0 {
                bong = true;
            }
        }
        if !bong {
            break;
        }
        answer += 20;
    }
    println!("{}", answer);
}
