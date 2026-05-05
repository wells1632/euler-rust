// build.rs  (sits at the crate root, next to Cargo.toml)
fn main() {
    println!("cargo:rustc-link-lib=cudart");
    println!("cargo:rustc-link-search=/usr/local/cuda/lib64");
    cc::Build::new()
        .cuda(true)
        .file("src/prime_search.cu")
        .compile("prime_search");
}
