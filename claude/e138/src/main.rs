use num_bigint::BigInt;
use num_traits::{Zero, One};

fn main() {
    let mut results: Vec<(BigInt, BigInt, BigInt)> = Vec::new();

    let mut u = BigInt::from(2);
    let mut v = BigInt::from(1);

    let nine   = BigInt::from(9);
    let twenty = BigInt::from(20);
    let four   = BigInt::from(4);
    let two    = BigInt::from(2);
    let zero   = BigInt::zero();
    let one    = BigInt::one();

    // u^2 - 5v^2 = -1
    // Case 1: h = b+1
    //   b = 2k, h = 2k+1
    //   4L^2 = b^2 + 4h^2 = 4k^2 + 4(2k+1)^2
    //   L^2  = k^2 + (2k+1)^2
    //   Let u = 2L, v = 2k+1 => u^2 = 4L^2 = 4k^2+4(2k+1)^2... 
    //   Actually: u^2 - 5v^2 = -1
    //   4L^2 - 5(2k+1)^2 = -1
    //   4L^2 = 5(2k+1)^2 - 1
    //   So: L = u/2, k = (v-1)/2, b = v-1, h = v
    //   Verify n=1: u=38,v=17 => L=19, k=8, b=16, h=17
    //   Check: L^2 = 361, k^2+h^2 = 64+289 = 353. Not equal!
    //
    // Let me re-derive from scratch using n=0 known solution b=16,L=17,h=15:
    //   k=8, h=15=b-1, L=17
    //   L^2 = 289, k^2+h^2 = 64+225 = 289 ✓
    //   So for Case 2 (h=b-1): k=8, h=15
    //   Try: v=h=15? Then 5v^2-1 = 5*225-1=1124, sqrt=33.5 not 2L=34
    //   Try: v=k=8? Then 5*64-1=319, not perfect square
    //   Try: v=L=17? u^2 = 5*289-1=1444=38^2 ✓  So u=38,v=17 maps to L=v, k=?
    //   u^2 - 5v^2 = -1 with v=L=17, u=38
    //   We need to find b from this: b=16=u/2-1? 38/2-1=18 no
    //   b=16: note u=38=2*19, and h=15, L=17... 
    //   h^2 = L^2 - k^2 = 289-64=225, h=15. And h=b-1=15 ✓
    //   So k = sqrt(L^2 - h^2)... 
    //
    // Let's try a completely different substitution.
    // We know L^2 = k^2 + h^2 with h = 2k±1
    // Case 2 (h=2k-1): L^2 = k^2 + (2k-1)^2 = 5k^2 - 4k + 1
    // Complete the square: L^2 - 5(k - 2/5)^2 = 1 - 4/5 = 1/5
    // Multiply by 5: (sqrt5 * L)^2 - (5k-2)^2 = 1
    // Let X = 5k-2, Y = L: 5Y^2 - X^2 = 1 => X^2 - 5Y^2 = -1 ✓
    // So: X = 5k-2, Y = L
    // From Pell solution (u,v): X=u, Y=v => k=(u+2)/5, L=v
    // Check n=1: u=38,v=17 => k=(38+2)/5=8, L=17, b=16, h=15 ✓✓✓

    // Case 1 (h=2k+1): L^2 = 5k^2 + 4k + 1
    // Let X = 5k+2, Y = L: X^2 - 5Y^2 = (5k+2)^2 - 5L^2 = 25k^2+20k+4-25k^2-20k-5 = -1 ✓
    // So: X = 5k+2, Y = L
    // From Pell solution (u,v): X=u, Y=v => k=(u-2)/5, L=v
    // Check n=1: u=38,v=17 => k=(38-2)/5=36/5 not integer
    // Check n=2: u=682,v=305 => k=(682-2)/5=136, L=305, b=272, h=273
    //   Verify: k^2+h^2 = 136^2+273^2 = 18496+74529 = 93025 = 305^2 ✓✓✓

    // So Case 2 uses solutions where (u+2) % 5 == 0
    // And Case 1 uses solutions where (u-2) % 5 == 0

    let five = BigInt::from(5);

    for _ in 0..50 {
        // Case 2: h = b-1, k=(u+2)/5, L=v
        if (&u + &two) % &five == zero {
            let k = (&u + &two) / &five;
            let l = &v;
            let b = &k * &two;
            let h = &b - &one;
            if k > zero && h > zero {
                results.push((b, l.clone(), h));
            }
        }

        // Case 1: h = b+1, k=(u-2)/5, L=v
        if (&u - &two) % &five == zero {
            let k = (&u - &two) / &five;
            let l = &v;
            let b = &k * &two;
            let h = &b + &one;
            if k > zero {
                results.push((b, l.clone(), h));
            }
        }

        let u_next = &nine * &u + &twenty * &v;
        let v_next = &four * &u + &nine  * &v;
        u = u_next;
        v = v_next;

        if results.len() >= 12 {
            break;
        }
    }

    results.sort_by_key(|r| r.1.clone());
    results.truncate(12);

    println!("The 12 smallest isosceles triangles (b, L, h):");
    let mut sum = BigInt::zero();
    for (b, l, h) in &results {
        println!("  b = {}, L = {}, h = {}", b, l, h);
        sum += l;
    }
    println!("\nSum of L: {}", sum);
}
