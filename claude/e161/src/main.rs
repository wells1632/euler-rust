use std::time::Instant;
use std::collections::HashMap;

// ── Brute-force solver (backtracking) for small grids ──────────────────────

const RAW_SHAPES: &[[(i32, i32); 3]] = &[
    [(0,0),(0,1),(0,2)],
    [(0,0),(1,0),(2,0)],
    [(0,0),(1,0),(1,1)],
    [(0,0),(0,1),(1,0)],
    [(0,0),(0,1),(1,1)],
    [(0,0),(1,-1),(1,0)],
];

fn brute_force(rows: usize, cols: usize) -> u64 {
    let cells = rows * cols;
    let mut grid = vec![false; cells];

    // Precompute: for each cell, all placements (as sorted flat-index triples)
    // where that cell is the reading-order minimum.
    let mut placements_at: Vec<Vec<[usize; 3]>> = vec![vec![]; cells];
    for anchor in 0..cells {
        let r0 = (anchor / cols) as i32;
        let c0 = (anchor % cols) as i32;
        for shape in RAW_SHAPES {
            let mut abs = [(0i32,0i32); 3];
            let mut ok = true;
            for (i, &(dr,dc)) in shape.iter().enumerate() {
                let r = r0+dr; let c = c0+dc;
                if r<0||r>=rows as i32||c<0||c>=cols as i32 { ok=false; break; }
                abs[i] = (r,c);
            }
            if !ok { continue; }
            let mut flat = [0usize;3];
            for (i,&(r,c)) in abs.iter().enumerate() {
                flat[i] = r as usize*cols + c as usize;
            }
            if *flat.iter().min().unwrap() != anchor { continue; }
            flat.sort();
            placements_at[anchor].push(flat);
        }
    }

    fn recurse(
        pos: usize, grid: &mut Vec<bool>,
        placements_at: &Vec<Vec<[usize;3]>>,
        cells: usize,
    ) -> u64 {
        // Advance past filled cells
        let mut p = pos;
        while p < cells && grid[p] { p += 1; }
        if p == cells { return 1; }
        let mut count = 0;
        for piece in &placements_at[p] {
            if piece.iter().any(|&c| grid[c]) { continue; }
            for &c in piece { grid[c] = true; }
            count += recurse(p+1, grid, placements_at, cells);
            for &c in piece { grid[c] = false; }
        }
        count
    }

    recurse(0, &mut grid, &placements_at, cells)
}

// ── DP solver ──────────────────────────────────────────────────────────────

fn build_placements_dp(rows: usize, cols: usize) -> (Vec<Vec<u64>>, usize) {
    let cells = rows * cols;
    // Maximum flat-index offset from anchor to any cell in a triomino.
    // I-horizontal: COLS+1 offset possible if we span a row? No —
    // a horizontal piece stays in one row, so max offset = 2.
    // Vertical: offset = 2*COLS.
    // L-shapes: max offset = COLS+1.
    // So worst case is 2*COLS for the vertical I-piece.
    let window = 2 * cols + 1;
    let mut placements_at: Vec<Vec<u64>> = vec![vec![]; cells];

    for anchor in 0..cells {
        let r0 = (anchor / cols) as i32;
        let c0 = (anchor % cols) as i32;
        for shape in RAW_SHAPES {
            let mut abs = [(0i32,0i32);3];
            let mut ok = true;
            for (i,&(dr,dc)) in shape.iter().enumerate() {
                let r=r0+dr; let c=c0+dc;
                if r<0||r>=rows as i32||c<0||c>=cols as i32 { ok=false; break; }
                abs[i]=(r,c);
            }
            if !ok { continue; }
            let mut flat=[0usize;3];
            for (i,&(r,c)) in abs.iter().enumerate() {
                flat[i]=r as usize*cols+c as usize;
            }
            if *flat.iter().min().unwrap() != anchor { continue; }
            let offsets: Vec<usize> = flat.iter().map(|&f| f-anchor).collect();
            if offsets.iter().any(|&o| o >= window) { continue; }
            let mask: u64 = offsets.iter().fold(0u64, |a,&o| a|(1<<o));
            placements_at[anchor].push(mask);
        }
    }
    (placements_at, window)
}

fn dp_solve(rows: usize, cols: usize) -> u64 {
    let cells = rows * cols;
    let (placements_at, _window) = build_placements_dp(rows, cols);

    let mut dp: HashMap<u64,u64> = HashMap::new();
    dp.insert(0,1);

    for pos in 0..cells {
        let mut next: HashMap<u64,u64> = HashMap::new();
        for (&profile, &ways) in &dp {
            if profile & 1 == 1 {
                *next.entry(profile>>1).or_insert(0) += ways;
            } else {
                for &pmask in &placements_at[pos] {
                    if profile & pmask == 0 {
                        *next.entry((profile|pmask)>>1).or_insert(0) += ways;
                    }
                }
            }
        }
        dp = next;
    }
    *dp.get(&0).unwrap_or(&0)
}

// ── Main: cross-check brute force vs DP on all small grids ─────────────────

fn main() {
    let start = Instant::now();
    // Test all grids up to ~18 cells where both solvers are fast
    let test_cases = vec![
        (1,3),(3,1),(1,6),(6,1),(1,9),(9,1),(2,3),(3,2),
        (2,6),(6,2),(2,9),(9,2),(3,3),(3,4),(4,3),(3,6),(6,3),
    ];

    let mut all_pass = true;
    for (r,c) in &test_cases {
        let bf = brute_force(*r,*c);
        let dp = dp_solve(*r,*c);
        let pass = bf==dp;
        if !pass { all_pass = false; }
        println!("{}x{}: brute={} dp={} {}",r,c,bf,dp,if pass{"PASS"}else{"FAIL"});
    }

    if all_pass {
        println!("\nAll small cases pass — running 9x12...");
        println!("9x12 tilings: {}", dp_solve(9,12));
    } else {
        println!("\nDP is wrong on the above — fix before scaling up.");
    }

    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);
}
