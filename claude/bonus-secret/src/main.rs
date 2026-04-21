use std::fs::File;
use std::io::BufWriter;

const W: usize = 10;
const H: usize = 6;
const N: usize = W * H;
const MOD: u64 = 7;

type Matrix = [[u64; N]; N];

fn mat_mul(a: &Matrix, b: &Matrix) -> Matrix {
    let mut c = [[0u64; N]; N];
    for i in 0..N {
        for k in 0..N {
            if a[i][k] == 0 { continue; }
            for j in 0..N {
                c[i][j] = (c[i][j] + a[i][k] * b[k][j]) % MOD;
            }
        }
    }
    c
}

fn mat_pow(mut base: Matrix, mut exp: u64) -> Matrix {
    let mut result = identity();
    while exp > 0 {
        if exp & 1 == 1 {
            result = mat_mul(&result, &base);
        }
        base = mat_mul(&base, &base);
        exp >>= 1;
    }
    result
}

fn identity() -> Matrix {
    let mut m = [[0u64; N]; N];
    for i in 0..N { m[i][i] = 1; }
    m
}

fn idx(col: usize, row: usize) -> usize {
    row * W + col
}

fn build_transition() -> Matrix {
    let mut m = [[0u64; N]; N];
    for r in 0..H {
        for c in 0..W {
            let i = idx(c, r);
            let neighbors = [
                idx((c + 1) % W, r),
                idx((c + W - 1) % W, r),
                idx(c, (r + 1) % H),
                idx(c, (r + H - 1) % H),
            ];
            for &j in &neighbors {
                m[i][j] = (m[i][j] + 1) % MOD;
            }
        }
    }
    m
}

fn main() {
    let mut grid = [0u64; N];

    // (row, col) 1-indexed, values: a=1,b=2,c=3,d=4,e=5, A=1,B=2,C=3,D=4,E=5
    let assignments: &[(usize, usize, u64)] = &[
        (1, 10, 4), // d
        (2,  1, 2), // B
        (3,  1, 1), // A
        (3,  2, 5), // E
        (2, 10, 3), // C
        (4,  1, 4), // D
        (5, 10, 2), // b
        (6,  1, 5), // e
        (6,  9, 3), // c
        (6, 10, 1), // a
    ];

    for &(r, c, v) in assignments {
        grid[idx(c - 1, r - 1)] = v % MOD;
    }

    let trans = build_transition();
    let trans_pow = mat_pow(trans, 1_000_000_000_000u64);

    let mut result = [0u64; N];
    for i in 0..N {
        for j in 0..N {
            result[i] = (result[i] + trans_pow[i][j] * grid[j]) % MOD;
        }
    }

    // Scale 0–6 to 0–255 and write PNG
    let pixel_data: Vec<u8> = result
        .iter()
        .map(|&v| ((v * 255) / 6) as u8)
        .collect();

    let file = File::create("output.png").expect("Could not create file");
    let ref mut w = BufWriter::new(file);
    let mut encoder = png::Encoder::new(w, W as u32, H as u32);
    encoder.set_color(png::ColorType::Grayscale);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().expect("Could not write header");
    writer.write_image_data(&pixel_data).expect("Could not write image data");

    println!("Written to output.png");
}