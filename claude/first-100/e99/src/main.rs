use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }
    
    let filename = &args[1];
    let contents = fs::read_to_string(filename)
        .expect("Failed to read file");
    
    let mut max_value = f64::MIN;
    let mut max_line = 0;
    let mut max_base = 0u64;
    let mut max_exp = 0u64;
    
    for (index, line) in contents.lines().enumerate() {
        let parts: Vec<&str> = line.trim().split(',').collect();
        
        if parts.len() == 2 {
            let base: u64 = parts[0].trim().parse().expect("Invalid base");
            let exponent: u64 = parts[1].trim().parse().expect("Invalid exponent");
            
            // Use logarithms to compare: base^exp = exp * log(base)
            let log_value = (exponent as f64) * (base as f64).ln();
            
            if log_value > max_value {
                max_value = log_value;
                max_line = index + 1;
                max_base = base;
                max_exp = exponent;
            }
        }
    }
    
    println!("\n===========================================");
    println!("RESULT: Line number {}", max_line);
    println!("===========================================");
    println!("Value: {}^{}", max_base, max_exp);
    println!("Log value: {:.6}", max_value);
}
