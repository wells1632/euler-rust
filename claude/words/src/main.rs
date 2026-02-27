use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;
use std::process;

fn main() {
    // Get dictionary file from command line
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 2 {
        eprintln!("Usage: {} <dictionary_file>", args[0]);
        eprintln!("Example: {} /usr/share/dict/words", args[0]);
        process::exit(1);
    }
    
    let dict_path = &args[1];
    
    println!("Loading dictionary from: {}", dict_path);
    let dictionary = match load_dictionary(dict_path) {
        Ok(dict) => {
            println!("Loaded {} words", dict.len());
            dict
        },
        Err(e) => {
            eprintln!("Error loading dictionary: {}", e);
            process::exit(1);
        }
    };
    
    let mut max_product = 0u64;
    let mut best_info = String::new();
    
    println!("Processing numbers 1-1000...");
    
    for n in 1..=1000 {
        let written = number_to_words(n);
        let rot13_written = rot13(&written);
        
        // Find longest anagram from ROT13'd letters
        if let Some(longest_word) = find_longest_anagram(&rot13_written, &dictionary) {
            // ROT13 the word back
            let rot13_word = rot13(&longest_word);
            
            // Calculate product of letter values
            let product = letter_product(&rot13_word);
            
            if product > max_product {
                max_product = product;
                best_info = format!(
                    "Number: {}, Written: '{}', ROT13: '{}', Longest anagram: '{}', ROT13 back: '{}', Product: {}",
                    n, written, rot13_written, longest_word, rot13_word, product
                );
            }
        }
        
        if n % 100 == 0 {
            println!("Processed {} numbers...", n);
        }
    }
    
    println!("\n{}", best_info);
    println!("\nLargest product: {}", max_product);
}

fn number_to_words(n: u32) -> String {
    if n == 0 { return "Zero".to_string(); }
    if n == 1000 { return "One thousand".to_string(); }
    
    let ones = ["", "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine"];
    let teens = ["Ten", "Eleven", "Twelve", "Thirteen", "Fourteen", "Fifteen", 
                 "Sixteen", "Seventeen", "Eighteen", "Nineteen"];
    let tens = ["", "", "Twenty", "Thirty", "Forty", "Fifty", "Sixty", "Seventy", "Eighty", "Ninety"];
    
    let mut result = String::new();
    
    // Hundreds
    if n >= 100 {
        let hundreds = (n / 100) as usize;
        result.push_str(ones[hundreds]);
        result.push_str(" hundred");
        
        if n % 100 != 0 {
            result.push_str(" and ");
        }
    }
    
    let remainder = n % 100;
    
    // Tens and ones
    if remainder >= 20 {
        let tens_digit = (remainder / 10) as usize;
        result.push_str(tens[tens_digit]);
        
        if remainder % 10 != 0 {
            result.push(' ');
            result.push_str(ones[(remainder % 10) as usize]);
        }
    } else if remainder >= 10 {
        result.push_str(teens[(remainder - 10) as usize]);
    } else if remainder > 0 {
        result.push_str(ones[remainder as usize]);
    }
    
    result
}

fn rot13(s: &str) -> String {
    s.chars().map(|c| {
        if c.is_ascii_alphabetic() {
            let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
            let offset = (c as u8 - base + 13) % 26;
            (base + offset) as char
        } else {
            c
        }
    }).collect()
}

fn find_longest_anagram(letters: &str, dictionary: &HashSet<String>) -> Option<String> {
    // Extract only letters and convert to lowercase
    let available: Vec<char> = letters.chars()
        .filter(|c| c.is_alphabetic())
        .map(|c| c.to_ascii_lowercase())
        .collect();
    
    let mut longest: Option<String> = None;
    let mut max_len = 0;
    
    for word in dictionary {
        if word.len() <= max_len {
            continue;
        }
        
        if can_form_word(word, &available) {
            max_len = word.len();
            longest = Some(word.clone());
        }
    }
    
    longest
}

fn can_form_word(word: &str, available: &[char]) -> bool {
    let mut available_counts = HashMap::new();
    for &c in available {
        *available_counts.entry(c).or_insert(0) += 1;
    }
    
    for c in word.chars() {
        let c = c.to_ascii_lowercase();
        let count = available_counts.get_mut(&c);
        
        match count {
            Some(n) if *n > 0 => *n -= 1,
            _ => return false,
        }
    }
    
    true
}

fn letter_product(word: &str) -> u64 {
    word.chars()
        .filter(|c| c.is_alphabetic())
        .map(|c| {
            let c = c.to_ascii_lowercase();
            (c as u8 - b'a' + 1) as u64
        })
        .product()
}

fn load_dictionary(path: &str) -> Result<HashSet<String>, std::io::Error> {
    let contents = fs::read_to_string(path)?;
    
    let words: HashSet<String> = contents.lines()
        .map(|s| s.trim().to_lowercase())
        .filter(|s| !s.is_empty() && s.chars().all(|c| c.is_alphabetic()))
        .collect();
    
    Ok(words)
}
