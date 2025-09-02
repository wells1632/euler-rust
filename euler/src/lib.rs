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

pub fn check_prime_i32(n: i32) -> bool {
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

pub fn get_factor_count_i32 (n:i32) -> i32 {
    let mut factor_count = 0i32;
    let nx = (n as f64).sqrt() as i32;
    for i in 1..=nx  {
        if n % i == 0 {
            factor_count+=2;
        }
    }
    if n==nx*nx {
        factor_count-=1;
    }
    return factor_count;
}
