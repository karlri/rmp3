fn main() {
    println!("cargo:rerun-if-changed=ffi/minimp3.c");
    let mut build = cc::Build::new();

    build.include("ffi/minimp3");
    build.flag("-fPIC");
    build.flag("-O2");
    // xtensa architecture:
    // need to pass -mlongcalls to linker, or link errors like
    // dangerous relocation: call8: call target out of range
    // we can only pass flags to linker with CFLAGS env var.
    // https://gcc.gnu.org/onlinedocs/gcc/Xtensa-Options.html search -mlongcalls
    if std::env::var("CARGO_CFG_TARGET_ARCH").unwrap() == "xtensa" {
        let mut cflags = std::env::var("CFLAGS").unwrap_or(String::new());
        if !cflags.contains("-mlongcalls") {
            if !cflags.ends_with(' ') {
                cflags.push(' ');
            }
            cflags.push_str("-mlongcalls");
            std::env::set_var("CFLAGS", cflags);
        }
    }

    if cfg!(feature = "float") {
        build.define("MINIMP3_FLOAT_OUTPUT", None);
    }
    if cfg!(not(feature = "simd")) {
        build.define("MINIMP3_NO_SIMD", None);
    }
    if cfg!(not(feature = "mp1-mp2")) {
        build.define("MINIMP3_ONLY_MP3", None);
    }

    build
        .define("MINIMP3_IMPLEMENTATION", None)
        .file("ffi/minimp3.c")
        .compile("minimp3");
}
