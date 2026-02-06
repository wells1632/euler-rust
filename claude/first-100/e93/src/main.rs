use std::collections::HashSet;

fn main() {
    let mut best_digits = [0, 0, 0, 0];
    let mut best_n = 0;
    
    println!("Searching for optimal digit set...\n");
    
    // Try all combinations of four distinct digits a < b < c < d
    for a in 0..=9 {
        for b in (a+1)..=9 {
            for c in (b+1)..=9 {
                for d in (c+1)..=9 {
                    let digits = [a, b, c, d];
                    let reachable = find_reachable_numbers(&digits);
                    let n = find_longest_consecutive(&reachable);
                    
                    if n > best_n {
                        best_n = n;
                        best_digits = digits;
                        println!("New best: {:?} -> consecutive 1 to {}", digits, n);
                    }
                }
            }
        }
    }
    
    println!("\n{}", "=".repeat(70));
    println!("RESULT:");
    println!("Best digit set: {:?}", best_digits);
    println!("Consecutive integers: 1 to {}", best_n);
    println!("Answer: {}{}{}{}", best_digits[0], best_digits[1], best_digits[2], best_digits[3]);
    println!("{}", "=".repeat(70));
}

fn find_reachable_numbers(digits: &[i32; 4]) -> HashSet<i32> {
    let mut reachable = HashSet::new();
    
    // Try all permutations of the four digits
    let mut perm = *digits;
    permute(&mut perm, 0, &mut reachable);
    
    reachable
}

fn permute(arr: &mut [i32; 4], start: usize, reachable: &mut HashSet<i32>) {
    if start == arr.len() {
        // Evaluate all possible expressions with this permutation
        evaluate_all_expressions(arr, reachable);
        return;
    }
    
    for i in start..arr.len() {
        arr.swap(start, i);
        permute(arr, start + 1, reachable);
        arr.swap(start, i);
    }
}

fn evaluate_all_expressions(nums: &[i32; 4], reachable: &mut HashSet<i32>) {
    let ops = ['+', '-', '*', '/'];
    
    // Try all combinations of three operators
    for &op1 in &ops {
        for &op2 in &ops {
            for &op3 in &ops {
                // Try all five tree structures
                
                // Structure 1: ((a op b) op c) op d
                if let Some(val) = eval_tree1(nums[0] as f64, nums[1] as f64, nums[2] as f64, nums[3] as f64, op1, op2, op3) {
                    add_if_positive_integer(val, reachable);
                }
                
                // Structure 2: (a op (b op c)) op d
                if let Some(val) = eval_tree2(nums[0] as f64, nums[1] as f64, nums[2] as f64, nums[3] as f64, op1, op2, op3) {
                    add_if_positive_integer(val, reachable);
                }
                
                // Structure 3: (a op b) op (c op d)
                if let Some(val) = eval_tree3(nums[0] as f64, nums[1] as f64, nums[2] as f64, nums[3] as f64, op1, op2, op3) {
                    add_if_positive_integer(val, reachable);
                }
                
                // Structure 4: a op ((b op c) op d)
                if let Some(val) = eval_tree4(nums[0] as f64, nums[1] as f64, nums[2] as f64, nums[3] as f64, op1, op2, op3) {
                    add_if_positive_integer(val, reachable);
                }
                
                // Structure 5: a op (b op (c op d))
                if let Some(val) = eval_tree5(nums[0] as f64, nums[1] as f64, nums[2] as f64, nums[3] as f64, op1, op2, op3) {
                    add_if_positive_integer(val, reachable);
                }
            }
        }
    }
}

fn add_if_positive_integer(val: f64, reachable: &mut HashSet<i32>) {
    if val > 0.0 && val <= 10000.0 && (val - val.round()).abs() < 1e-9 {
        reachable.insert(val.round() as i32);
    }
}

// ((a op1 b) op2 c) op3 d
fn eval_tree1(a: f64, b: f64, c: f64, d: f64, op1: char, op2: char, op3: char) -> Option<f64> {
    let r1 = apply_op(a, b, op1)?;
    let r2 = apply_op(r1, c, op2)?;
    apply_op(r2, d, op3)
}

// (a op1 (b op2 c)) op3 d
fn eval_tree2(a: f64, b: f64, c: f64, d: f64, op1: char, op2: char, op3: char) -> Option<f64> {
    let r1 = apply_op(b, c, op2)?;
    let r2 = apply_op(a, r1, op1)?;
    apply_op(r2, d, op3)
}

// (a op1 b) op2 (c op3 d)
fn eval_tree3(a: f64, b: f64, c: f64, d: f64, op1: char, op2: char, op3: char) -> Option<f64> {
    let r1 = apply_op(a, b, op1)?;
    let r2 = apply_op(c, d, op3)?;
    apply_op(r1, r2, op2)
}

// a op1 ((b op2 c) op3 d)
fn eval_tree4(a: f64, b: f64, c: f64, d: f64, op1: char, op2: char, op3: char) -> Option<f64> {
    let r1 = apply_op(b, c, op2)?;
    let r2 = apply_op(r1, d, op3)?;
    apply_op(a, r2, op1)
}

// a op1 (b op2 (c op3 d))
fn eval_tree5(a: f64, b: f64, c: f64, d: f64, op1: char, op2: char, op3: char) -> Option<f64> {
    let r1 = apply_op(c, d, op3)?;
    let r2 = apply_op(b, r1, op2)?;
    apply_op(a, r2, op1)
}

fn apply_op(a: f64, b: f64, op: char) -> Option<f64> {
    match op {
        '+' => Some(a + b),
        '-' => Some(a - b),
        '*' => Some(a * b),
        '/' => {
            if b.abs() < 1e-9 {
                None
            } else {
                Some(a / b)
            }
        }
        _ => None,
    }
}

fn find_longest_consecutive(numbers: &HashSet<i32>) -> i32 {
    let mut n = 1;
    while numbers.contains(&n) {
        n += 1;
    }
    n - 1
}
