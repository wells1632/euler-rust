use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;

fn parse_words(filename: &str) -> Vec<String> {
    let contents = fs::read_to_string(filename)
        .expect("Failed to read file");
    
    contents
        .split(',')
        .map(|s| s.trim().trim_matches('"').to_uppercase())
        .filter(|s| !s.is_empty())
        .collect()
}

fn get_anagram_groups(words: Vec<String>) -> HashMap<String, Vec<String>> {
    let mut groups: HashMap<String, Vec<String>> = HashMap::new();
    
    for word in words {
        let mut chars: Vec<char> = word.chars().collect();
        chars.sort();
        let key: String = chars.into_iter().collect();
        groups.entry(key).or_insert_with(Vec::new).push(word);
    }
    
    groups.into_iter()
        .filter(|(_, v)| v.len() >= 2)
        .collect()
}

fn is_perfect_square(n: u64) -> bool {
    let sqrt = (n as f64).sqrt() as u64;
    sqrt * sqrt == n
}

fn word_to_number(word: &str, mapping: &HashMap<char, u8>) -> Option<u64> {
    let first_char = word.chars().next()?;
    if mapping.get(&first_char) == Some(&0) {
        return None; // No leading zeros
    }
    
    let mut result = 0u64;
    for c in word.chars() {
        let digit = *mapping.get(&c)?;
        result = result * 10 + digit as u64;
    }
    Some(result)
}

fn find_square_anagram_pairs(group: &[String]) -> Option<u64> {
    let unique_chars: HashSet<char> = group.iter()
        .flat_map(|w| w.chars())
        .collect();
    
    if unique_chars.len() > 10 {
        return None;
    }
    
    let chars: Vec<char> = unique_chars.into_iter().collect();
    let mut max_square = 0u64;
    
    permute_digits(&chars, &mut vec![false; 10], &mut HashMap::new(), 0, group, &mut max_square);
    
    if max_square > 0 {
        Some(max_square)
    } else {
        None
    }
}

fn permute_digits(
    chars: &[char],
    used: &mut Vec<bool>,
    mapping: &mut HashMap<char, u8>,
    index: usize,
    words: &[String],
    max_square: &mut u64,
) {
    if index == chars.len() {
        let mut squares = Vec::new();
        
        for word in words {
            if let Some(num) = word_to_number(word, mapping) {
                if is_perfect_square(num) {
                    squares.push(num);
                }
            }
        }
        
        if squares.len() >= 2 {
            *max_square = (*max_square).max(*squares.iter().max().unwrap());
        }
        return;
    }
    
    for digit in 0..10 {
        if !used[digit] {
            used[digit] = true;
            mapping.insert(chars[index], digit as u8);
            
            permute_digits(chars, used, mapping, index + 1, words, max_square);
            
            mapping.remove(&chars[index]);
            used[digit] = false;
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }
    
    let filename = &args[1];
    let words = parse_words(filename);
    let anagram_groups = get_anagram_groups(words);
    
    let mut overall_max = 0u64;
    
    for (_, group) in anagram_groups.iter() {
        if let Some(max_sq) = find_square_anagram_pairs(group) {
            println!("Anagram group {:?}: max square = {}", group, max_sq);
            overall_max = overall_max.max(max_sq);
        }
    }
    
    println!("\nLargest square number: {}", overall_max);
}
