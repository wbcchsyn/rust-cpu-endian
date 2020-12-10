extern crate cc;

fn main() {
    cc::Build::new()
        .cpp(true)
        .flag("--std=c++20")
        .file("c_src/native_endian.cpp")
        .compile("native_endian_")
}
