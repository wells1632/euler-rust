fn main() {
    let mut collatz = 0;
    let mut collatz2 = 0;
    let mut count = 1;
    for i in 1..=1000000 {
        let mut boof: i64 = i as i64;
        while boof > 1 {
            count += 1;
            if boof % 2 == 0 {
                boof = boof / 2;
            } else {
                boof = boof * 3 + 1;
            }
        }
        if count > collatz {
            collatz = count;
            collatz2 = i;
        }
        count = 1;
    }

    println!("{}", collatz2);
}
