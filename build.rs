extern crate cc;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
fn main() {}

#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
fn main() {
    cc::Build::new()
        .cpp(true)
        .flag("--std=c++20")
        .file("c_src/native_endian.cpp")
        .compile("native_endian_")
}
