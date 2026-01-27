use std::collections::{HashMap, HashSet, VecDeque};
use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        process::exit(1);
    }
    
    let filename = &args[1];
    
    let attempts = match read_attempts(filename) {
        Ok(att) => att,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            process::exit(1);
        }
    };
    
    let passcode = find_shortest_passcode(&attempts);
    
    println!("Shortest possible passcode: {}", passcode);
}

fn read_attempts(filename: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(filename)?;
    
    let attempts: Vec<String> = contents
        .lines()
        .map(|line| line.trim().to_string())
        .filter(|line| !line.is_empty())
        .collect();
    
    Ok(attempts)
}

fn find_shortest_passcode(attempts: &[String]) -> String {
    // Build graph of ordering constraints
    let mut graph: HashMap<char, HashSet<char>> = HashMap::new();
    let mut all_digits: HashSet<char> = HashSet::new();
    
    for attempt in attempts {
        let chars: Vec<char> = attempt.chars().collect();
        
        for &c in &chars {
            all_digits.insert(c);
            graph.entry(c).or_insert_with(HashSet::new);
        }
        
        // Add edges for ordering constraints
        for i in 0..chars.len() {
            for j in i + 1..chars.len() {
                graph.get_mut(&chars[i]).unwrap().insert(chars[j]);
            }
        }
    }
    
    // Topological sort to find shortest sequence
    let mut in_degree: HashMap<char, usize> = HashMap::new();
    
    for &digit in &all_digits {
        in_degree.insert(digit, 0);
    }
    
    for edges in graph.values() {
        for &to in edges {
            *in_degree.get_mut(&to).unwrap() += 1;
        }
    }
    
    // Start with digits that have no predecessors
    let mut queue: VecDeque<char> = VecDeque::new();
    for (&digit, &degree) in &in_degree {
        if degree == 0 {
            queue.push_back(digit);
        }
    }
    
    let mut result = String::new();
    
    while let Some(digit) = queue.pop_front() {
        result.push(digit);
        
        if let Some(neighbors) = graph.get(&digit) {
            for &next in neighbors {
                let degree = in_degree.get_mut(&next).unwrap();
                *degree -= 1;
                if *degree == 0 {
                    queue.push_back(next);
                }
            }
        }
    }
    
    result
}
