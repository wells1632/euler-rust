use std::time::Instant;
// =================================================================
//  T(n) — Count all observable triangles in the equilateral triangle
//  with medians drawn in every T(1) building block.
//
//  Coordinate system: skewed integer grid (avoids all irrational arithmetic)
//    T(n) vertices: A=(0,0), B=(12n,0), C=(6n,12n)
//    Scale: each T(1) block occupies a 12×12 skewed unit.
//
//  6 families of lines (all integer coefficients ax+by=c):
//    F0 (horiz):  0x +  1y =  12k,        k in 0..=n-1
//    F1 (left):   2x -  1y =  24k,        k in 0..=n-1
//    F2 (right):  2x +  1y =  24*(k+1),   k in 0..=n-1
//    F3 (medL):   2x -  3y =  24*m,       m in -(n-1)..=n-1
//    F4 (medR):   2x +  3y =  24*(m+1),   m in 0..=2*n-2
//    F5 (vert):   1x +  0y =   6*k,       k in 1..=2*n-1
//
//  A triangle is valid iff:
//    - Each of its 3 edges lies on one of the drawn lines
//      (i.e. each pair of vertices shares a drawn line)
//    - Its 3 vertices are non-collinear
//    - All 3 vertices lie within (or on the boundary of) T(n)
//
//  We enumerate line triples (one from each of 3 distinct families),
//  compute the 3 pairwise intersection points, and check validity.
//  This is O(n^3) per family-triple, 20 triples total → O(n^3) overall.
// =================================================================

/// A line ax + by = c (integer coefficients).
#[derive(Clone, Copy, Debug)]
struct Line { a: i64, b: i64, c: i64 }

/// Rational number p/q, kept in a form where q > 0 (but NOT necessarily reduced,
/// since we only need exact equality and comparison, not a canonical form for hashing).
#[derive(Clone, Copy, Debug)]
struct Rat { p: i64, q: i64 }

impl Rat {
    fn new(p: i64, q: i64) -> Self {
        debug_assert!(q != 0);
        if q < 0 { Rat { p: -p, q: -q } } else { Rat { p, q } }
    }
    fn is_nonneg(self) -> bool { self.p >= 0 }  // q > 0 always
    // self <= other  ↔  p1*q2 <= p2*q1  (q1,q2 > 0)
    //    fn le(self, other: Self) -> bool { self.p * other.q <= other.p * self.q }
}

/// Intersect two lines. Returns None if parallel (det=0).
fn intersect(l1: Line, l2: Line) -> Option<(Rat, Rat)> {
    let det = l1.a * l2.b - l2.a * l1.b;
    if det == 0 { return None; }
    let x = Rat::new(l1.c * l2.b - l2.c * l1.b, det);
    let y = Rat::new(l1.a * l2.c - l2.a * l1.c, det);
    Some((x, y))
}

/// Is rational point (x,y) inside or on the boundary of T(n)?
/// T(n) vertices: (0,0), (12n,0), (6n,12n)  in skewed coords.
/// Constraints:
///   y >= 0
///   2x - y >= 0      (left edge: y <= 2x)
///   2x + y <= 24n    (right edge)
fn in_triangle(x: Rat, y: Rat, n: i64) -> bool {
    // y >= 0
    if !y.is_nonneg() { return false; }
    // 2x - y >= 0  ↔  2*(xp/xq) - (yp/yq) >= 0
    //               ↔  2*xp*yq - yp*xq >= 0  (xq,yq > 0)
    let lhs2 = 2 * x.p * y.q - y.p * x.q;
    if lhs2 < 0 { return false; }
    // 2x + y <= 24n  ↔  2*xp*yq + yp*xq <= 24n*xq*yq
    let lhs3 = 2 * x.p * y.q + y.p * x.q;
    let rhs3 = 24 * n * x.q * y.q;
    if lhs3 > rhs3 { return false; }
    true
}

