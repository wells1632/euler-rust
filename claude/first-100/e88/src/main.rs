use std::collections::HashSet;

fn find_factorizations(
    product: usize,
    sum: usize,
    num_factors: usize,
    min_factor: usize,
    k_limit: usize,
    min_ps: &mut Vec<usize>,
) {
    // We can add 1's to make sum equal product
    // If product = P and sum = S with F factors, we need (P - S) ones
    // Total factors k = F + (P - S)
    let k = num_factors + product - sum;
    
    if k >= 2 && k <= k_limit {
        if min_ps[k] == 0 || product < min_ps[k] {
            min_ps[k] = product;
        }
    }
    
    // Try adding more factors
    // Upper bound: we know minimal product-sum for k is at most 2k
    // So we don't need to go beyond 2 * k_limit
    for factor in min_factor.. {
        let new_product = product * factor;
        if new_product > 2 * k_limit {
            break;
        }
        find_factorizations(
            new_product,
            sum + factor,
            num_factors + 1,
            factor,
            k_limit,
            min_ps,
        );
    }
}

fn main() {
    let k_limit = 12_000;
    
    // Use a vector instead of HashMap for faster access
    // Index represents k value
    let mut min_ps = vec![0; k_limit + 1];
    
    println!("Finding minimal product-sum numbers for k = 2 to {}...", k_limit);
    
    // Start with product=1, sum=0, 0 factors, min_factor=2
    find_factorizations(1, 0, 0, 2, k_limit, &mut min_ps);
    
    // Check how many k values we found
    let found_count = min_ps.iter().skip(2).filter(|&&x| x > 0).count();
    println!("Found solutions for {} values of k (from k=2 to k={})", found_count, k_limit);
    
    println!("\nSome examples:");
    for k in [2, 3, 4, 5, 6, 7, 8, 9, 10, 12, 100, 1000].iter() {
        if *k <= k_limit && min_ps[*k] > 0 {
            println!("k = {}: minimal product-sum = {}", k, min_ps[*k]);
        }
    }
    
    // Get unique minimal product-sum numbers
    let unique_values: HashSet<usize> = min_ps.iter()
        .skip(2)  // Skip k=0 and k=1
        .filter(|&&x| x > 0)
        .copied()
        .collect();
    
    let sum: usize = unique_values.iter().sum();
    
    println!("\nTotal k values with solutions: {}", found_count);
    println!("Unique minimal product-sum numbers: {}", unique_values.len());
    println!("Sum of all UNIQUE minimal product-sum numbers: {}", sum);
    
    // Verify some duplicates
    let mut value_counts: std::collections::HashMap<usize, Vec<usize>> = std::collections::HashMap::new();
    for k in 2..=k_limit {
        if min_ps[k] > 0 {
            value_counts.entry(min_ps[k]).or_insert_with(Vec::new).push(k);
        }
    }
    
    println!("\nSome values that appear for multiple k:");
    let mut duplicates: Vec<_> = value_counts.iter()
        .filter(|(_, ks)| ks.len() > 1)
        .collect();
    duplicates.sort_by_key(|(v, _)| *v);
    
    for (value, ks) in duplicates.iter().take(10) {
        let mut k_list = (*ks).clone();
        k_list.sort_unstable();
        if k_list.len() <= 10 {
            println!("  {} appears for k = {:?}", value, k_list);
        } else {
            println!("  {} appears for {} values of k (first few: {:?}...)", 
                     value, k_list.len(), &k_list[..5]);
        }
    }
}
