#!/usr/bin/env python3
"""Reproduce portable SharpYUV with explicit platform and assertion mappings."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import shlex
import subprocess

from import_translation import translate

ROOT = Path(__file__).resolve().parents[1]
CRATE = ROOT / "crates/sharpyuv"


def corrected(source, raw):
    text, changes = translate(source, raw, codec_registry=False)
    # Debug assertions are captured from the original Debug preprocessing. Their
    # predicates have no side effects; the platform helper follows NDEBUG in
    # release and uses the target C runtime's assertion entry point in debug.
    text, count = re.subn(r"(?<!fn )\b__assert_rtn\(", "crate::platform::assertion(", text)
    if count:
        changes["target-runtime-assertion"] = count
    text = re.sub(r"    fn __assert_rtn\([\s\S]*?\) -> !;\n", "", text)
    if source.stem == "sharpyuv_csp":
        # The original headers declare one public matrix type shared by both
        # units. Preserve that identity for Rust callers as well as its C layout.
        text, count = re.subn(r"#\[derive\(Copy, Clone\)\]\n#\[repr\(C\)\]\npub struct SharpYuvConversionMatrix \{[\s\S]*?\n\}",
                              "pub use crate::sharpyuv::SharpYuvConversionMatrix;", text)
        if count != 1:
            raise SystemExit("Unexpected shared matrix declaration")
        changes["shared-header-type"] = count
    if source.stem == "sharpyuv":
        text = text.replace("    fn malloc(", '    #[cfg_attr(sharpyuv_test_allocator, link_name = "sharp_test_malloc")]\n    fn malloc(')
        text = text.replace("    fn free(", '    #[cfg_attr(sharpyuv_test_allocator, link_name = "sharp_test_free")]\n    fn free(')
        changes["test-allocator-symbols"] = 2
        # This unit only stores/compares callbacks. The C initializer uses an
        # object address as a sentinel, which is not a valid Rust fn value.
        # Keep its pointer bits and all original guards; DSP retains callable
        # Option<fn> storage and is only assigned actual callbacks or NULL.
        original = 'pub type VP8CPUInfo = Option<unsafe extern "C" fn(CPUFeature) -> core::ffi::c_int>;'
        if text.count(original) != 1:
            raise SystemExit("Unexpected CPU callback representation")
        text = text.replace(original, "pub type VP8CPUInfo = *const core::ffi::c_void;")
        changes["opaque-callback-sentinel"] = 1
        text = text.replace("pub type pthread_mutex_t = __darwin_pthread_mutex_t;",
                            "pub type pthread_mutex_t = libc::pthread_mutex_t;")
        text, count = re.subn(r"static mut sharpyuv_lock: pthread_mutex_t = \{[\s\S]*?\n    \};",
                              "static mut sharpyuv_lock: pthread_mutex_t = libc::PTHREAD_MUTEX_INITIALIZER;", text)
        if count != 1:
            raise SystemExit("Unexpected native mutex initializer")
        changes["target-libc-mutex"] = 1
    # C address-taking does not create a Rust borrow, especially before a
    # pthread lock is acquired. Include extern declarations and local statics.
    names = set(re.findall(r"\bstatic mut (\w+):", text))
    count = 0
    for name in sorted(names, key=len, reverse=True):
        text, n = re.subn(r"\b" + name + r"\.(is_none|is_some)\(\)",
                          r"core::ptr::read(core::ptr::addr_of!(" + name + r")).\1()", text)
        count += n
        text, n = re.subn(r"&mut " + name + r"\b", "core::ptr::addr_of_mut!(" + name + ")", text)
        count += n
        text, n = re.subn(r"&" + name + r"\b", "core::ptr::addr_of!(" + name + ")", text)
        count += n
    if count:
        changes["raw-static-address"] = count
    # C's empty conditional still evaluates the callback pointer comparison.
    # Retain that evaluation while making the intentionally unused result explicit.
    text, count = re.subn(r"(?m)^(    )(core::ptr::read\(core::ptr::addr_of!\(SharpYuvGetCPUInfo\)\)\.is_some\(\);)$",
                          r"\1let _ = \2", text)
    if count:
        changes["discard-empty-conditional-result"] = count
    return text, changes


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true")
    parser.add_argument("--import-only", action="store_true", help="Reuse the recorded local C2Rust output")
    parser.add_argument("--clang-include", type=Path,
                        default=ROOT / "build/toolchain/llvm@16/16.0.6_1/lib/clang/16/include")
    args = parser.parse_args()
    subprocess.run(["python3", ROOT / "tools/verify_upstream.py"], check=True)
    entries = [entry for entry in json.loads((ROOT / "build/sharpyuv-reference-debug/compile_commands.json").read_text())
               if "sharpyuv.dir" in entry["command"]]
    if len(entries) != 7:
        raise SystemExit("Expected five scalar sources and two upstream disabled-SIMD translation units")
    work = ROOT / "build/sharpyuv-translate"
    work.mkdir(exist_ok=True)
    for entry in entries:
        entry["command"] += " -isystem " + shlex.quote(str(args.clang_include.resolve()))
    database = work / "compile_commands.json"
    database.write_text(json.dumps(entries, indent=2) + "\n")
    raw_root = ROOT / "build/sharpyuv-generated"
    if not args.import_only:
        environment = os.environ.copy()
        if environment.get("C2RUST_LIBRARY_PATH"):
            environment["DYLD_LIBRARY_PATH"] = environment["C2RUST_LIBRARY_PATH"]
        else:
            environment["DYLD_LIBRARY_PATH"] = str(args.clang_include.parents[2]) + ":/opt/homebrew/lib"
        result = subprocess.run(["c2rust-transpile", database, "--emit-build-files", "--fail-on-error",
                                 "--output-dir", raw_root], env=environment,
                                text=True, stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
        (ROOT / "build/sharpyuv-transpile.log").write_text(result.stdout)
        if result.returncode or "error:" in result.stdout or "error generated" in result.stdout:
            raise SystemExit("C2Rust failed; inspect build/sharpyuv-transpile.log")
    units = []
    for entry in entries:
        source = Path(entry["file"])
        raw = (raw_root / "src" / source.with_suffix(".rs").name).read_text()
        text, changes = corrected(source, raw)
        target = CRATE / "src" / source.with_suffix(".rs").name
        if args.check:
            if target.read_text() != text:
                raise SystemExit(f"Translation differs: {target}")
        else:
            target.write_text(text)
        units.append({"source": str(source.relative_to(ROOT)), "rust": str(target.relative_to(ROOT)),
                      "raw_sha256": hashlib.sha256(raw.encode()).hexdigest(),
                      "rust_sha256": hashlib.sha256(text.encode()).hexdigest(), "corrections": changes})
    record = {"translator": "c2rust 0.21.0", "llvm": "16.0.6", "reference": "Debug, WEBP_ENABLE_SIMD=OFF, WEBP_USE_THREAD=ON",
              "generation_target": "aarch64-apple-darwin", "units": units}
    manifest = CRATE / "TRANSLATION.json"
    encoded = json.dumps(record, indent=2) + "\n"
    if args.check:
        if manifest.read_text() != encoded:
            raise SystemExit("Translation manifest differs")
    else:
        manifest.write_text(encoded)
    print(f"{'Verified' if args.check else 'Imported'} {len(units)} SharpYUV units")


if __name__ == "__main__":
    main()
