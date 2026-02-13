use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Dart {
    Single(u32),
    Double(u32),
    Triple(u32),
    Bull,      // 25
    DBull,     // 50
}

impl Dart {
    fn value(&self) -> u32 {
        match self {
            Dart::Single(n) => *n,
            Dart::Double(n) => *n * 2,
            Dart::Triple(n) => *n * 3,
            Dart::Bull => 25,
            Dart::DBull => 50,
        }
    }
    
    fn is_double(&self) -> bool {
        matches!(self, Dart::Double(_) | Dart::DBull)
    }
}

fn main() {
    println!("Dart Checkout Calculator\n");
    
    // Test with score 6
    let test_ways = count_checkouts(6);
    println!("Score 6: {} ways (expected 11)\n", test_ways);
    
    let mut total = 0;
    
    for score in 2..100 {
        let ways = count_checkouts(score);
        if ways > 0 {
            total += ways;
        }
    }
    
    println!("\n{}", "=".repeat(70));
    println!("ANSWER: {} distinct checkouts for scores < 100", total);
    println!("{}", "=".repeat(70));
}

fn count_checkouts(target: u32) -> usize {
    let all_darts = generate_all_darts();
    let mut count = 0;
    
    // 1-dart checkout: must be a double
    for &dart in &all_darts {
        if dart.is_double() && dart.value() == target {
            count += 1;
        }
    }
    
    // 2-dart checkout: any dart + specific double
    for &first in &all_darts {
        for &finish in &all_darts {
            if finish.is_double() && first.value() + finish.value() == target {
                count += 1;
            }
        }
    }
    
    // 3-dart checkout: unordered pair + specific double
    let mut three_dart = HashSet::new();
    
    for i in 0..all_darts.len() {
        for j in i..all_darts.len() {
            let d1 = all_darts[i];
            let d2 = all_darts[j];
            
            for &finish in &all_darts {
                if finish.is_double() && d1.value() + d2.value() + finish.value() == target {
                    // Create sorted pair for uniqueness
                    let pair = if d1 <= d2 {
                        (d1, d2, finish)
                    } else {
                        (d2, d1, finish)
                    };
                    three_dart.insert(pair);
                }
            }
        }
    }
    
    count += three_dart.len();
    count
}

fn generate_all_darts() -> Vec<Dart> {
    let mut darts = Vec::new();
    
    for n in 1..=20 {
        darts.push(Dart::Single(n));
        darts.push(Dart::Double(n));
        darts.push(Dart::Triple(n));
    }
    
    darts.push(Dart::Bull);
    darts.push(Dart::DBull);
    
    darts
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_score_6() {
        assert_eq!(count_checkouts(6), 11);
    }
}
