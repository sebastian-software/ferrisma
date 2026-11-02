fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("linux") {
        // Keep internal translated symbols bound to this implementation when
        // a differential caller also loads the original C library.
        println!("cargo:rustc-link-arg-cdylib=-Wl,-Bsymbolic");
    }
    println!("cargo:rerun-if-env-changed=PKG_CONFIG_PATH");
    let output = std::process::Command::new("pkg-config")
        .args(["--libs", "libvmaf"]).output().expect("pkg-config libvmaf");
    assert!(output.status.success(), "The selected libaom profile requires libvmaf");
    for argument in String::from_utf8(output.stdout).unwrap().split_whitespace() {
        if let Some(path) = argument.strip_prefix("-L") {
            println!("cargo:rustc-link-search=native={path}");
        } else if let Some(name) = argument.strip_prefix("-l") {
            println!("cargo:rustc-link-lib={name}");
        } else if argument == "-pthread" {
            // pkg-config uses the compiler-driver spelling on GNU Linux.
            println!("cargo:rustc-link-lib=pthread");
        } else {
            panic!("Unmapped libvmaf linker argument: {argument}");
        }
    }
    // The retained native VMAF archive contains libsvm's C++ implementation.
    // Its pkg-config file does not name the C++ standard library. C++ drivers
    // provide it implicitly; Rust and C consumers must retain it explicitly.
    match std::env::var("CARGO_CFG_TARGET_OS").as_deref() {
        Ok("linux") => println!("cargo:rustc-link-lib=stdc++"),
        Ok("macos") => println!("cargo:rustc-link-lib=c++"),
        _ => {}
    }
}
