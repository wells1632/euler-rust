fn main() {
    let limit = 10_000_000;
    let mut memo = vec![0u8; limit];
    
    // Pre-set known values
    memo[1] = 1;
    memo[89] = 89;
    
    let count = count_numbers_reaching_89(limit, &mut memo);
    println!("Numbers below {} that reach 89: {}", limit, count);
}

fn sum_of_squared_digits(mut n: usize) -> usize {
    let mut sum = 0;
    while n > 0 {
        let digit = n % 10;
        sum += digit * digit;
        n /= 10;
    }
    sum
}

fn find_destination(n: usize, memo: &mut [u8]) -> u8 {
    if n < memo.len() && memo[n] != 0 {
        return memo[n];
    }
    
    let next = sum_of_squared_digits(n);
    let result = find_destination(next, memo);
    
    // Store result if within array bounds
    if n < memo.len() {
        memo[n] = result;
    }
    
    result
}

fn count_numbers_reaching_89(limit: usize, memo: &mut [u8]) -> usize {
    let mut count = 0;
    
    for i in 1..limit {
        let destination = find_destination(i, memo);
        if destination == 89 {
            count += 1;
        }
        
        // Progress indicator
        if i % 1_000_000 == 0 {
            println!("Processed: {} million, Count so far: {}", i / 1_000_000, count);
        }
    }
    
    count
}
