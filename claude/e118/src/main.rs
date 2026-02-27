fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    let limit = (n as f64).sqrt() as u32;
    for i in (3..=limit).step_by(2) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn generate_partitions(digits: &[u8], start: usize, current: &mut Vec<u32>, result: &mut Vec<Vec<u32>>) {
    if start == digits.len() {
        result.push(current.clone());
        return;
    }
    
    // Try forming numbers of different lengths starting from 'start'
    let mut num = 0u32;
    for end in start..digits.len() {
        num = num * 10 + digits[end] as u32;
        if is_prime(num) {
            current.push(num);
            generate_partitions(digits, end + 1, current, result);
            current.pop();
        }
    }
}

fn main() {
    use std::collections::HashSet;
    
    let mut digits = [1u8, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut all_valid_sets: HashSet<Vec<u32>> = HashSet::new();
    
    // Generate all permutations of digits 1-9
    let mut permutation_count = 0;
    permute(&mut digits, 0, &mut |perm| {
        permutation_count += 1;
        let mut partitions = Vec::new();
        let mut current = Vec::new();
        generate_partitions(perm, 0, &mut current, &mut partitions);
        
        // For each valid partition, add the sorted set
        for partition in partitions {
            let mut sorted = partition.clone();
            sorted.sort_unstable();
            all_valid_sets.insert(sorted);
        }
    });
    
    println!("Total permutations checked: {}", permutation_count);
    println!("Distinct sets containing only primes: {}", all_valid_sets.len());
}

fn permute<F>(arr: &mut [u8], start: usize, callback: &mut F)
where
    F: FnMut(&[u8]),
{
    if start == arr.len() {
        callback(arr);
        return;
    }
    
    for i in start..arr.len() {
        arr.swap(start, i);
        permute(arr, start + 1, callback);
        arr.swap(start, i);
    }
}
