use std::collections::HashMap;
use rand::Rng;

const BOARD_SIZE: usize = 40;
const NUM_SIMULATIONS: usize = 10_000_000;

struct Deck {
    cards: Vec<i32>,
    position: usize,
}

impl Deck {
    fn new(cards: Vec<i32>) -> Self {
        Deck { cards, position: 0 }
    }
    
    fn draw(&mut self) -> i32 {
        let card = self.cards[self.position];
        self.position = (self.position + 1) % self.cards.len();
        card
    }
}

fn main() {
    let mut rng = rand::thread_rng();
    let mut landing_counts = HashMap::new();
    
    // Chance cards: -1 means no movement, -2 means Go to Jail, -3 means back 3
    // -4 means nearest railroad, -5 means nearest utility
    let chance_cards = vec![
        0,   // Go
        24,  // Illinois Ave
        11,  // St. Charles Place
        -5,  // Nearest Utility
        -4,  // Nearest Railroad
        -4,  // Nearest Railroad
        -3,  // Go back 3
        -2,  // Go to Jail
        -1, -1, -1, -1, -1, -1, -1, -1,  // No movement (8 cards)
    ];
    
    // Community Chest cards
    let cc_cards = vec![
        0,   // Go
        -2,  // Go to Jail
        -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,  // No movement (14 cards)
    ];
    
    let mut chance_deck = Deck::new(chance_cards);
    let mut cc_deck = Deck::new(cc_cards);
    
    let mut position = 0;
    
    for _ in 0..NUM_SIMULATIONS {
        // Roll two 4-sided dice
        let roll = rng.gen_range(1..=4) + rng.gen_range(1..=4);
        position = (position + roll) % BOARD_SIZE;
        
        // Process the square
        position = process_square(position, &mut chance_deck, &mut cc_deck);
        
        *landing_counts.entry(position).or_insert(0) += 1;
    }
    
    // Find top 3 most visited squares
    let mut squares: Vec<_> = landing_counts.iter().collect();
    squares.sort_by(|a, b| b.1.cmp(a.1));
    
    println!("Top 10 most visited squares:");
    for (i, (square, count)) in squares.iter().take(10).enumerate() {
        println!("{}. Square {:02}: {} visits ({:.4}%)", 
                 i + 1, square, count, 
                 (**count as f64 / NUM_SIMULATIONS as f64) * 100.0);
    }
    
    // Create modal string
    let modal_string = format!("{:02}{:02}{:02}", 
                               squares[0].0, squares[1].0, squares[2].0);
    println!("\nModal string (top 3 squares): {}", modal_string);
}

fn process_square(mut pos: usize, chance: &mut Deck, cc: &mut Deck) -> usize {
    // Chance squares: 7, 22, 36
    if pos == 7 || pos == 22 || pos == 36 {
        let card = chance.draw();
        pos = handle_card(pos, card);
    }
    // Community Chest squares: 2, 17, 33
    else if pos == 2 || pos == 17 || pos == 33 {
        let card = cc.draw();
        pos = handle_card(pos, card);
    }
    // Go to Jail square: 30
    else if pos == 30 {
        pos = 10;  // Jail
    }
    
    pos
}

fn handle_card(pos: usize, card: i32) -> usize {
    match card {
        -1 => pos,  // No movement
        -2 => 10,   // Go to Jail
        -3 => {     // Go back 3
            if pos >= 3 {
                pos - 3
            } else {
                BOARD_SIZE + pos - 3
            }
        }
        -4 => {     // Nearest Railroad (5, 15, 25, 35)
            if pos < 5 || pos >= 35 { 5 }
            else if pos < 15 { 15 }
            else if pos < 25 { 25 }
            else { 35 }
        }
        -5 => {     // Nearest Utility (12, 28)
            if pos < 12 || pos >= 28 { 12 }
            else { 28 }
        }
        n if n >= 0 => n as usize,
        _ => pos,
    }
}
