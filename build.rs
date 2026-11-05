use std::process::Command;

fn main() {
    println!("cargo:rustc-check-cfg=cfg(libavif_rs_asan)");
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("linux") {
        return native_linux();
    }
    let mut packages = vec![];
    for (feature, package) in [("RAV1E", "rav1e"), ("SVT", "SvtAv1Enc")] {
        if std::env::var_os(format!("CARGO_FEATURE_{feature}")).is_some() {
            packages.push(package);
        }
    }
    let root = std::path::PathBuf::from(
        std::env::var_os("LIBAVIF_RS_NATIVE_DEPENDENCY_ROOT")
            .unwrap_or_else(|| std::env::var_os("CARGO_MANIFEST_DIR").unwrap()),
    );
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
    println!("cargo:rerun-if-env-changed=LIBAVIF_RS_NATIVE_DEPENDENCY_ROOT");
    println!("cargo:rerun-if-changed=build.rs");
}

/// Link GNU Linux exactly as its validated native root profiles (ADR 0053 and
/// ADR 0054). The pinned engine archives stay unbundled, so original consumers
/// link the same imported engine targets; shared engines resolve through the
/// Rust shared library's run path. Only the verified complete engine set is
/// linked. The crate itself rejects every unverified feature set.
fn native_linux() {
    // Keep translated internal symbols bound to this implementation when a
    // differential caller also loads the original C library.
    println!("cargo:rustc-link-arg-cdylib=-Wl,-Bsymbolic");
    let enabled = |feature: &str| std::env::var_os(format!("CARGO_FEATURE_{feature}")).is_some();
    let engines = ["LIBGAV1", "RAV1E", "SVT", "LIBYUV"];
    if engines.iter().all(|engine| enabled(engine)) && !enabled("AVM") {
        let root = std::path::PathBuf::from(
            std::env::var_os("LIBAVIF_RS_NATIVE_DEPENDENCY_ROOT")
                .unwrap_or_else(|| std::env::var_os("CARGO_MANIFEST_DIR").unwrap()),
        );
        let mut runtime_paths: Vec<String> = vec![];
        for engine in engines {
            if let Some((library, local)) = match engine {
                "LIBGAV1" => Some(("gav1", "libgav1-build")),
                "LIBYUV" => Some(("yuv", "libyuv-build")),
                _ => None,
            } {
                let path = root.join("build/deps").join(local);
                assert!(
                    path.join(format!("lib{library}.a")).is_file(),
                    "Build the pinned {} dependency first: {}",
                    engine.to_lowercase(),
                    path.display()
                );
                println!("cargo:rustc-link-search=native={}", path.display());
                println!("cargo:rustc-link-lib=static:-bundle={library}");
                println!("cargo:rustc-link-lib=dylib=stdc++");
                continue;
            }
            let package = if engine == "RAV1E" { "rav1e" } else { "SvtAv1Enc" };
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
            assert!(output.status.success(), "pkg-config could not find {package}");
            for flag in String::from_utf8(output.stdout).unwrap().split_whitespace() {
                if let Some(path) = flag.strip_prefix("-L") {
                    println!("cargo:rustc-link-search=native={path}");
                    if !runtime_paths.iter().any(|known| known == path) {
                        runtime_paths.push(path.to_owned());
                    }
                } else if let Some(library) = flag.strip_prefix("-l") {
                    println!("cargo:rustc-link-lib={library}");
                } else {
                    panic!("Unexpected {package} link flag: {flag}");
                }
            }
        }
        for path in runtime_paths {
            println!("cargo:rustc-link-arg-cdylib=-Wl,-rpath,{path}");
        }
    }
    println!("cargo:rerun-if-env-changed=PKG_CONFIG_PATH");
    println!("cargo:rerun-if-env-changed=LIBAVIF_RS_NATIVE_DEPENDENCY_ROOT");
    println!("cargo:rerun-if-changed=build.rs");
}
