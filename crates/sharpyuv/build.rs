fn main() {
    println!("cargo:rustc-check-cfg=cfg(sharpyuv_test_allocator)");
}
