fn count_not_divisible(n: u64, p: u64) -> u64 {
    if n == 0 {
        return 0;
    }

    // Extract base-p digits of n (least significant first)
    let mut digits = Vec::new();
    let mut tmp = n;
    while tmp > 0 {
        digits.push(tmp % p);
        tmp /= p;
    }
    // Process most significant digit first
    digits.reverse();

    // Iteratively compute f(n) using the recurrence:
    // f(d * p^k + r) = T(d) * f(p^k) + (d+1) * f(r)
    // We process digit by digit from most significant to least.
    //
    // At each step we track:
    //   result: accumulated count so far
    //   multiplier: the (d+1) factor chain from previous digits
    //
    // After processing digit d at position k (p^k block size):
    //   new_result = result + T(d) * f(p^k)
    //   f(p^k) = (p*(p+1)/2)^k  -- but we track it as we go
    //   new_multiplier = multiplier * (d+1)

    // f(p^k) for the current number of remaining digits
    // starts at f(p^num_digits) and we use it top-down
    // Actually, let's think differently:
    //
    // We walk digits left to right. We maintain `acc` = f(value of digits seen so far).
    // For each new digit d:
    //   acc = T(d) * f_block + (d+1) * acc
    // where f_block = (p*(p+1)/2) (i.e., f(p^1) = p*(p+1)/2)
    // Wait — f_block at each level is f(p^1) only for the last level.
    // At each level going right, the block size shrinks by one power of p.
    //
    // Cleaner: process digits LEFT to RIGHT.
    // acc starts at 0 (= f(0))
    // For each digit d (from most significant to least):
    //   acc = T(d) * fp1 + (d+1) * acc
    //   where fp1 = p*(p+1)/2 ... NO, this isn't right either because
    //   fp1 is the count for a full block at that level.
    //
    // Actually the recurrence telescopes cleanly left-to-right:
    // Think of building n digit by digit. If current value is V and we
    // append digit d (i.e., new value = V*p + d+1, representing first V*p+d+1 rows):
    //
    // f(V*p + d + 1) = f(V*p) + sum_{i=0}^{d} f_row(V*p + i)
    //                  -- not obviously simpler
    //
    // Let's just go with the clean known formula:
    // Process digits of n in base p from MSD to LSD.
    // Maintain running total. At each step with digit d and remaining depth k:
    //   contribution = T(d) * (p*(p+1)/2)^k
    //   carry multiplier = (d+1)
    //
    // So:

    let fp1 = p * (p + 1) / 2; // f(p) = number of non-div entries in p rows = 1+2+...+p = p*(p+1)/2

    let mut result: u64 = 0;
    let mut multiplier: u64 = 1; // product of (d_i + 1) for digits seen so far

    let num_digits = digits.len();

    for (i, &d) in digits.iter().enumerate() {
        let remaining_depth = (num_digits - 1 - i) as u32;
        // f(p^remaining_depth) = fp1^remaining_depth
        let f_block = fp1.pow(remaining_depth);
        let t_d = d * (d + 1) / 2;
        result += multiplier * t_d * f_block;
        multiplier *= d + 1;
    }

    // Add the contribution of the final remainder (which is 0 rows, f(0)=0)
    // plus the trailing multiplier * f(0) = 0, so nothing to add.
    // But we need to add multiplier * f(0+1)? No — n itself is exact.
    // Actually we've been computing f(n) where n is the number formed by digits.
    // The last step leaves f(0)*multiplier = 0. Correct.

    result
}

fn main() {
    let p: u64 = 7;

    println!("First 7 rows:           {}", count_not_divisible(7, p));
    println!("First 100 rows:         {}", count_not_divisible(100, p));
    println!("First 1_000_000_000 rows: {}", count_not_divisible(1_000_000_000, p));
}
