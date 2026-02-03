fn main() {
    let max_coord = 50;
    let count = count_right_triangles(max_coord);
    println!("Number of right triangles with coordinates 0-{}: {}", max_coord, count);
}

fn count_right_triangles(max: i32) -> usize {
    let mut count = 0;
    
    // One vertex is always at origin (0,0)
    // Try all combinations of two other points
    for x1 in 0..=max {
        for y1 in 0..=max {
            // Skip if first point is at origin
            if x1 == 0 && y1 == 0 {
                continue;
            }
            
            for x2 in 0..=max {
                for y2 in 0..=max {
                    // Skip if second point is at origin or same as first point
                    if (x2 == 0 && y2 == 0) || (x1 == x2 && y1 == y2) {
                        continue;
                    }
                    
                    // Check if we have a right triangle
                    // Right angle at origin: dot product of OA and OB = 0
                    let dot_origin = x1 * x2 + y1 * y2;
                    
                    // Right angle at A: dot product of AO and AB = 0
                    let dot_a = -x1 * (x2 - x1) + -y1 * (y2 - y1);
                    
                    // Right angle at B: dot product of BO and BA = 0
                    let dot_b = -x2 * (x1 - x2) + -y2 * (y1 - y2);
                    
                    if dot_origin == 0 || dot_a == 0 || dot_b == 0 {
                        count += 1;
                    }
                }
            }
        }
    }
    
    // Divide by 2 because we counted each triangle twice (once for each ordering of points)
    count / 2
}
