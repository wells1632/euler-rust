use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
	eprintln!("Usage: {} <filename>", args[0]);
	std::process::exit(1);
    }

    let filename = &args[1];

    // Read the file
    let contents = fs::read_to_string(filename)
	.expect("Failed to read file");

    // Parse the names (remove quotes and split by comma)
    let mut names: Vec<String> = contents
	.split(',')
	.map(|s| s.trim().trim_matches('"').to_string())
	.collect();

    // Sort alphabetically
    names.sort();

    // Calculate total score
    let mut total_score = 0;

    for (index, name) in names.iter().enumerate() {
	let position = (index + 1) as u64;
	let name_value = calculate_name_value(name);
	let score = position * name_value;
	total_score += score;
    }

    println!("Total of all name scores: {}", total_score);
}

fn calculate_name_value(name: &str) -> u64 {
    name.chars()
	.filter(|c| c.is_alphabetic())
	.map(|c| (c.to_ascii_uppercase() as u64) - ('A' as u64) + 1)
	.sum()
}
