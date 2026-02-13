fn main() {
    println!("Special Sum Set - Minimal Equality Tests Required\n");
    println!("Counting pairs where ordering can't be determined from sorted property...\n");
    
    // Verify known values
    println!("Verification:");
    println!("n=4: {} pairs (expected: 1)", count_tests_needed(4));
    println!("n=7: {} pairs (expected: 70)", count_tests_needed(7));
    
    println!("\nCalculating for n=12:");
    let result = count_tests_needed(12);
    
    println!("\n{}", "=".repeat(70));
    println!("ANSWER: For n=12, {} subset pairs need to be tested", result);
    println!("{}", "=".repeat(70));
}

fn count_tests_needed(n: usize) -> u64 {
    // Generate all pairs of disjoint subsets of equal size
    // Count only those where we can't determine ordering from element-wise comparison
    
    let mut total = 0u64;
    
    // For each subset size k
    for k in 1..=n/2 {
        // Generate all k-element subsets from n elements
        let subsets = generate_k_subsets(n, k);
        
        // Count pairs where ordering can't be determined
        let mut count = 0u64;
        
        for i in 0..subsets.len() {
            for j in i+1..subsets.len() {
                let b = &subsets[i];
                let c = &subsets[j];
                
                // Check if disjoint
                if are_disjoint(b, c) {
                    // Check if ordering can't be determined
                    if !can_determine_order(b, c) {
                        count += 1;
                    }
                }
            }
        }
        
        if count > 0 {
            println!("  k={}: {} pairs need testing", k, count);
        }
        
        total += count;
    }
    
    println!("  Total: {}", total);
    total
}

fn generate_k_subsets(n: usize, k: usize) -> Vec<Vec<usize>> {
    let mut subsets = Vec::new();
    let mut current = Vec::new();
    generate_k_subsets_helper(n, k, 0, &mut current, &mut subsets);
    subsets
}

fn generate_k_subsets_helper(
    n: usize,
    k: usize,
    start: usize,
    current: &mut Vec<usize>,
    result: &mut Vec<Vec<usize>>
) {
    if current.len() == k {
        result.push(current.clone());
        return;
    }
    
    for i in start..n {
        current.push(i);
        generate_k_subsets_helper(n, k, i + 1, current, result);
        current.pop();
    }
}

fn are_disjoint(a: &[usize], b: &[usize]) -> bool {
    for &x in a {
        if b.contains(&x) {
            return false;
        }
    }
    true
}

fn can_determine_order(b: &[usize], c: &[usize]) -> bool {
    // If every element in b is less than corresponding element in c, we know B < C
    // If every element in b is greater than corresponding element in c, we know B > C
    
    let mut b_sorted = b.to_vec();
    let mut c_sorted = c.to_vec();
    b_sorted.sort();
    c_sorted.sort();
    
    let all_b_less = b_sorted.iter().zip(c_sorted.iter()).all(|(bi, ci)| bi < ci);
    let all_b_greater = b_sorted.iter().zip(c_sorted.iter()).all(|(bi, ci)| bi > ci);
    
    all_b_less || all_b_greater
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_known_values() {
        assert_eq!(count_tests_needed(4), 1);
        assert_eq!(count_tests_needed(7), 70);
    }
}
