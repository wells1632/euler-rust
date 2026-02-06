use std::collections::HashMap;

fn main() {
    let mut digit_groups: HashMap<String, Vec<u64>> = HashMap::new();
    
    // Generate cubes and group them by sorted digits
    let mut n = 1u64;
    loop {
        let cube = n * n * n;
        
        // Sort the digits to create a canonical form
        let sorted_digits = sort_digits(cube);
        
        // Add this cube to the group
        digit_groups.entry(sorted_digits.clone())
            .or_insert_with(Vec::new)
            .push(cube);
        
        // Check if we found a group with exactly 5 cubes
        if let Some(cubes) = digit_groups.get(&sorted_digits) {
            if cubes.len() == 5 {
                // Sort to find the smallest
                let mut cubes_sorted = cubes.clone();
                cubes_sorted.sort();
                
                println!("Found exactly 5 permutations!");
                println!("The smallest cube is: {}", cubes_sorted[0]);
                println!("\nAll five cubes:");
                for &cube in &cubes_sorted {
                    let cube_root = (cube as f64).cbrt().round() as u64;
                    println!("  {} = {}³", cube, cube_root);
                }
                return;
            }
        }
        
        n += 1;
        
        // Safety check to avoid infinite loop
        if n > 10000 {
            break;
        }
    }
    
    println!("No solution found in the range checked.");
}

fn sort_digits(mut num: u64) -> String {
    let mut digits: Vec<char> = num.to_string().chars().collect();
    digits.sort_unstable();
    digits.into_iter().collect()
}
