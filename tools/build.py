#!/usr/bin/env python3
"""Build the unchanged upstream consumers against C or the Rust archive."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import platform
import subprocess
import shlex
import sys

ROOT = Path(__file__).resolve().parents[1]
DEPENDENCIES = {
    "googletest": ("https://github.com/google/googletest.git", "52eb8108c5bdec04579160ae17225d66034bd723"),
    "libargparse": ("https://github.com/kmurray/libargparse.git", "ee74d1b53bd680748af14e737378de57e2a0a954"),
}


def run(command, **kwargs):
    environment = kwargs.setdefault("env", os.environ.copy())
    local = Path(environment.get("LIBAVIF_RS_NATIVE_DEPENDENCY_ROOT", ROOT)) / "build/deps/pkgconfig"
    if local.is_dir():
        existing = environment.get("PKG_CONFIG_PATH", "")
        environment["PKG_CONFIG_PATH"] = existing + (":" if existing else "") + str(local)
    subprocess.run([str(arg) for arg in command], cwd=ROOT, check=True, **kwargs)


def dependencies(fetch, dependency_root=ROOT):
    for name, (url, revision) in DEPENDENCIES.items():
        path = dependency_root / "build/deps" / name
        if not path.exists():
            if not fetch:
                raise SystemExit(f"Missing {name}; run with --fetch-deps to download the pinned test dependencies")
            path.parent.mkdir(parents=True, exist_ok=True)
            run(["git", "clone", "--no-checkout", url, path])
            run(["git", "-C", path, "checkout", "--detach", revision])
        actual = subprocess.check_output(["git", "-C", str(path), "rev-parse", "HEAD"], text=True).strip()
        if actual != revision:
            raise SystemExit(f"Unexpected {name} revision: {actual}; expected {revision}")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("implementation", choices=["baseline", "port"])
    parser.add_argument("--fetch-deps", action="store_true")
    parser.add_argument("--jobs", type=int, default=4)
    parser.add_argument("--aom-reference-build", type=Path,
                        help="Link the original C baseline to this complete pinned portable AOM reference")
    parser.add_argument("--native-dependency-root", type=Path, default=ROOT,
                        help="Read existing pinned test dependencies and optional engines from this checkout")
    parser.add_argument("--debug", action="store_true", help="Test unoptimized Rust with the same upstream Release configuration")
    parser.add_argument("--upstream-debug", action="store_true",
                        help="Use original Debug C preprocessing as well as unoptimized Rust; requires --debug")
    parser.add_argument("--root-references", type=Path,
                        help="Use the pinned original dav1d/SharpYUV archives from build_root_references.py")
    parser.add_argument("--root-profile", type=Path,
                        help="Build an isolated native generated profile for pre-admission original consumer checks")
    parser.add_argument("--production-profile", type=Path,
                        help="Record that the production package compiled exactly this generated native profile")
    parser.add_argument("--build-dir", type=Path, help="Isolated output directory inside this checkout's build directory")
    parser.add_argument("--asan", action="store_true", help="Instrument the library and original test consumers with AddressSanitizer")
    parser.add_argument("--instrumented-aom-reference", action="store_true",
                        help="Link the ASan C baseline to the complete instrumented original AOM closure")
    parser.add_argument("--compiler-default-diagnostic", action="store_true",
                        help="Build a separate original C baseline with compiler-default contraction")
    parser.add_argument("--mini", action="store_true", help="Enable upstream experimental reduced headers")
    parser.add_argument("--extended-pixi", action="store_true", help="Enable upstream experimental extended pixel information")
    for feature in ["libgav1", "rav1e", "svt", "libyuv", "avm"]:
        parser.add_argument("--" + feature, action="store_true")
    args = parser.parse_args()
    if args.upstream_debug and not args.debug:
        parser.error('--upstream-debug requires --debug')
    if args.root_profile and (args.implementation != 'port' or args.compiler_default_diagnostic or args.avm):
        parser.error('--root-profile requires port consumers with a generated AV1 codec closure; AVM is separate')
    if args.production_profile and (args.implementation != 'port' or args.root_profile or args.asan):
        parser.error('--production-profile records ordinary production port consumers')
    if sys.platform == 'linux' and args.implementation == 'port' and not args.root_profile:
        # The Linux package selects Debug preprocessing with debug assertions;
        # its consumers must use the identical original C configuration.
        if not args.production_profile:
            parser.error('Linux production consumers must name the generated profile they validate; '
                         'sanitized consumers use --root-profile')
        if args.debug != args.upstream_debug:
            parser.error('The Linux production Debug profile uses original Debug preprocessing; add --upstream-debug')
    if args.asan and args.debug:
        parser.error('The sanitizer configuration uses original Release preprocessing')
    if args.instrumented_aom_reference and (args.implementation != 'baseline' or not args.asan):
        parser.error('--instrumented-aom-reference requires the ASan original C baseline')
    root_references = args.root_references.resolve() if args.root_references else None
    if root_references:
        record = json.loads((root_references / 'reference-profile.json').read_text())
        expected_target = ('aarch64-apple-darwin' if sys.platform == 'darwin' and platform.machine() == 'arm64'
                           else 'x86_64-unknown-linux-gnu' if sys.platform == 'linux' and platform.machine() == 'x86_64'
                           else None)
        expected_profile = 'debug' if args.upstream_debug else 'release'
        if (record['target'] != expected_target or record['profile'] != expected_profile
                or record['original_dav1d_units'] != 51 or record['original_sharpyuv_units'] != 7):
            raise SystemExit('Original root references have a different target, C profile, or source closure')
        for name, item in record['references'].items():
            path = Path(item['path']).resolve()
            if not path.is_relative_to(root_references) or hashlib.sha256(path.read_bytes()).hexdigest() != item['sha256']:
                raise SystemExit(f'Original root reference identity differs: {name}')
        for name, expected in record['source_manifests_sha256'].items():
            if hashlib.sha256((ROOT / name).read_bytes()).hexdigest() != expected:
                raise SystemExit(f'Original root reference source changed: {name}')
    if args.aom_reference_build:
        if args.implementation != 'baseline' or args.asan:
            parser.error('--aom-reference-build requires a nonsanitized original C baseline; ASan needs a separately instrumented AOM reference')
        args.aom_reference_build = args.aom_reference_build.resolve()
        profile = 'debug' if args.debug else 'release'
        reference = json.loads((args.aom_reference_build / 'reference-profile.json').read_text())
        reference_platform = 'macos' if sys.platform == 'darwin' else 'linux'
        translation = json.loads((ROOT / f'crates/aom/TRANSLATION-{reference_platform}-{profile}.json').read_text())
        if (reference['profile'] != profile or reference['library_translation_units'] != 168
                or '-ffp-contract=off' not in shlex.split(reference['c_flags'])
                or not (args.aom_reference_build / 'libaom.a').is_file()):
            raise SystemExit('The C baseline requires the complete noncontracting AOM reference in the matching profile')
        from build_aom import library_entries
        if len(library_entries(args.aom_reference_build)) != 168:
            raise SystemExit('The original AOM reference archive does not have the declared complete source closure')
        for name, expected in translation['generated_reference_headers'].items():
            if hashlib.sha256((args.aom_reference_build / name).read_bytes()).hexdigest() != expected:
                raise SystemExit(f'The original AOM reference header differs: {name}')
    dependency_root = args.native_dependency_root.resolve()
    os.environ['LIBAVIF_RS_NATIVE_DEPENDENCY_ROOT'] = str(dependency_root)
    if args.compiler_default_diagnostic and args.implementation != 'baseline':
        parser.error('Compiler-default diagnostics apply only to the original C baseline')
    run(["python3", ROOT / "tools/verify_upstream.py"])
    dependencies(args.fetch_deps, dependency_root)
    features = [name for name, enabled in [("mini", args.mini), ("extended-pixi", args.extended_pixi),
                ("libgav1", args.libgav1), ("rav1e", args.rav1e), ("svt", args.svt), ("libyuv", args.libyuv), ("avm", args.avm)] if enabled]
    suffix = (("-" + "-".join(features) if features else "")
              + ("-aom-reference" if args.aom_reference_build else "")
              + ("-debug" if args.debug else "") + ("-asan-llvm16" if args.asan else ""))
    if args.compiler_default_diagnostic:
        suffix += '-compiler-default'
    numeric_flags = '' if args.compiler_default_diagnostic else '-ffp-contract=off'
    build = ROOT / "build" / (args.implementation + suffix)
    if args.build_dir:
        build = args.build_dir.resolve()
        if not build.is_relative_to((ROOT / 'build').resolve()):
            parser.error('--build-dir must stay inside this checkout\'s build directory')
    command = [
        "cmake", "-S", ROOT / "upstream/libavif", "-B", build, "-G", "Ninja",
        "-DCMAKE_BUILD_TYPE=" + ('Debug' if args.upstream_debug else 'Release'), "-DCMAKE_EXPORT_COMPILE_COMMANDS=ON",
        "-DCMAKE_C_FLAGS=" + numeric_flags,
        "-DBUILD_SHARED_LIBS=OFF", "-DAVIF_BUILD_TESTS=ON", "-DAVIF_BUILD_APPS=ON",
        "-DAVIF_CODEC_AOM=SYSTEM", "-DAVIF_CODEC_DAV1D=SYSTEM", f"-DAVIF_LIBYUV={'SYSTEM' if args.libyuv else 'OFF'}",
        "-DAVIF_LIBSHARPYUV=SYSTEM", "-DAVIF_LIBXML2=SYSTEM", "-DAVIF_GTEST=LOCAL",
        f"-DFETCHCONTENT_SOURCE_DIR_GOOGLETEST={dependency_root}/build/deps/googletest",
        f"-DFETCHCONTENT_SOURCE_DIR_LIBARGPARSE={dependency_root}/build/deps/libargparse",
        f"-DLIBAVIF_RS_NATIVE_DEPENDENCY_ROOT={dependency_root}",
        f"-DAVIF_ENABLE_EXPERIMENTAL_MINI={'ON' if args.mini else 'OFF'}",
        f"-DAVIF_ENABLE_EXPERIMENTAL_EXTENDED_PIXI={'ON' if args.extended_pixi else 'OFF'}",
        f"-DAVIF_CODEC_LIBGAV1={'SYSTEM' if args.libgav1 else 'OFF'}",
        f"-DAVIF_CODEC_RAV1E={'SYSTEM' if args.rav1e else 'OFF'}",
        f"-DAVIF_CODEC_SVT={'SYSTEM' if args.svt else 'OFF'}",
        f"-DAVIF_CODEC_AVM={'SYSTEM' if args.avm else 'OFF'}",
        f"-DCMAKE_PROJECT_libavif_INCLUDE={ROOT}/tools/local-dependencies.cmake",
    ]
    if root_references:
        command += [f'-DDAV1D_LIBRARY={root_references}/dav1d/src/libdav1d.a',
                    f'-DDAV1D_INCLUDE_DIR={ROOT}/upstream/dav1d/include;{root_references}/dav1d/include',
                    f'-DLIBSHARPYUV_LIBRARY={root_references}/sharpyuv/libsharpyuv.a',
                    f'-DLIBSHARPYUV_INCLUDE_DIR={ROOT}/upstream/libwebp']
    if args.implementation == 'port':
        command += ['-DLIBAVIF_RS_AOM_IN_RUST=ON', '-DCMAKE_DISABLE_FIND_PACKAGE_aom=TRUE']
    if args.aom_reference_build:
        command += [f'-DLIBAVIF_RS_AOM_REFERENCE_BUILD={args.aom_reference_build}',
                    '-DCMAKE_DISABLE_FIND_PACKAGE_aom=TRUE']
    if args.asan:
        from aom_sanitizer import llvm_prefix, native_runtime
        prefix = llvm_prefix()
        triple, runtime_file, runtime_link = native_runtime()
        runtime_path = runtime_file.parent
        compilers = {name: prefix / "bin" / name for name in ["clang", "clang++"]}
        if sys.platform == "darwin":
            wrappers = ROOT / "build/toolchain/wrappers"
            wrappers.mkdir(parents=True, exist_ok=True)
            for compiler in compilers:
                wrapper = wrappers / compiler
                wrapper.write_text("#!/bin/sh\nexport DYLD_LIBRARY_PATH=" + shlex.quote(str(prefix / "lib") + ":/opt/homebrew/lib") +
                                   "\nexec " + shlex.quote(str(prefix / "bin" / compiler)) + ' "$@"\n')
                wrapper.chmod(0o755)
                compilers[compiler] = wrapper
        command += ["-DCMAKE_C_FLAGS=" + numeric_flags + " -fsanitize=address -fno-omit-frame-pointer",
                    "-DCMAKE_CXX_FLAGS=-fsanitize=address -fno-omit-frame-pointer",
                    f"-DCMAKE_C_COMPILER={compilers['clang']}", f"-DCMAKE_CXX_COMPILER={compilers['clang++']}",
                    "-DCMAKE_EXE_LINKER_FLAGS=" + shlex.join(runtime_link)]
        if args.instrumented_aom_reference:
            from aom_sanitizer import instrumented_reference
            reference, _, _ = instrumented_reference()
            command += [f'-DLIBAVIF_RS_AOM_REFERENCE_BUILD={reference}', '-DCMAKE_DISABLE_FIND_PACKAGE_aom=TRUE']
    if args.root_profile:
        from check_root_profile import validate_profile, profile_work, digest
        target_triple = ('aarch64-apple-darwin' if sys.platform == 'darwin' and platform.machine() == 'arm64'
                         else 'x86_64-unknown-linux-gnu' if sys.platform == 'linux' and platform.machine() == 'x86_64'
                         else None)
        generated = validate_profile(args.root_profile.resolve(), target_triple)
        sanitizer = 'address' if args.asan else None
        variant = '-'.join(features) or 'default'
        if (generated['preprocessing_profile'] != ('debug' if args.debug else 'release')
                or args.debug != args.upstream_debug or generated['variant'] != variant):
            parser.error('The isolated Rust root profile must match original C preprocessing and container features')
        run(['python3', ROOT / 'tools/check_root_profile.py', args.root_profile.resolve(), '--build',
             *(['--sanitizer', sanitizer] if sanitizer else [])])
        proof = json.loads((profile_work(generated, sanitizer) / 'summary.json').read_text())
        item = proof['artifacts']['libavif.a']
        archive = Path(item['path'])
        if (proof['translation_manifest_sha256'] != digest(args.root_profile.resolve())
                or proof.get('sanitizer') != sanitizer or digest(archive) != item['sha256']):
            raise SystemExit('The isolated root archive does not match its actual generated build proof')
        command += [f'-DLIBAVIF_RS_ARCHIVE={archive}']
    elif args.implementation == "port":
        cargo = ["cargo", "build", "--locked"]
        if not args.debug:
            cargo.append("--release")
        if features:
            cargo += ["--features", ",".join(features)]
        environment = os.environ.copy()
        # Keep the immutable archives used by running standalone shards.
        from root_production import cargo_target
        target = cargo_target(features)
        environment['CARGO_TARGET_DIR'] = str(target)
        if args.asan:
            target = target / "asan"
            environment["CARGO_TARGET_DIR"] = str(target)
            flags = environment.get("CARGO_ENCODED_RUSTFLAGS", "").split("\x1f") if environment.get("CARGO_ENCODED_RUSTFLAGS") else shlex.split(environment.get("RUSTFLAGS", ""))
            flags += ["-Zsanitizer=address", "-Zexternal-clangrt", "-Cforce-frame-pointers=yes",
                      "--cfg", "libavif_rs_asan"]
            if sys.platform == "darwin":
                flags += ["-Lnative=" + str(runtime_path), "-ldylib=rustc-nightly_rt.asan",
                          "-Clink-arg=-Wl,-rpath," + str(runtime_path)]
            environment["CARGO_ENCODED_RUSTFLAGS"] = "\x1f".join(flags)
            environment.pop("RUSTFLAGS", None)
            cargo += ["--target", triple]
            target = target / triple
        run(cargo, env=environment)
        archive = target / ("debug" if args.debug else "release") / "libavif.a"
        if args.production_profile:
            from root_production import native_target, record_build
            proof = record_build(target, "debug" if args.debug else "release", features, cargo,
                                 args.production_profile, native_target())
            build.mkdir(parents=True, exist_ok=True)
            (build / "production-package.json").write_text(json.dumps(proof, indent=2) + "\n")
            print(f"Production package compiled the {proof['root_profile']['units']}-unit {proof['variant']} profile")
        command += [f"-DLIBAVIF_RS_ARCHIVE={archive}"]
    run(command)
    run(["cmake", "--build", build, "-j", args.jobs])
    if args.implementation == "port":
        entries = json.loads((build / "compile_commands.json").read_text())
        library = [entry["file"] for entry in entries if "avif_obj.dir" in entry["command"]]
        if len(library) != 1 or not library[0].endswith("rust_archive_anchor.c"):
            raise SystemExit(f"Unexpected C library implementations linked into Rust tests: {library}")
        print("Confirmed: upstream tests link Rust; no C libavif implementation is compiled")


if __name__ == "__main__":
    main()
