use std::env;

fn main() {
    let linker_file = env::var("LINKER_FILE").unwrap_or_else(|_| "linker.ld".to_string());

    println!("cargo:rerun-if-changed={}", linker_file);
    println!("cargo:rerun-if-changed=build.rs");
}
