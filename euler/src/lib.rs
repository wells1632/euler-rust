pub fn check_prime_i64(n: i64) -> bool {
    let mut i = 2;
    while i*i<=n {
        if n%i == 0 {
            return false;
        }
        i+=1;
    }
    return true;
}

pub fn check_palindrome_i32(n: i32) -> bool {
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
