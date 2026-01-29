use std::fs::File;
use std::io::{BufRead, BufReader};
use std::env;

fn roman_to_decimal(roman: &str) -> i32 {
    let mut result = 0;
    let mut prev_value = 0;
    
    for ch in roman.chars().rev() {
        let value = match ch {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => 0,
        };
        
        if value < prev_value {
            result -= value;
        } else {
            result += value;
        }
        prev_value = value;
    }
    
    result
}

fn decimal_to_minimal_roman(mut num: i32) -> String {
    let values = [
        (1000, "M"),
        (900, "CM"),
        (500, "D"),
        (400, "CD"),
        (100, "C"),
        (90, "XC"),
        (50, "L"),
        (40, "XL"),
        (10, "X"),
        (9, "IX"),
        (5, "V"),
        (4, "IV"),
        (1, "I"),
    ];
    
    let mut result = String::new();
    
    for (value, numeral) in values.iter() {
        while num >= *value {
            result.push_str(numeral);
            num -= value;
        }
    }
    
    result
}

fn main() {
    // Get filename from command line or use default
    let args: Vec<String> = env::args().collect();
    let filename = if args.len() > 1 {
        &args[1]
    } else {
        "roman.txt"
    };
    
    // Open and read file
    let file = File::open(filename).expect("Unable to open file");
    let reader = BufReader::new(file);
    
    let mut total_savings = 0;
    let mut line_count = 0;
    let mut examples = Vec::new();
    
    for line in reader.lines() {
        let line = line.expect("Unable to read line").trim().to_string();
        
        if line.is_empty() {
            continue;
        }
        
        line_count += 1;
        
        // Convert to decimal then back to minimal form
        let decimal_value = roman_to_decimal(&line);
        let minimal_form = decimal_to_minimal_roman(decimal_value);
        
        let original_len = line.len();
        let minimal_len = minimal_form.len();
        let savings = original_len as i32 - minimal_len as i32;
        
        if savings > 0 {
            total_savings += savings;
            
            // Store first few examples
            if examples.len() < 10 {
                examples.push((line.clone(), minimal_form, savings));
            }
        }
    }
    
    println!("Roman Numeral Analysis");
    println!("======================");
    println!("Lines processed: {}", line_count);
    println!("Total characters saved: {}", total_savings);
    println!();
    
    if !examples.is_empty() {
        println!("Example conversions:");
        for (original, minimal, savings) in examples {
            println!("  {} ({}) -> {} ({}) : saved {} character(s)",
                     original, original.len(), minimal, minimal.len(), savings);
        }
    }
}
