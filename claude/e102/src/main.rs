use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        eprintln!("Example: {} triangles.txt", args[0]);
        std::process::exit(1);
    }
    
    let filename = &args[1];
    
    println!("Reading file: {}", filename);
    
    match count_triangles_containing_origin(filename) {
        Ok(count) => {
            println!("\nNumber of triangles containing the origin: {}", count);
        }
        Err(e) => {
            eprintln!("Error reading file '{}': {}", filename, e);
            eprintln!("\nMake sure the file exists and contains triangles in CSV format:");
            eprintln!("X1,Y1,X2,Y2,X3,Y3");
            std::process::exit(1);
        }
    }
}

fn count_triangles_containing_origin(filename: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    
    let mut count = 0;
    let mut total_lines = 0;
    let mut line_num = 0;
    
    for line in reader.lines() {
        line_num += 1;
        let line = line?;
        
        // Skip empty lines
        if line.trim().is_empty() {
            continue;
        }
        
        total_lines += 1;
        
        // Parse the CSV line
        let coords: Result<Vec<f64>, _> = line
            .split(',')
            .map(|s| s.trim().parse::<f64>())
            .collect();
        
        match coords {
            Ok(c) if c.len() == 6 => {
                let (x1, y1, x2, y2, x3, y3) = (c[0], c[1], c[2], c[3], c[4], c[5]);
                
                if point_in_triangle(0.0, 0.0, x1, y1, x2, y2, x3, y3) {
                    count += 1;
                    println!("Line {}: Triangle ({},{}) ({},{}) ({},{}) contains origin", 
                             line_num, x1, y1, x2, y2, x3, y3);
                }
            }
            Ok(_) => {
                eprintln!("Line {}: Expected 6 coordinates, found {}", line_num, line.split(',').count());
            }
            Err(e) => {
                eprintln!("Line {}: Parse error: {}", line_num, e);
            }
        }
    }
    
    println!("\nProcessed {} triangles", total_lines);
    Ok(count)
}

fn point_in_triangle(px: f64, py: f64, x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64) -> bool {
    // Using the sign of cross products method
    // A point P is inside triangle ABC if it's on the same side of all three edges
    
    fn sign(px: f64, py: f64, ax: f64, ay: f64, bx: f64, by: f64) -> f64 {
        (px - bx) * (ay - by) - (ax - bx) * (py - by)
    }
    
    let d1 = sign(px, py, x1, y1, x2, y2);
    let d2 = sign(px, py, x2, y2, x3, y3);
    let d3 = sign(px, py, x3, y3, x1, y1);
    
    let has_neg = (d1 < 0.0) || (d2 < 0.0) || (d3 < 0.0);
    let has_pos = (d1 > 0.0) || (d2 > 0.0) || (d3 > 0.0);
    
    !(has_neg && has_pos)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_point_in_triangle() {
        // Triangle containing origin
        assert!(point_in_triangle(0.0, 0.0, -1.0, -1.0, 1.0, -1.0, 0.0, 1.0));
        
        // Triangle not containing origin
        assert!(!point_in_triangle(0.0, 0.0, 1.0, 1.0, 2.0, 1.0, 1.5, 2.0));
        
        // Origin on edge (should return true)
        assert!(point_in_triangle(0.0, 0.0, -1.0, 0.0, 1.0, 0.0, 0.0, 1.0));
    }
}
