use cc;

fn main() {
    cc::Build::new()
        .compiler("mos-kim1-clang")
        .include("/usr/local/mos-platform/common/include/")
        .include("/usr/local/mos-platform/common/kim1/include/")
        .file("kim.c")
        .compile("kim");
    println!("cargo:rerun-if-changed=kim.c");
    println!("cargo:rustc-link-lib=static=kim");
    println!("cargo:rerun-if-changed=build.rs");
}
