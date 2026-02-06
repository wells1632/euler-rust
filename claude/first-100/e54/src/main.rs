use std::collections::HashMap;
use std::env;
use std::fs;
use std::process;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Rank {
    Two = 2, Three, Four, Five, Six, Seven, Eight, Nine, Ten,
    Jack, Queen, King, Ace,
}

impl Rank {
    fn from_char(c: char) -> Self {
        match c {
            '2' => Rank::Two, '3' => Rank::Three, '4' => Rank::Four,
            '5' => Rank::Five, '6' => Rank::Six, '7' => Rank::Seven,
            '8' => Rank::Eight, '9' => Rank::Nine, 'T' => Rank::Ten,
            'J' => Rank::Jack, 'Q' => Rank::Queen, 'K' => Rank::King,
            'A' => Rank::Ace,
            _ => panic!("Invalid rank"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Card {
    rank: Rank,
    suit: char,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandRank {
    HighCard(Vec<Rank>),
    OnePair(Rank, Vec<Rank>),
    TwoPair(Rank, Rank, Rank),
    ThreeOfKind(Rank, Vec<Rank>),
    Straight(Rank),
    Flush(Vec<Rank>),
    FullHouse(Rank, Rank),
    FourOfKind(Rank, Rank),
    StraightFlush(Rank),
    RoyalFlush,
}

fn parse_card(s: &str) -> Card {
    let chars: Vec<char> = s.chars().collect();
    Card { rank: Rank::from_char(chars[0]), suit: chars[1] }
}

fn evaluate_hand(cards: &[Card]) -> HandRank {
    let mut rank_counts: HashMap<Rank, usize> = HashMap::new();
    for card in cards {
        *rank_counts.entry(card.rank).or_insert(0) += 1;
    }
    
    let mut ranks: Vec<Rank> = cards.iter().map(|c| c.rank).collect();
    ranks.sort_by(|a, b| b.cmp(a));
    
    let is_flush = cards.iter().all(|c| c.suit == cards[0].suit);
    
    let is_straight = {
        let mut sorted = ranks.clone();
        sorted.sort();
        sorted.windows(2).all(|w| w[1] as u8 == w[0] as u8 + 1) ||
        (sorted == vec![Rank::Two, Rank::Three, Rank::Four, Rank::Five, Rank::Ace])
    };
    
    let mut counts: Vec<(usize, Rank)> = rank_counts.iter()
        .map(|(&r, &c)| (c, r))
        .collect();
    counts.sort_by(|a, b| b.cmp(a));
    
    // Check for Royal Flush (10-J-Q-K-A of same suit)
    if is_straight && is_flush {
        let mut sorted = ranks.clone();
        sorted.sort();
        if sorted == vec![Rank::Ten, Rank::Jack, Rank::Queen, Rank::King, Rank::Ace] {
            return HandRank::RoyalFlush;
        }
        
        let high = if ranks.contains(&Rank::Ace) && ranks.contains(&Rank::Two) {
            Rank::Five
        } else {
            ranks[0]
        };
        return HandRank::StraightFlush(high);
    }
    
    if counts[0].0 == 4 {
        let kicker = counts[1].1;
        return HandRank::FourOfKind(counts[0].1, kicker);
    }
    
    if counts[0].0 == 3 && counts[1].0 == 2 {
        return HandRank::FullHouse(counts[0].1, counts[1].1);
    }
    
    if is_flush {
        return HandRank::Flush(ranks);
    }
    
    if is_straight {
        let high = if ranks.contains(&Rank::Ace) && ranks.contains(&Rank::Two) {
            Rank::Five
        } else {
            ranks[0]
        };
        return HandRank::Straight(high);
    }
    
    if counts[0].0 == 3 {
        let mut kickers: Vec<Rank> = counts[1..].iter().map(|&(_, r)| r).collect();
        kickers.sort_by(|a, b| b.cmp(a)); // Sort descending by rank
        return HandRank::ThreeOfKind(counts[0].1, kickers);
    }
    
    if counts[0].0 == 2 && counts[1].0 == 2 {
        let (pair1, pair2) = if counts[0].1 > counts[1].1 {
            (counts[0].1, counts[1].1)
        } else {
            (counts[1].1, counts[0].1)
        };
        return HandRank::TwoPair(pair1, pair2, counts[2].1);
    }
    
    if counts[0].0 == 2 {
        let mut kickers: Vec<Rank> = counts[1..].iter().map(|&(_, r)| r).collect();
        kickers.sort_by(|a, b| b.cmp(a)); // Sort descending by rank
        return HandRank::OnePair(counts[0].1, kickers);
    }
    
    HandRank::HighCard(ranks)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        eprintln!("Example: {} poker_hands.txt", args[0]);
        process::exit(1);
    }
    
    let filename = &args[1];
    
    let data = fs::read_to_string(filename)
        .unwrap_or_else(|err| {
            eprintln!("Error reading file '{}': {}", filename, err);
            process::exit(1);
        });
    
    let lines: Vec<&str> = data.lines().collect();
    let mut player1_wins = 0;
    let mut player2_wins = 0;
    let mut ties = 0;
    let mut total_hands = 0;
    let mut line_number = 0;
    
    for line in lines {
        line_number += 1;
        
        if line.trim().is_empty() {
            continue;
        }
        
        let cards_str: Vec<&str> = line.split_whitespace().collect();
        
        if cards_str.len() != 10 {
            eprintln!("Warning: Invalid line (expected 10 cards): {}", line);
            continue;
        }
        
        let player1_cards: Vec<Card> = cards_str[0..5].iter()
            .map(|&s| parse_card(s))
            .collect();
        let player2_cards: Vec<Card> = cards_str[5..10].iter()
            .map(|&s| parse_card(s))
            .collect();
        
        let hand1 = evaluate_hand(&player1_cards);
        let hand2 = evaluate_hand(&player2_cards);
        
        // Check for Royal Flush
        if hand1 == HandRank::RoyalFlush {
            println!("🎉 ROYAL FLUSH detected for Player 1 on line {}!", line_number);
            println!("   Cards: {}", &cards_str[0..5].join(" "));
        }
        if hand2 == HandRank::RoyalFlush {
            println!("🎉 ROYAL FLUSH detected for Player 2 on line {}!", line_number);
            println!("   Cards: {}", &cards_str[5..10].join(" "));
        }
        
        if hand1 > hand2 {
            player1_wins += 1;
        } else if hand1 < hand2 {
            player2_wins += 1;
        } else {
            ties += 1;
        }
        
        total_hands += 1;
    }
    
    println!("\nResults:");
    println!("--------");
    println!("Total hands played: {}", total_hands);
    println!("Player 1 wins: {} ({:.1}%)", player1_wins, 
             (player1_wins as f64 / total_hands as f64) * 100.0);
    println!("Player 2 wins: {} ({:.1}%)", player2_wins,
             (player2_wins as f64 / total_hands as f64) * 100.0);
    println!("Ties: {} ({:.1}%)", ties,
             (ties as f64 / total_hands as f64) * 100.0);
}
