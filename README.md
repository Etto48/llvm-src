# llvm-src

llvm-src is a rust crate that provides a simple way to download and build the LLVM source code. It is intended to be used as a build dependency for other crates that need to build LLVM from source.

## Requirements

- `git`
- `cmake`
- `ninja`

On Ubuntu, you can install these with `sudo apt-get install git cmake ninja-build`.

## Usage

Add the following to your `Cargo.toml`:

```toml
[build-dependencies]
llvm-src = "0.1"
```

Then, in your `build.rs`:

```rust
fn main() {
    let artifacts = llvm_src::Build::default().build();
    artifacts.print_cargo_metadata();
}
```
