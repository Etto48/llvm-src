use std::path::PathBuf;

use llvm_src::Build;

pub fn main() {
    let mut build = Build::new();
    std::env::set_var("NUM_JOBS", "12");
    build
        .host("x86_64-pc-windows-msvc")
        .target("x86_64-pc-windows-msvc")
        .profile("Release")
        .out_dir(&PathBuf::from("target"));
    let artifacts = build.build();
    artifacts.print_cargo_metadata();
}