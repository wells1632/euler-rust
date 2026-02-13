fn evaluate_polynomial(n: i64) -> i64 {
    // u = 1 - n + n² - n³ + n^4 - n^5 + n^6 - n^7 + n^8 - n^9 + n^10
    let mut result = 1i64;
    let mut power = 1i64;
    
    for i in 1..=10 {
        power *= n;
        if i % 2 == 1 {
            result -= power;
        } else {
            result += power;
        }
    }
    
    result
}

fn forward_differences(terms: &[i64]) -> Vec<i64> {
    if terms.len() <= 1 {
        return terms.to_vec();
    }
    
    let mut diffs = Vec::new();
    for i in 0..terms.len() - 1 {
        diffs.push(terms[i + 1] - terms[i]);
    }
    diffs
}

fn interpolate(terms: &[i64], n: usize) -> i64 {
    // Use forward differences to compute OP(k, n)
    let mut diff_table = vec![terms.to_vec()];
    
    // Build the difference table
    let mut current = terms.to_vec();
    while current.len() > 1 {
        current = forward_differences(&current);
        diff_table.push(current.clone());
    }
    
    // Use Newton's forward difference formula
    let mut result = diff_table[0][0];
    let mut binomial = 1i64;
    let t = n as i64 - 1;
    
    for i in 1..diff_table.len() {
        binomial = binomial * (t - (i as i64 - 1)) / i as i64;
        result += binomial * diff_table[i][0];
    }
    
    result
}

fn main() {
    // Generate the first 11 terms of the sequence
    let mut sequence = Vec::new();
    for n in 1..=11 {
        sequence.push(evaluate_polynomial(n));
    }
    
    println!("Original sequence:");
    for (i, term) in sequence.iter().enumerate() {
        println!("u({}) = {}", i + 1, term);
    }
    println!();
    
    let mut fit_sum = 0i64;
    
    // For each k from 1 to 10
    for k in 1..=10 {
        let first_k_terms: Vec<i64> = sequence.iter().take(k).copied().collect();
        let op_k_plus_1 = interpolate(&first_k_terms, k + 1);
        let actual_k_plus_1 = sequence[k];
        
        println!("k = {}: OP({},{}) = {}, actual u({}) = {}", 
                 k, k, k + 1, op_k_plus_1, k + 1, actual_k_plus_1);
        
        if op_k_plus_1 != actual_k_plus_1 {
            println!("  -> BOP! FIT = {}", op_k_plus_1);
            fit_sum += op_k_plus_1;
        }
        println!();
    }
    
    println!("Sum of FITs for BOPs: {}", fit_sum);
}