/// Are three rational points collinear?
/// Cross product (B-A) × (C-A) == 0.
fn collinear(
    (ax, ay): (Rat, Rat),
    (bx, by): (Rat, Rat),
    (cx, cy): (Rat, Rat),
) -> bool {
    // (bx-ax, by-ay) × (cx-ax, cy-ay)
    // = (bx-ax)(cy-ay) - (by-ay)(cx-ax) == 0
    // Each difference: (p1/q1 - p2/q2) = (p1*q2 - p2*q1)/(q1*q2)
    // Cross product numerator (ignoring positive denominator):
    // (bxp*axq - axp*bxq) * (cyp*ayq - ayp*cyq)
    //   - (byp*ayq - ayp*byq) * (cxp*axq - axp*cxq)  == 0
    let bax = bx.p * ax.q - ax.p * bx.q;  // numerator of (bx - ax)
    let bay = by.p * ay.q - ay.p * by.q;
    let cax = cx.p * ax.q - ax.p * cx.q;
    let cay = cy.p * ay.q - ay.p * cy.q;
    // Cross = bax*(q_bx*q_ax) ... actually denominators are all positive,
    // so sign of cross product is determined by bax*cay - bay*cax:
    // Full cross numerator = bax * cay * (bx.q*ax.q * cy.q*ay.q)
    //                      - bay * cax * (by.q*ay.q * cx.q*ax.q)
    // This overflows for large n. We need to check bax*cay == bay*cax
    // BUT the q factors are different! Let's include them:
    // (bx-ax) = bax / (bx.q*ax.q),  (cy-ay) = cay / (cy.q*ay.q)
    // (by-ay) = bay / (by.q*ay.q),  (cx-ax) = cax / (cx.q*ax.q)
    // cross = bax*cay / (bx.q*ax.q*cy.q*ay.q)
    //       - bay*cax / (by.q*ay.q*cx.q*ax.q)
    // == 0 iff bax*cay * by.q*ay.q*cx.q*ax.q
    //       == bay*cax * bx.q*ax.q*cy.q*ay.q
    // Factor out ax.q:
    // bax*cay * by.q*ay.q*cx.q == bay*cax * bx.q*cy.q*ay.q
    // Divide by ay.q (positive):
    // bax*cay * by.q*cx.q == bay*cax * bx.q*cy.q
    //
    // For our problem all q values are |det| values which could be large,
    // but for n<=36 the values stay within i128 range.
    let lhs = (bax as i128) * (cay as i128) * (by.q as i128) * (cx.q as i128);
    let rhs = (bay as i128) * (cax as i128) * (bx.q as i128) * (cy.q as i128);
    lhs == rhs
}

/// Generate all lines for T(n).
fn generate_lines(n: i64) -> Vec<Line> {
    let mut lines = Vec::new();
    // F0: y = 12k, k=0..n-1  →  0x + 1y = 12k
    for k in 0..n {
        lines.push(Line { a: 0, b: 1, c: 12 * k });
    }
    // F1: 2x - y = 24k, k=0..n-1
    for k in 0..n {
        lines.push(Line { a: 2, b: -1, c: 24 * k });
    }
    // F2: 2x + y = 24*(k+1), k=0..n-1
    for k in 0..n {
        lines.push(Line { a: 2, b: 1, c: 24 * (k + 1) });
    }
    // F3: 2x - 3y = 24*m, m=-(n-1)..=n-1
    for m in -(n - 1)..=n - 1 {
        lines.push(Line { a: 2, b: -3, c: 24 * m });
    }
    // F4: 2x + 3y = 24*(m+1), m=0..=2*n-2
    for m in 0..=2 * n - 2 {
        lines.push(Line { a: 2, b: 3, c: 24 * (m + 1) });
    }
    // F5: x = 6k, k=1..=2*n-1
    for k in 1..=2 * n - 1 {
        lines.push(Line { a: 1, b: 0, c: 6 * k });
    }
    lines
}

fn count_triangles(n: i64) -> u64 {
    let lines = generate_lines(n);
    let nl = lines.len();
    let mut count = 0u64;

    // Enumerate all triples of lines from distinct families.
    // Two lines from the same family are parallel (same a:b ratio) → no intersection.
    // We can just try all C(nl,3) triples and skip parallel pairs.
    // For n=36: nl = 9*36-3 = 321, C(321,3) ≈ 5.5M — fast enough.
    for i in 0..nl {
        for j in i + 1..nl {
            let pij = match intersect(lines[i], lines[j]) {
                Some(p) => p,
                None => continue,
            };
            if !in_triangle(pij.0, pij.1, n) { continue; }

            for k in j + 1..nl {
                let pik = match intersect(lines[i], lines[k]) {
                    Some(p) => p,
                    None => continue,
                };
                if !in_triangle(pik.0, pik.1, n) { continue; }

                let pjk = match intersect(lines[j], lines[k]) {
                    Some(p) => p,
                    None => continue,
                };
                if !in_triangle(pjk.0, pjk.1, n) { continue; }

                if !collinear(pij, pik, pjk) {
                    count += 1;
                }
            }
        }
    }
    count
}

fn main() {
    let start = Instant::now();
    println!("Verifying known values:");
    let t1 = count_triangles(1);
    let t2 = count_triangles(2);
    println!("T(1) = {} (expected 16)", t1);
    println!("T(2) = {} (expected 104)", t2);

    if t1 == 16 && t2 == 104 {
        println!("\nVerification passed! Computing T(3)..T(36):");
        for n in 3..=36 {
            println!("T({:2}) = {}", n, count_triangles(n));
        }
    } else {
        println!("Verification FAILED — check line families.");
    }

    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);
}
