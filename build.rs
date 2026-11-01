use std::process::Command;

fn main() {
    println!("cargo:rustc-check-cfg=cfg(libavif_rs_asan)");
    let mut packages = vec!["aom", "dav1d"];
    for (feature, package) in [("RAV1E", "rav1e"), ("SVT", "SvtAv1Enc")] {
        if std::env::var_os(format!("CARGO_FEATURE_{feature}")).is_some() {
            packages.push(package);
        }
    }
    let root = std::path::PathBuf::from(std::env::var_os("CARGO_MANIFEST_DIR").unwrap());
    if std::env::var_os("CARGO_FEATURE_AVM").is_some() {
        let path = root.join("build/deps/avm-build");
        if path.join("libavm.dylib").exists() {
            println!("cargo:rustc-link-search=native={}", path.display());
            println!("cargo:rustc-link-lib=dylib=avm");
            println!("cargo:rustc-link-arg=-Wl,-rpath,{}", path.display());
        } else {
            packages.push("avm");
        }
    }
    for (feature, library, local) in [
        ("LIBGAV1", "gav1", "libgav1-build"),
        ("LIBYUV", "yuv", "libyuv-build"),
    ] {
        if std::env::var_os(format!("CARGO_FEATURE_{feature}")).is_some() {
            let path = root.join("build/deps").join(local);
            if path.join(format!("lib{library}.a")).exists() {
                println!("cargo:rustc-link-search=native={}", path.display());
                println!("cargo:rustc-link-lib=static={library}");
                println!("cargo:rustc-link-lib=c++");
            } else {
                packages.push(if feature == "LIBGAV1" {
                    "libgav1"
                } else {
                    "libyuv"
                });
            }
        }
    }
    for package in packages {
        let mut command = Command::new("pkg-config");
        command.args(["--libs", package]);
        let local = root.join("build/deps/pkgconfig");
        if local.is_dir() {
            let mut paths: Vec<_> = std::env::var_os("PKG_CONFIG_PATH")
                .map(|value| std::env::split_paths(&value).collect())
                .unwrap_or_default();
            paths.push(local);
            command.env("PKG_CONFIG_PATH", std::env::join_paths(paths).unwrap());
        }
        let output = command
            .output()
            .expect("pkg-config is required for the upstream codec dependencies");
        assert!(
            output.status.success(),
            "pkg-config could not find {package}"
        );
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
