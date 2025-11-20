/*
What is the index of the first term in the Fibonacci sequence to contain 
1000 digits?
 */

use std::process;

fn main() {
    /* 
    fib1 is our current sequence
    _fib2 is our last sequence
    fib3 is our placeholder
    */
    let mut fib1 = Vec::new();
    let mut _fib2 = Vec::new();
    let mut fib_index = 0;
    // Push in the first value and increment index by one:
    fib1.push(1);
    fib_index += 1;
    // Push in the second value and increment index by one:
    _fib2 = fib1.clone();
    fib_index += 1;
    // Now we go into a loop to add a bunch of stuff
    while fib1.len() < 1000 {
	let mut fib3 = Vec::new();
	if fib1.len() == _fib2.len() {
	    for i in 0..fib1.len() {
		fib3.push(fib1[i] + _fib2[i]);
	    }
	} else if fib1.len() == (_fib2.len() + 1) {
	    fib3.push(fib1[0]);
	    for i in 0.._fib2.len() {
		fib3.push(fib1[i+1] + _fib2[i]);
	    }
	} else {
	    println!("Error!");
	    process::exit(1);
	}
	// Clean up fib3
	if fib3[0] > 9 {
	    fib3[0] = fib3[0] - 10;
	    fib3.insert(0,1);
	}
	for i in (0..fib3.len()).rev() {
	    if fib3[i] > 9 {
		fib3[i] = fib3[i] - 10;
		if i > 0 {
		    fib3[i-1] = fib3[i-1] + 1;
		} else {
		    fib3.insert(0,1);
		}
	    }
	}
	// Now that fib3 is cleaned up, let's do the shift
	_fib2 = fib1.clone();
	fib1 = fib3.clone();

	fib_index += 1;
    }

    println!("{}", fib1.len());
    println!("{}", fib_index);
}
