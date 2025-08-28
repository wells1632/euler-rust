pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

pub fn check_prime(n: i64) -> bool {
    let mut i = 2;
    while i*i<=n {
        if n%i == 0 {
            return false;
        }
        i+=1;
    }
    return true;
}


