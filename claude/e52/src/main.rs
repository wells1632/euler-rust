use std::thread;
use std::sync::{Arc, Mutex};
use std::process;
use std::time::Instant;


fn get_sorted_digits(n: u64) -> Vec<char> {
    let mut digits: Vec<char> = n.to_string().chars().collect();
    digits.sort();
    digits
}

fn has_same_digits(a: u64, b: u64) -> bool {
    get_sorted_digits(a) == get_sorted_digits(b)
}

fn check_number(x: u64) -> bool {
    let x2 = 2 * x;
    let x3 = 3 * x;
    let x4 = 4 * x;
    let x5 = 5 * x;
    let x6 = 6 * x;
    
    has_same_digits(x, x2) 
        && has_same_digits(x, x3)
        && has_same_digits(x, x4)
        && has_same_digits(x, x5)
        && has_same_digits(x, x6)
}

fn main() {
    let num_threads = 8;
    let chunk_size = 50_000;
    let mut x = 1;
    let mut now = Instant::now();
    
    loop {
        let x2 = 2 * x;
        let x3 = 3 * x;
        let x4 = 4 * x;
        let x5 = 5 * x;
        let x6 = 6 * x;
        
        // Check if all multiples have the same digits
        if has_same_digits(x, x2) 
            && has_same_digits(x, x3)
            && has_same_digits(x, x4)
            && has_same_digits(x, x5)
            && has_same_digits(x, x6) {
            
            println!("Found: x = {}", x);
            println!("x   = {}", x);
            println!("2x  = {}", x2);
            println!("3x  = {}", x3);
            println!("4x  = {}", x4);
            println!("5x  = {}", x5);
            println!("6x  = {}", x6);
		println!("Time to complete single thread version: {:.2?}", now.elapsed());
	    break;
        }
        
        x += 1;
    }
    now = Instant::now();
    let max_search = 1_000_000;
    
    let result = Arc::new(Mutex::new(None));
    let mut handles = vec![];
    
    for thread_id in 0..num_threads {
        let result = Arc::clone(&result);
        
        let handle = thread::spawn(move || {
            let start = thread_id * chunk_size + 1;
            let end = ((thread_id + 1) * chunk_size).min(max_search);
            
            for x in start..=end {
                // Check if another thread already found a smaller result
                {
                    let current_result = result.lock().unwrap();
                    if let Some(found) = *current_result {
                        if found < start {
                            return; // Stop searching this chunk
                        }
                    }
                }
                
                if check_number(x) {
                    let mut current_result = result.lock().unwrap();
                    if current_result.is_none() || x < current_result.unwrap() {
                        *current_result = Some(x);
                        println!("Thread {} found: {}", thread_id, x);
                    }
                    return;
                }
            }
        });
        
        handles.push(handle);
    }
    
    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
    
    // Print final result
    if let Some(x) = *result.lock().unwrap() {
        println!("\nSmallest x = {}", x);
        println!("x   = {}", x);
        println!("2x  = {}", 2 * x);
        println!("3x  = {}", 3 * x);
        println!("4x  = {}", 4 * x);
        println!("5x  = {}", 5 * x);
        println!("6x  = {}", 6 * x);
	println!("Time to complete threaded version: {:.2?}", now.elapsed());
    };
}
