use std::collections::HashMap;

fn main() {
    // State: (a2, a3, a4, a5) counts of sheets in envelope
    // After first batch: (1,1,1,1)
    // We process batches 2 through 16, track envelope state BEFORE each batch
    // We want expected count of single-sheet states for batches 2..=15 (excluding first and last)

    // Use exact rational arithmetic via fractions, or f64 with care
    // State probabilities: HashMap<(u8,u8,u8,u8), f64>

    let mut state_probs: HashMap<(u8, u8, u8, u8), f64> = HashMap::new();
    // After batch 1, envelope is (1,1,1,1)
    state_probs.insert((1, 1, 1, 1), 1.0);

    let mut expected_single = 0.0f64;

    // Batches 2 through 16: 15 batches
    // Exclude first (batch 1 already done) and last (batch 16)
    // So we check envelope state before batches 2..=15 (14 checks)
    // Then also process batch 16 to complete the week (but don't count it)

    for batch in 2..=16 {
        // Check if single sheet (before this batch), count if batch 2..=15
        if batch <= 15 {
            for (&state, &prob) in &state_probs {
                let total = state.0 as u8 + state.1 + state.2 + state.3;
                if total == 1 {
                    expected_single += prob;
                }
            }
        }

        // Now process this batch: pick a random sheet, cut down to A5, use it
        let mut new_probs: HashMap<(u8, u8, u8, u8), f64> = HashMap::new();

        for (&(a2, a3, a4, a5), &prob) in &state_probs {
            let total = a2 + a3 + a4 + a5;
            if total == 0 {
                continue;
            }
            let total_f = total as f64;

            // Pick A5
            if a5 > 0 {
                let p = (a5 as f64) / total_f;
                let new_state = (a2, a3, a4, a5 - 1);
                *new_probs.entry(new_state).or_insert(0.0) += prob * p;
            }
            // Pick A4: cut -> 2xA5, use 1 -> net: -1 A4, +1 A5
            if a4 > 0 {
                let p = (a4 as f64) / total_f;
                let new_state = (a2, a3, a4 - 1, a5 + 1);
                *new_probs.entry(new_state).or_insert(0.0) += prob * p;
            }
            // Pick A3: cut -> 1xA4 + 2xA5, use 1 -> net: -1 A3, +1 A4, +1 A5
            if a3 > 0 {
                let p = (a3 as f64) / total_f;
                let new_state = (a2, a3 - 1, a4 + 1, a5 + 1);
                *new_probs.entry(new_state).or_insert(0.0) += prob * p;
            }
            // Pick A2: cut -> 1xA3 + 1xA4 + 2xA5, use 1 -> net: -1 A2, +1 A3, +1 A4, +1 A5
            if a2 > 0 {
                let p = (a2 as f64) / total_f;
                let new_state = (a2 - 1, a3 + 1, a4 + 1, a5 + 1);
                *new_probs.entry(new_state).or_insert(0.0) += prob * p;
            }
        }

        state_probs = new_probs;
    }

    println!("{:.6}", expected_single);
}
