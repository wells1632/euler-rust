fn main() {
    let mut biggest = 0;

    for i in 100..999 {
        for j in 100..999 {
            let test = i * j;
            if check_palindrome(test) {
                if test > biggest {
                    biggest = test;
                }
            }
        }
    }
    println!("Largest palindome as the product of two three digit numbers:");
    println!("{}", biggest);
}

fn check_palindrome(n: i32) -> bool {
    let s: String = n.to_string();
    let mut r: String = "".to_string();


    for i in s.chars() {
        r.insert_str(0,&i.to_string());
    }
    if s == r {
        return true;
    } else {
        return false;
    }
}
