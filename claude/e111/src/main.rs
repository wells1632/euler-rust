use std::collections::HashSet;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let length = if args.len() > 1 {
        match args[1].parse::<usize>() {
            Ok(n) if n > 0 => n,
            Ok(_) => {
                eprintln!("Error: Length must be greater than 0");
                std::process::exit(1);
            }
            Err(_) => {
                eprintln!("Error: Invalid number format");
                eprintln!("Usage: {} <length>", args[0]);
                eprintln!("Example: {} 10", args[0]);
                std::process::exit(1);
            }
        }
    } else {
        println!("Usage: {} <length>", args[0]);
        println!("Using default length of 10");
        10
    };
    
    println!("Finding primes with maximum repeated digits for length = {}\n", length);
    
    let result = pe111(length);
    
    println!("\n{}", "=".repeat(70));
    println!("ANSWER for length {}: {}", length, result);
    println!("{}", "=".repeat(70));
}

fn pe111(length: usize) -> u128 {
    let digits: HashSet<u8> = (0..=9).collect();
    let mut total = 0u128;
    
    for d in 0..=9 {
        println!("Processing digit {}...", d);
        
        let mut primes = HashSet::new();
        
        // Create base number (all d's)
        let base = create_base_number(d, length);
        let mut m: usize = 0;
        
        // Special case: if base is prime
        if d == 1 && is_prime(base) {
            primes.insert(base);
        }
        
        while primes.is_empty() {
            m += 1;
            
            // Determine fixed positions based on digit
            let fixed_positions = get_fixed_positions(d, length);
            let num_fixed = fixed_positions.len();
            let remaining = m.saturating_sub(num_fixed);
            
            if remaining == 0 {
                // Use only fixed positions
                generate_primes_fixed(
                    &mut primes,
                    base,
                    d,
                    &fixed_positions,
                    &digits,
                    length,
                );
            } else {
                // Need to select additional positions
                let available: Vec<usize> = (0..length)
                    .filter(|&pos| !fixed_positions.contains(&pos))
                    .collect();
                
                // Generate combinations of positions
                for combo in combinations(&available, remaining) {
                    let mut all_positions = fixed_positions.clone();
                    all_positions.extend(combo);
                    
                    generate_primes_fixed(
                        &mut primes,
                        base,
                        d,
                        &all_positions,
                        &digits,
                        length,
                    );
                }
            }
        }
        
        let sum: u128 = primes.iter().sum();
        println!("  Digit {}: M={}, N={}, S={}", d, m - 1, primes.len(), sum);
        total += sum;
    }
    
    total
}

fn create_base_number(digit: u8, length: usize) -> u128 {
    let mut num = 0u128;
    for i in 0..length {
        num += (digit as u128) * 10u128.pow(i as u32);
    }
    num
}

fn get_fixed_positions(digit: u8, length: usize) -> Vec<usize> {
    match digit {
        0 => vec![0, length - 1],
        2 | 4 | 5 | 6 | 8 => vec![0],
        _ => vec![],
    }
}

fn generate_primes_fixed(
    primes: &mut HashSet<u128>,
    base: u128,
    d: u8,
    positions: &[usize],
    all_digits: &HashSet<u8>,
    length: usize,
) {
    let other_digits: Vec<u8> = all_digits
        .iter()
        .filter(|&&x| x != d)
        .copied()
        .collect();
    
    let num_positions = positions.len();
    
    // Generate all products of other_digits
    generate_products(&other_digits, num_positions, &mut |values| {
        // Build the number
        let mut p = base;
        
        for (i, &pos) in positions.iter().enumerate() {
            let new_digit = values[i];
            if new_digit != 0 || pos != length - 1 {
                let old_contrib = (d as u128) * 10u128.pow(pos as u32);
                let new_contrib = (new_digit as u128) * 10u128.pow(pos as u32);
                p = p - old_contrib + new_contrib;
            }
        }
        
        // Check divisibility by 3
        let sum_replaced: u32 = values.iter().map(|&x| x as u32).sum();
        let remaining_d = length - num_positions;
        let digit_sum = (d as u32 * remaining_d as u32) + sum_replaced;
        
        if digit_sum % 3 != 0 && is_prime(p) {
            primes.insert(p);
        }
    });
}

fn generate_products<F>(digits: &[u8], length: usize, callback: &mut F)
where
    F: FnMut(&[u8]),
{
    let mut current = vec![0u8; length];
    generate_products_helper(digits, length, 0, &mut current, callback);
}

fn generate_products_helper<F>(
    digits: &[u8],
    length: usize,
    pos: usize,
    current: &mut Vec<u8>,
    callback: &mut F,
)
where
    F: FnMut(&[u8]),
{
    if pos == length {
        callback(current);
        return;
    }
    
    for &digit in digits {
        current[pos] = digit;
        generate_products_helper(digits, length, pos + 1, current, callback);
    }
}

fn combinations(items: &[usize], k: usize) -> Vec<Vec<usize>> {
    let mut result = Vec::new();
    let mut current = Vec::new();
    combinations_helper(items, k, 0, &mut current, &mut result);
    result
}

fn combinations_helper(
    items: &[usize],
    k: usize,
    start: usize,
    current: &mut Vec<usize>,
    result: &mut Vec<Vec<usize>>,
) {
    if current.len() == k {
        result.push(current.clone());
        return;
    }
    
    for i in start..items.len() {
        current.push(items[i]);
        combinations_helper(items, k, i + 1, current, result);
        current.pop();
    }
}

fn is_prime(n: u128) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 || n == 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    
    // Miller-Rabin with deterministic witnesses for u128
    let witnesses = [2u128, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];
    
    let mut d = n - 1;
    let mut r = 0;
    while d % 2 == 0 {
        d /= 2;
        r += 1;
    }
    
    'witness: for &a in &witnesses {
        if a >= n {
            continue;
        }
        
        let mut x = mod_pow(a, d, n);
        
        if x == 1 || x == n - 1 {
            continue 'witness;
        }
        
        for _ in 0..r - 1 {
            x = mod_mul(x, x, n);
            if x == n - 1 {
                continue 'witness;
            }
        }
        
        return false;
    }
    
    true
}

fn mod_mul(a: u128, b: u128, m: u128) -> u128 {
    ((a as u128).wrapping_mul(b as u128)) % m
}

fn mod_pow(mut base: u128, mut exp: u128, modulus: u128) -> u128 {
    let mut result = 1u128;
    base %= modulus;
    
    while exp > 0 {
        if exp % 2 == 1 {
            result = mod_mul(result, base, modulus);
        }
        base = mod_mul(base, base, modulus);
        exp /= 2;
    }
    
    result
}
