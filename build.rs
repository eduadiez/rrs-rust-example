// build.rs

fn main() {
    // Use the linker script.
    println!("cargo:rustc-link-arg=-Tsrc/link.ld");
}
