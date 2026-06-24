use std::time::Instant;
fn main() {
    let start = Instant::now();
    // Count n-digit hex numbers (no leading zero) containing 0, 1, and A each at least once.
    // First digit: 15 choices (1-F), rest: 16^(n-1) total unrestricted.
    //
    // Use inclusion-exclusion on the 3 required digits {0, 1, A}.
    // Let f(n, forbidden_first, forbidden_any) = count of n-digit numbers where
    //   first digit avoids {0} ∪ forbidden_first, other digits avoid forbidden_any.
    //
    // For a set S ⊆ {0,1,A} missing, by inclusion-exclusion sign is (-1)^|S|.
    //   first digit choices: 15 - |S ∩ {1,A}|   (0 is already excluded from first digit)
    //   other digit choices per position: 16 - |S|
    //   total for this term: max(0, first) * (16-|S|)^(n-1)

    let mut total: i128 = 0;

    for n in 1usize..=16 {
        // Inclusion-exclusion over subsets of {0, 1, A}
        // Represent subsets: bit0=0, bit1=1, bit2=A
        for mask in 0u8..8 {
//            let missing_0 = (mask & 1) != 0;
            let missing_1 = (mask & 2) != 0;
            let missing_a = (mask & 4) != 0;

            let bits = mask.count_ones() as i128;
            let sign: i128 = if bits % 2 == 0 { 1 } else { -1 };

            // First digit: 1-F = 15 choices, then remove missing_1 and missing_A from those
            // (missing_0 doesn't affect first digit since 0 is already excluded)
            let mut first_choices: i128 = 15;
            if missing_1 { first_choices -= 1; }
            if missing_a { first_choices -= 1; }

            if first_choices < 0 { continue; }

            // Remaining n-1 digits: 16 - |missing| choices each
            let remaining_choices: i128 = 16 - bits;
            let remaining_positions = (n as u32).saturating_sub(1);
            let ways = first_choices * remaining_choices.pow(remaining_positions);

            total += sign * ways;
        }
    }

    println!("Decimal: {}", total);
    println!("Hexadecimal: {:X}", total);

    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);
}
