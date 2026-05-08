// build.rs
fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();

    let kernel_o = format!("{}/kernel.o", out_dir);
    let kernel_a = format!("{}/libkernel.a", out_dir);

    let nvcc_output = std::process::Command::new("nvcc")
        .args(&[
            "-O2",
            "-c", "src/kernel.cu",
            "-o", &kernel_o,
            "--compiler-options", "-fPIC",
        ])
        .output()
        .expect("nvcc failed to spawn — is it on PATH?");

    println!("cargo:warning=nvcc stdout: {}", String::from_utf8_lossy(&nvcc_output.stdout));
    println!("cargo:warning=nvcc stderr: {}", String::from_utf8_lossy(&nvcc_output.stderr));
    println!("cargo:warning=nvcc status: {}", nvcc_output.status);

    if !nvcc_output.status.success() {
        panic!("nvcc compilation failed — see warnings above");
    }

    std::process::Command::new("ar")
        .args(&["crus", &kernel_a, &kernel_o])
        .status()
        .expect("ar failed");

    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=kernel");
    println!("cargo:rustc-link-search=native=/usr/local/cuda-10.2/targets/aarch64-linux/lib/");
    println!("cargo:rustc-link-lib=cudart");
    println!("cargo:rerun-if-changed=src/kernel.cu");
}
