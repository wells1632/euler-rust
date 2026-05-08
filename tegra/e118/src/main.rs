use std::time::Instant;
// main.rs
use std::collections::HashSet;

extern "C" {
    fn launch_find_prime_partitions(
        perms: *const u8,
        num_perms: u32,
        out: *mut u32,
        out_part_lens: *mut u8,
        out_counts: *mut u32,
    );
}

fn permute_collect(arr: &mut [u8], start: usize, result: &mut Vec<Vec<u8>>) {
    if start == arr.len() {
        result.push(arr.to_vec());
        return;
    }
    for i in start..arr.len() {
        arr.swap(start, i);
        permute_collect(arr, start + 1, result);
        arr.swap(start, i);
    }
}

fn main() {
    let start = Instant::now();
    let mut digits = [1u8, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut all_perms: Vec<Vec<u8>> = Vec::new();
    permute_collect(&mut digits, 0, &mut all_perms);

    let num_perms = all_perms.len() as u32;
    let flat_perms: Vec<u8> = all_perms.into_iter().flatten().collect();

    const MAX_PARTS: usize = 64;
    const MAX_PART_NUMS: usize = 9;

    let mut out = vec![0u32; num_perms as usize * MAX_PARTS * MAX_PART_NUMS];
    let mut out_part_lens = vec![0u8; num_perms as usize * MAX_PARTS];
    let mut out_counts = vec![0u32; num_perms as usize];

    unsafe {
        launch_find_prime_partitions(
            flat_perms.as_ptr(),
            num_perms,
            out.as_mut_ptr(),
            out_part_lens.as_mut_ptr(),
            out_counts.as_mut_ptr(),
        );
    }

    let mut all_valid_sets: HashSet<Vec<u32>> = HashSet::new();

    for tid in 0..num_perms as usize {
        let count = out_counts[tid] as usize;
        for p in 0..count {
            let len = out_part_lens[tid * MAX_PARTS + p] as usize;
            let base = tid * MAX_PARTS * MAX_PART_NUMS + p * MAX_PART_NUMS;
            let mut partition: Vec<u32> = out[base..base + len].to_vec();
            partition.sort_unstable();
            all_valid_sets.insert(partition);
        }
    }

    println!("Total permutations checked: {}", num_perms);
    println!("Distinct sets containing only primes: {}", all_valid_sets.len());
    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);
}
