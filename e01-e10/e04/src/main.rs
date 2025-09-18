use euler::check_palindrome_i32;

fn main() {
    let mut biggest = 0;

    for i in 100..999 {
        for j in 100..999 {
            let test = i * j;
            if check_palindrome_i32(test) {
                if test > biggest {
                    biggest = test;
                }
            }
        }
    }
    println!("Largest palindome as the product of two three digit numbers:");
    println!("{}", biggest);
}

