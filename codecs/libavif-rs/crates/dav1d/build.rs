fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("linux") {
        // Keep translated internal C symbols bound to this implementation.
        // The differential DSP harness loads the original C library separately.
        println!("cargo:rustc-link-arg-cdylib=-Wl,-Bsymbolic");
    }
}
