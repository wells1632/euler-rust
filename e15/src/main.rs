fn main() {
    println!("{}", count_lattice_paths(20,20));
}

fn count_lattice_paths(i: i64, j: i64) -> i64 {
    if i <= 1 || j <= 1 {
        return i+j;
    }
    count_lattice_paths(i-1, j) + count_lattice_paths(i, j-1)
}
