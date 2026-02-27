fn main() {
    const MAX: usize = 20_000;
    let mut ways = vec![0u32; MAX];

    for w in 1usize.. {
        if 2 * (w + w + 1) >= MAX { break; }

        for h in 1..=w {
            if 2 * (w * h + w + h) >= MAX { break; }

            for l in 1..=h {
                let mut cubes = 2 * (w * h + w * l + h * l);
                let mut n = 0usize;
                while cubes < MAX {
                    ways[cubes] += 1;
                    cubes += 4 * (w + h + l);
                    cubes += 8 * n;
                    n += 1;
                }
            }
        }
    }

    for i in 0..MAX {
        if ways[i] == 1000 {
            println!("{}", i);
            break;
        }
    }
}
