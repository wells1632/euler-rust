// build.rs
use std::process::Command;
use std::env;
use std::path::PathBuf;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let cuda_obj = out_dir.join("monopoly_sim.o");
    let lib_path = out_dir.join("libmonopoly_sim.a");

    // Step 1: Find nvcc
    let nvcc = if std::path::Path::new("/usr/local/cuda/bin/nvcc").exists() {
        "/usr/local/cuda/bin/nvcc"
    } else {
        "nvcc"
    };
    eprintln!("cargo:warning=Using nvcc at: {}", nvcc);
    eprintln!("cargo:warning=OUT_DIR is: {}", out_dir.display());

    // Step 2: Compile CUDA to object file
    let nvcc_output = Command::new(nvcc)
        .args(&[
            "-O3",
            "-arch=sm_53",
            "--compiler-options", "-fPIC",
            "-c", "src/monopoly_sim.cu",
            "-o", cuda_obj.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to spawn nvcc — is CUDA installed at /usr/local/cuda?");

    if !nvcc_output.status.success() {
        panic!(
            "nvcc failed!\nstdout: {}\nstderr: {}",
            String::from_utf8_lossy(&nvcc_output.stdout),
            String::from_utf8_lossy(&nvcc_output.stderr)
        );
    }
    eprintln!("cargo:warning=nvcc succeeded, object at: {}", cuda_obj.display());

    // Step 3: Archive into static lib
    let ar_output = Command::new("ar")
        .args(&[
            "rcs",
            lib_path.to_str().unwrap(),
            cuda_obj.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to spawn ar");

    if !ar_output.status.success() {
        panic!(
            "ar failed!\nstdout: {}\nstderr: {}",
            String::from_utf8_lossy(&ar_output.stdout),
            String::from_utf8_lossy(&ar_output.stderr)
        );
    }
    eprintln!("cargo:warning=ar succeeded, lib at: {}", lib_path.display());

    // Step 4: Tell Cargo to link everything
    println!("cargo:rustc-link-search=native={}", out_dir.display());
    println!("cargo:rustc-link-lib=static=monopoly_sim");
    println!("cargo:rustc-link-search=native=/usr/local/cuda/lib64");
    println!("cargo:rustc-link-lib=dylib=cudart");
    println!("cargo:rustc-link-lib=dylib=curand");

    println!("cargo:rerun-if-changed=src/monopoly_sim.cu");
    println!("cargo:rerun-if-changed=build.rs");
}
