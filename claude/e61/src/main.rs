use std::collections::HashMap;

fn main() {
    // Generate all 4-digit polygonal numbers for each type
    let triangle = generate_polygonal(3, |n| n * (n + 1) / 2);
    let square = generate_polygonal(4, |n| n * n);
    let pentagonal = generate_polygonal(5, |n| n * (3 * n - 1) / 2);
    let hexagonal = generate_polygonal(6, |n| n * (2 * n - 1));
    let heptagonal = generate_polygonal(7, |n| n * (5 * n - 3) / 2);
    let octagonal = generate_polygonal(8, |n| n * (3 * n - 2));

    let all_sets = vec![
        (3, &triangle),
        (4, &square),
        (5, &pentagonal),
        (6, &hexagonal),
        (7, &heptagonal),
        (8, &octagonal),
    ];

    // Try to find a cyclic chain starting with each polygonal type
    for start_idx in 0..all_sets.len() {
        let mut used_types = vec![false; 9]; // indices 0-8 (we use 3-8)
        let mut chain = Vec::new();
        
        if let Some(result) = find_cycle(&all_sets, start_idx, &mut chain, &mut used_types, start_idx) {
            println!("Found cyclic set:");
            let mut sum = 0;
            for (num, type_id) in &result {
                let type_name = match type_id {
                    3 => "Triangle",
                    4 => "Square",
                    5 => "Pentagonal",
                    6 => "Hexagonal",
                    7 => "Heptagonal",
                    8 => "Octagonal",
                    _ => "Unknown",
                };
                println!("{}: {}", type_name, num);
                sum += num;
            }
            println!("\nSum: {}", sum);
            return;
        }
    }
}

fn generate_polygonal<F>(type_id: usize, formula: F) -> HashMap<u32, Vec<u32>>
where
    F: Fn(u32) -> u32,
{
    let mut map = HashMap::new();
    let mut n = 1;
    
    loop {
        let num = formula(n);
        if num >= 10000 {
            break;
        }
        if num >= 1000 {
            let prefix = num / 100; // First two digits
            map.entry(prefix).or_insert_with(Vec::new).push(num);
        }
        n += 1;
    }
    
    map
}

fn find_cycle(
    all_sets: &[(usize, &HashMap<u32, Vec<u32>>)],
    current_idx: usize,
    chain: &mut Vec<(u32, usize)>,
    used_types: &mut Vec<bool>,
    start_idx: usize,
) -> Option<Vec<(u32, usize)>> {
    let (current_type, current_set) = all_sets[current_idx];
    
    if chain.len() == 6 {
        // Check if we can close the cycle
        let first_num = chain[0].0;
        let last_num = chain[5].0;
        let first_prefix = first_num / 100;
        let last_suffix = last_num % 100;
        
        if first_prefix == last_suffix && last_suffix >= 10 {
            return Some(chain.clone());
        }
        return None;
    }
    
    let suffix_to_match = if chain.is_empty() {
        None
    } else {
        Some(chain.last().unwrap().0 % 100)
    };
    
    if chain.is_empty() {
        // First number - try all numbers from this set
        for (prefix, nums) in current_set {
            if *prefix < 10 { continue; } // Skip numbers starting with 0
            for &num in nums {
                if num % 100 < 10 { continue; } // Skip numbers ending with 0X
                
                chain.push((num, current_type));
                used_types[current_type] = true;
                
                // Try next type
                for next_idx in 0..all_sets.len() {
                    if used_types[all_sets[next_idx].0] {
                        continue;
                    }
                    if let Some(result) = find_cycle(all_sets, next_idx, chain, used_types, start_idx) {
                        return Some(result);
                    }
                }
                
                chain.pop();
                used_types[current_type] = false;
            }
        }
    } else {
        // Match the suffix
        let target_prefix = suffix_to_match.unwrap();
        if let Some(nums) = current_set.get(&target_prefix) {
            for &num in nums {
                if num % 100 < 10 { continue; } // Skip numbers ending with 0X
                
                chain.push((num, current_type));
                used_types[current_type] = true;
                
                if chain.len() == 6 {
                    if let Some(result) = find_cycle(all_sets, current_idx, chain, used_types, start_idx) {
                        return Some(result);
                    }
                } else {
                    // Try next type
                    for next_idx in 0..all_sets.len() {
                        if used_types[all_sets[next_idx].0] {
                            continue;
                        }
                        if let Some(result) = find_cycle(all_sets, next_idx, chain, used_types, start_idx) {
                            return Some(result);
                        }
                    }
                }
                
                chain.pop();
                used_types[current_type] = false;
            }
        }
    }
    
    None
}
