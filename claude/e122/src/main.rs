use std::collections::{HashSet, VecDeque};

fn main() {
    let mut total_sum = 0;
    
    for k in 1..=200 {
        let m_k = minimum_multiplications(k);
        total_sum += m_k;
        
        if k == 15 {
            println!("m(15) = {}", m_k);
        }
    }
    
    println!("Sum of m(k) for 1<=k<=200: {}", total_sum);
}

fn minimum_multiplications(target: usize) -> usize {
    if target == 1 {
        return 0;
    }
    
    // BFS by depth - track all chains at current depth
    let mut current_chains: Vec<Vec<usize>> = vec![vec![1]];
    let mut depth = 0;
    
    loop {
        let mut next_chains: Vec<Vec<usize>> = Vec::new();
        let mut seen_at_depth: HashSet<Vec<usize>> = HashSet::new();
        
        for chain in current_chains {
            // Try all additions
            for i in 0..chain.len() {
                for j in i..chain.len() {
                    let new_val = chain[i] + chain[j];
                    
                    if new_val > target {
                        continue;
                    }
                    
                    if new_val == target {
                        return depth + 1;
                    }
                    
                    if !chain.contains(&new_val) {
                        let mut new_chain = chain.clone();
                        new_chain.push(new_val);
                        new_chain.sort_unstable();
                        
                        if seen_at_depth.insert(new_chain.clone()) {
                            next_chains.push(new_chain);
                        }
                    }
                }
            }
        }
        
        current_chains = next_chains;
        depth += 1;
    }
}
