fn main() {
    println!("cargo:rerun-if-env-changed=PKG_CONFIG_PATH");
    let output = std::process::Command::new("pkg-config")
        .args(["--libs", "libvmaf"]).output().expect("pkg-config libvmaf");
    assert!(output.status.success(), "The selected libaom profile requires libvmaf");
    for argument in String::from_utf8(output.stdout).unwrap().split_whitespace() {
        if let Some(path) = argument.strip_prefix("-L") {
            println!("cargo:rustc-link-search=native={path}");
        } else if let Some(name) = argument.strip_prefix("-l") {
            println!("cargo:rustc-link-lib={name}");
        } else {
            panic!("Unmapped libvmaf linker argument: {argument}");
        }
    }
}
