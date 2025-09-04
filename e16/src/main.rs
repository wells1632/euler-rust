fn main() {
    // What are the sum of the numbers in the value of 2^1000?

    // Naive approach does not work due to overflow:
    /*
    let mut n: i64 = 1;
    for i in 1..1001 {
        n = n * 2;
    }
    println!("{}", n);
     */
    // So let's try a vector instead:

    use std::io::{stdin, stdout, Write};
    let mut s=String::new();
    let mut result: Vec<i32> = vec![1i32];
    let mut answer: i32 = 0;

    print!("Enter power of 2 you wish to see: ");
    let _=stdout().flush();
    stdin().read_line(&mut s).expect("Did not enter a correct string");
    if let Some('\n')=s.chars().next_back() {
        s.pop();
    }
    if let Some('\r')=s.chars().next_back() {
        s.pop();
    }

    for _i in 1..=s.parse().expect("Not an integer") {
        for j in (0..result.len()).rev() {
            result[j] = result[j]*2;
            if result[j] > 9 {
                if j == result.len()-1 {
                    result.push(1);
                    result[j] = result[j]-10;

                } else {
                    result[j] = result[j]-10;
                    result[j+1]+=1;
                }
            }
        }
    }

    for i in result.iter() {
        answer+=i;
    }
    
    println!("Answer: {}  Size of Array: {}", answer, result.len());
/*
    for (i, x) in result.iter().enumerate() {
        println!("In position {} we have value {}", i, x);
    }
*/
}
