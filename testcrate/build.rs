fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let artifacts = llvm_src::Build::default().build();
    artifacts.print_cargo_metadata();
}