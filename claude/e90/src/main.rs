use std::collections::HashSet;

fn generate_cube_combinations() -> Vec<Vec<u8>> {
    let mut cubes = Vec::new();
    
    // Generate all 6-element subsets of digits 0-9 (combinations without replacement)
    fn generate_combinations(start: usize, current: &mut Vec<u8>, remaining: usize, cubes: &mut Vec<Vec<u8>>) {
        if remaining == 0 {
            cubes.push(current.clone());
            return;
        }
        
        for i in start..=(10 - remaining) {
            current.push(i as u8);
            generate_combinations(i + 1, current, remaining - 1, cubes);
            current.pop();
        }
    }
    
    let mut current = Vec::new();
    generate_combinations(0, &mut current, 6, &mut cubes);
    cubes
}

fn can_show_digit(cube: &[u8], digit: u8) -> bool {
    cube.contains(&digit) || 
    (digit == 9 && cube.contains(&6)) ||
    (digit == 6 && cube.contains(&9))
}

fn can_display_all_squares(cube1: &[u8], cube2: &[u8]) -> bool {
    // All 2-digit square numbers below 100: 01, 04, 09, 16, 25, 36, 49, 64, 81
    let squares = [(0, 1), (0, 4), (0, 9), (1, 6), (2, 5), (3, 6), (4, 9), (6, 4), (8, 1)];
    
    for (tens, ones) in squares {
        let can_display = 
            (can_show_digit(cube1, tens) && can_show_digit(cube2, ones)) ||
            (can_show_digit(cube1, ones) && can_show_digit(cube2, tens));
            
        if !can_display {
            return false;
        }
    }
    true
}

fn main() {
    let cubes = generate_cube_combinations();
    let mut count = 0;
    
    println!("Generated {} unique 6-element subsets", cubes.len());
    
    // Count all unordered pairs of cubes that can display all squares
    for i in 0..cubes.len() {
        for j in i..cubes.len() {
            if can_display_all_squares(&cubes[i], &cubes[j]) {
                count += 1;
            }
        }
    }
    
    println!("Number of distinct arrangements: {}", count);
}
