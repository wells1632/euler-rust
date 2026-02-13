use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: {} <network_file.csv>", args[0]);
        eprintln!("Example: {} network.txt", args[0]);
        std::process::exit(1);
    }
    
    let filename = &args[1];
    
    match process_network(filename) {
        Ok(savings) => {
            println!("\n{}", "=".repeat(70));
            println!("RESULT: Maximum savings = {}", savings);
            println!("{}", "=".repeat(70));
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

fn process_network(filename: &str) -> Result<u32, Box<dyn std::error::Error>> {
    println!("Reading network from: {}\n", filename);
    
    let matrix = read_matrix(filename)?;
    let n = matrix.len();
    
    if n == 0 {
        return Err("Empty matrix".into());
    }
    
    println!("Network size: {}x{} vertices", n, n);
    
    // Extract edges
    let mut edges = Vec::new();
    let mut total_weight = 0u32;
    
    for i in 0..n {
        for j in i+1..n {
            if let Some(weight) = matrix[i][j] {
                edges.push(Edge { from: i, to: j, weight });
                total_weight += weight;
            }
        }
    }
    
    println!("Total edges: {}", edges.len());
    println!("Total weight: {}", total_weight);
    
    if edges.is_empty() {
        return Err("No edges in network".into());
    }
    
    // Find MST
    let mst_weight = kruskal_mst(n, &edges);
    let mst_edges = n - 1;
    
    println!("\nMST Analysis:");
    println!("  MST edges: {}", mst_edges);
    println!("  MST weight: {}", mst_weight);
    println!("  Removable edges: {}", edges.len() - mst_edges);
    
    let savings = total_weight - mst_weight;
    println!("  Weight saved: {}", savings);
    
    Ok(savings)
}

fn read_matrix(filename: &str) -> Result<Vec<Vec<Option<u32>>>, Box<dyn std::error::Error>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut matrix = Vec::new();
    
    for (line_num, line) in reader.lines().enumerate() {
        let line = line?;
        let line = line.trim();
        
        if line.is_empty() {
            continue;
        }
        
        let row: Vec<Option<u32>> = line
            .split(',')
            .enumerate()
            .map(|(col, s)| {
                let s = s.trim();
                if s == "-" || s.is_empty() {
                    None
                } else {
                    match s.parse::<u32>() {
                        Ok(val) => Some(val),
                        Err(_) => {
                            eprintln!("Warning: Invalid value '{}' at row {}, col {}", 
                                     s, line_num + 1, col + 1);
                            None
                        }
                    }
                }
            })
            .collect();
        
        matrix.push(row);
    }
    
    Ok(matrix)
}

#[derive(Debug, Clone, Copy)]
struct Edge {
    from: usize,
    to: usize,
    weight: u32,
}

fn kruskal_mst(n: usize, edges: &[Edge]) -> u32 {
    let mut sorted_edges = edges.to_vec();
    sorted_edges.sort_by_key(|e| e.weight);
    
    let mut uf = UnionFind::new(n);
    let mut mst_weight = 0u32;
    let mut edges_added = 0;
    
    println!("\nMST edges:");
    
    for edge in sorted_edges {
        if uf.union(edge.from, edge.to) {
            println!("  {} -- {} : {}", edge.from, edge.to, edge.weight);
            mst_weight += edge.weight;
            edges_added += 1;
            
            if edges_added == n - 1 {
                break;
            }
        }
    }
    
    if edges_added < n - 1 {
        eprintln!("\nWarning: Graph is not connected! Only found {} edges for {} vertices", 
                  edges_added, n);
    }
    
    mst_weight
}

struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
            rank: vec![0; n],
        }
    }
    
    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }
    
    fn union(&mut self, x: usize, y: usize) -> bool {
        let root_x = self.find(x);
        let root_y = self.find(y);
        
        if root_x == root_y {
            return false;
        }
        
        if self.rank[root_x] < self.rank[root_y] {
            self.parent[root_x] = root_y;
        } else if self.rank[root_x] > self.rank[root_y] {
            self.parent[root_y] = root_x;
        } else {
            self.parent[root_y] = root_x;
            self.rank[root_x] += 1;
        }
        
        true
    }
}
