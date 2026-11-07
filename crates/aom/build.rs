fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("linux") {
        // Keep internal translated symbols bound to this implementation when
        // a differential caller also loads the original C library.
        println!("cargo:rustc-link-arg-cdylib=-Wl,-Bsymbolic");
    }
}
