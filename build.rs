fn main() {
    cc::Build::new()
        .compiler("mos-clang")
        .include("/usr/local/mos-platform/common/include/")
        .file("kim.c")
        .compile("kim");
    println!("cargo:rerun-if-changed=kim.c");
    println!("cargo:rustc-link-lib=static=kim");
    println!("cargo:rustc-link-search=/usr/local/mos-platform/common/lib");
    println!("cargo:rerun-if-changed=build.rs");
}
