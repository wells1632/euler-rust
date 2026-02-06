use std::collections::HashMap;

fn main() {
    let limit = 10_000_000;
    let mut cache = HashMap::new();
    let count = count_numbers_reaching_89(limit, &mut cache);
    println!("Numbers below {} that reach 89: {}", limit, count);
}

fn sum_of_squared_digits(mut n: u32) -> u32 {
    let mut sum = 0;
    while n > 0 {
        let digit = n % 10;
        sum += digit * digit;
        n /= 10;
    }
    sum
}

fn reaches_89(n: u32, cache: &mut HashMap<u32, bool>) -> bool {
    if n == 1 {
        return false;
    }
    if n == 89 {
        return true;
    }
    
    if let Some(&result) = cache.get(&n) {
        return result;
    }
    
    let next = sum_of_squared_digits(n);
    let result = reaches_89(next, cache);
    cache.insert(n, result);
    result
}

fn count_numbers_reaching_89(limit: u32, cache: &mut HashMap<u32, bool>) -> u32 {
    let mut count = 0;
    for i in 1..limit {
        if reaches_89(i, cache) {
            count += 1;
        }
        // Progress indicator
        if i % 1_000_000 == 0 {
            println!("Processed: {} million", i / 1_000_000);
        }
    }
    count
}
