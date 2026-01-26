fn main() {
    let mut max_solution = String::new();
    let mut max_value = 0u64;
    
    // Generate all ways to partition 1-10 into outer (5) and inner (5) sets
    // For 16-digit solution, 10 must be in outer ring (so we get single digits)
    let mut nums: Vec<u8> = (1..=10).collect();
    
    generate_partitions(&mut nums, 0, 5, &mut |outer: &[u8], inner: &[u8]| {
        // 10 must be in outer ring for 16-digit string
        if !outer.contains(&10) {
            return;
        }
        
        // Try all permutations of inner ring
        let mut inner_vec = inner.to_vec();
        permute_inner(&mut inner_vec, 0, &mut |inner_perm: &[u8]| {
            // Try all permutations of outer ring
            let mut outer_vec = outer.to_vec();
            permute_outer(&mut outer_vec, 0, &mut |outer_perm: &[u8]| {
                if is_magic_5gon(outer_perm, inner_perm) {
                    let solution = generate_string(outer_perm, inner_perm);
                    if solution.len() == 16 {
                        if let Ok(val) = solution.parse::<u64>() {
                            if val > max_value {
                                max_value = val;
                                max_solution = solution;
                            }
                        }
                    }
                }
            });
        });
    });
    
    println!("Maximum 16-digit string: {}", max_solution);
}

fn generate_partitions<F>(nums: &mut [u8], start: usize, outer_size: usize, callback: &mut F)
where
    F: FnMut(&[u8], &[u8]),
{
    if outer_size == 0 {
        callback(&nums[0..start], &nums[start..]);
        return;
    }
    
    for i in start..nums.len() {
        nums.swap(start, i);
        generate_partitions(nums, start + 1, outer_size - 1, callback);
        nums.swap(start, i);
    }
}

fn permute_inner<F>(nums: &mut [u8], start: usize, callback: &mut F)
where
    F: FnMut(&[u8]),
{
    if start == nums.len() {
        callback(nums);
        return;
    }
    
    for i in start..nums.len() {
        nums.swap(start, i);
        permute_inner(nums, start + 1, callback);
        nums.swap(start, i);
    }
}

fn permute_outer<F>(nums: &mut [u8], start: usize, callback: &mut F)
where
    F: FnMut(&[u8]),
{
    if start == nums.len() {
        callback(nums);
        return;
    }
    
    for i in start..nums.len() {
        nums.swap(start, i);
        permute_outer(nums, start + 1, callback);
        nums.swap(start, i);
    }
}

fn is_magic_5gon(outer: &[u8], inner: &[u8]) -> bool {
    let sum = outer[0] as u16 + inner[0] as u16 + inner[1] as u16;
    
    for i in 0..5 {
        let line_sum = outer[i] as u16 + inner[i] as u16 + inner[(i + 1) % 5] as u16;
        if line_sum != sum {
            return false;
        }
    }
    
    true
}

fn generate_string(outer: &[u8], inner: &[u8]) -> String {
    let min_idx = (0..5).min_by_key(|&i| outer[i]).unwrap();
    
    let mut result = String::new();
    for i in 0..5 {
        let idx = (min_idx + i) % 5;
        result.push_str(&outer[idx].to_string());
        result.push_str(&inner[idx].to_string());
        result.push_str(&inner[(idx + 1) % 5].to_string());
    }
    
    result
}
