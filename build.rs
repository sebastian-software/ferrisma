use std::process::Command;

fn main() {
    for package in ["aom", "dav1d", "libsharpyuv"] {
        let output = Command::new("pkg-config")
            .args(["--libs", package])
            .output()
            .expect("pkg-config is required for the upstream codec dependencies");
        assert!(output.status.success(), "pkg-config could not find {package}");
        for flag in String::from_utf8(output.stdout).unwrap().split_whitespace() {
            if let Some(path) = flag.strip_prefix("-L") {
                println!("cargo:rustc-link-search=native={path}");
            } else if let Some(library) = flag.strip_prefix("-l") {
                println!("cargo:rustc-link-lib={library}");
            } else {
                println!("cargo:rustc-link-arg={flag}");
            }
        }
    }
    println!("cargo:rerun-if-env-changed=PKG_CONFIG_PATH");
    println!("cargo:rerun-if-changed=build.rs");
}
