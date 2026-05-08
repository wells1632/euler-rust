fn main() {
    println!("cargo:rustc-link-lib=cuda_kernel");
    println!("cargo:rustc-link-search=native=.");
    println!("cargo:rustc-link-search=native=/usr/local/cuda/lib64");
    println!("cargo:rustc-link-lib=cudart");
}
