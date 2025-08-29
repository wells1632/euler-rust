fn main() {
    'outer: for i in 1..998 {
        for j in 2..997 {
            for k in 3..996 {
                if (i*i + j*j) == k*k {
                    if i+j+k == 1000 {
                        println!("Answer: {}", i*j*k);
                        break 'outer;
                    }
                }
            }
        }
    }
}
