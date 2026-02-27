fn main() {
    let turns = 15;
    let win_probability = calculate_winning_probability(turns);
    
    println!("Winning probability for {} turns: {}", turns, win_probability);
    
    // Maximum prize fund is floor(1 / probability)
    let max_prize = (1.0 / win_probability).floor() as u64;
    
    println!("Maximum prize fund: ${}", max_prize);
}

fn calculate_winning_probability(turns: usize) -> f64 {
    // Need more than half blues to win
    let blues_needed = turns / 2 + 1;
    
    let mut total_probability = 0.0;
    
    // Calculate probability for each possible number of blues >= blues_needed
    for num_blues in blues_needed..=turns {
        let prob = probability_exact_blues(turns, num_blues);
        total_probability += prob;
    }
    
    total_probability
}

fn probability_exact_blues(turns: usize, num_blues: usize) -> f64 {
    // Use dynamic programming to calculate probability
    // At turn i (0-indexed), there are (i+1) red discs and 1 blue disc
    // P(blue at turn i) = 1/(i+2)
    
    let mut prob = 0.0;
    
    // Generate all possible sequences with exactly num_blues blue draws
    generate_sequences(turns, num_blues, 0, 0, 1.0, &mut prob);
    
    prob
}

fn generate_sequences(
    turns: usize,
    target_blues: usize,
    current_turn: usize,
    blues_so_far: usize,
    current_prob: f64,
    total_prob: &mut f64,
) {
    // Base case: completed all turns
    if current_turn == turns {
        if blues_so_far == target_blues {
            *total_prob += current_prob;
        }
        return;
    }
    
    let remaining_turns = turns - current_turn;
    let blues_needed = target_blues - blues_so_far;
    
    // Pruning: check if it's still possible to reach target
    if blues_needed > remaining_turns || blues_needed < 0 {
        return;
    }
    
    // At turn i (0-indexed), there are (i+1) red discs and 1 blue disc
    // Total discs = i+2
    let total_discs = (current_turn + 2) as f64;
    let prob_blue = 1.0 / total_discs;
    let prob_red = (current_turn + 1) as f64 / total_discs;
    
    // Try drawing blue
    if blues_so_far < target_blues {
        generate_sequences(
            turns,
            target_blues,
            current_turn + 1,
            blues_so_far + 1,
            current_prob * prob_blue,
            total_prob,
        );
    }
    
    // Try drawing red
    generate_sequences(
        turns,
        target_blues,
        current_turn + 1,
        blues_so_far,
        current_prob * prob_red,
        total_prob,
    );
}
