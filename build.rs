fn main() {
    // Get absolute path
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let lib_path = std::path::Path::new(&manifest_dir).join("japi").join("lib");

    // Debug: print what we're looking for
    println!("cargo:warning=Manifest dir: {}", manifest_dir);
    println!("cargo:warning=Lib path: {}", lib_path.display());

    // Check if the file exists
    let dll_a = lib_path.join("libJAPI.dll.a");
    println!("cargo:warning=Looking for: {}", dll_a.display());
    println!("cargo:warning=File exists: {}", dll_a.exists());

    // Add the search path
    println!("cargo:rustc-link-search=native={}", lib_path.display());
    println!("cargo:rustc-link-lib=dylib=JAPI");
}
