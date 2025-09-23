use std::fs::File;
use std::io::{self};
use std::io::{BufRead};
use std::path::Path;

// Work out the first ten digits of the sum of the following one-hundred 50-digit numbers.
//
// Numbers were dumped into a data file data.txt

// This solution only uses 32-bit number fields, and realistically would work just fine on
// an 8-bit computer, since it drops any need for large numbers

fn main() {
    // Completely stupid, but let's pull in all of the lines and punch the values into a 100x50 grid
    let mut matrix: Vec<Vec<i32>> = Vec::new();
    if let Ok(lines) = read_lines("data.txt") {
        for line in lines.map_while(Result::ok) {
            let mut matrix_line: Vec<i32> = vec![];
            for i in line.chars() {
                matrix_line.push(i as i32-48);
            }
            matrix.push(matrix_line);
        }
    }

    // Now we will do addition the hard way, throwing all of the numbers into a separate vector
    let mut result: Vec<i32> = vec![];
    let mut carryover = 0i32;
    for i in (0..50).rev() {
        let mut interm = carryover;
        for j in 0..100 {
            interm += matrix[j][i];
        }
        let remaind = interm % 10;
        result.push(remaind);
        carryover = interm / 10;
    }
    while carryover > 0 {
        result.push(carryover % 10);
        carryover = carryover / 10;
    }
    for i in ((result.len()-10)..result.len()).rev() {
        print!("{}", result[i]);
    }
    println!("");
    //    println!("Total: {}", total);
    
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
    
