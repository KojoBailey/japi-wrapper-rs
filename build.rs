fn main() {
    println!("cargo:rustc-link-search=native=japi/lib");
    println!("cargo:rustc-link-lib=dylib=JAPI");
}
