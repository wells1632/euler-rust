use num_bigint::BigInt;
use num_traits::Zero;

fn main() {
    let mut seeds: Vec<(BigInt, BigInt)> = Vec::new();
    for u in 1i64..=100 {
        for v in 0i64..=100 {
            if u * u - 5 * v * v == 44 {
                seeds.push((BigInt::from(u), BigInt::from(v)));
            }
        }
    }

    println!("Seed solutions to u^2 - 5v^2 = 44:");
    for (u, v) in &seeds {
        println!("  u={}, v={}", u, v);
    }

    let nine   = BigInt::from(9i64);
    let twenty = BigInt::from(20i64);
    let four   = BigInt::from(4i64);
    let five   = BigInt::from(5i64);
    let seven  = BigInt::from(7i64);

    let mut nuggets: Vec<BigInt> = Vec::new();

    // Collect from all seed chains, enough iterations to get well past 30
    for (u0, v0) in &seeds {
        let mut u: BigInt = u0.clone();
        let mut v: BigInt = v0.clone();

        for _ in 0..200 {
            let u_minus_7 = &u - &seven;
            if u_minus_7 > BigInt::zero() && &u_minus_7 % &five == BigInt::zero() {
                let n = &u_minus_7 / &five;

                // x positive requires v > 1+n
                if &v > &(BigInt::from(1i64) + &n) && n > BigInt::zero() {
                    nuggets.push(n.clone());
                }
            }

            let u_next = &nine * &u + &twenty * &v;
            let v_next = &four  * &u + &nine  * &v;
            u = u_next;
            v = v_next;
        }
    }

    nuggets.sort();
    nuggets.dedup();
    
    println!("\nAll nuggets found (first 35):");
    for (i, n) in nuggets.iter().take(35).enumerate() {
        println!("  {}: {}", i + 1, n);
    }

    if nuggets.len() < 30 {
        println!("ERROR: Only found {} nuggets, need 30", nuggets.len());
        return;
    }

    nuggets.truncate(30);
    println!("\nFirst 30 golden nuggets:");
    let mut sum = BigInt::zero();
    for (i, n) in nuggets.iter().enumerate() {
        println!("  {}: {}", i + 1, n);
        sum += n;
    }
    println!("\nSum of first 30 golden nuggets: {}", sum);
}
