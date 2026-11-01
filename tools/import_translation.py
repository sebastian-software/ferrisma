#!/usr/bin/env python3
"""Import C2Rust output and apply only the documented mechanical corrections."""
import argparse
import hashlib
import json
from pathlib import Path
import re

ROOT = Path(__file__).resolve().parents[1]


def translate(source: Path, raw: str, codec_registry: bool = True) -> tuple[str, dict]:
    text = raw
    changes = {}

    # C2Rust hashes CFG labels using translation-unit identities. Canonicalize
    # only dispatcher assignments and matching arms, not source data constants.
    labels = {}
    for label in re.findall(r"\bcurrent_block(?:_\d+)? = (\d+);", text):
        if label not in labels:
            labels[label] = str(len(labels))
    pattern = r"(\bcurrent_block(?:_\d+)? = )(\d+)(?=;)|(^\s*)(\d+)(?= =>)"
    def relabel(match):
        prefix, value = (match[1], match[2]) if match[1] is not None else (match[3], match[4])
        return prefix + labels.get(value, value)
    text, count = re.subn(pattern, relabel, text, flags=re.M)
    if labels:
        changes["canonical-cfg-labels"] = len(labels)

    def replace(old, new, label):
        nonlocal text
        count = text.count(old)
        if count:
            text = text.replace(old, new)
            changes[label] = changes.get(label, 0) + count

    # Darwin AArch64 long double has the same representation as double. C2Rust
    # emits f128 even for dead branches in the SDK's type-dispatching isnan macro.
    replace("use ::f128;\n", "", "darwin-long-double")
    replace("f128::f128::new(v)", "v as core::ffi::c_double", "darwin-long-double")
    replace(
        "f128::f128::new(toneMappedPixelRGBA[c_3 as usize])",
        "toneMappedPixelRGBA[c_3 as usize] as core::ffi::c_double",
        "darwin-long-double",
    )
    replace("f128::f128", "core::ffi::c_double", "darwin-long-double")

    # Preserve C array-to-pointer decay without introducing Rust references to
    # static storage. Read the complete array type, including nested arrays.
    for name, array in re.findall(r"^\s*static mut (\w+): (\[.*?\]) =", text, re.M | re.S):
        element = array[1:-1].rsplit(";", 1)[0].strip()
        for method, address in [("as_mut_ptr", "addr_of_mut"), ("as_ptr", "addr_of")]:
            pattern = rf"\b{re.escape(name)}\s*\.{method}\(\)"
            text, count = re.subn(pattern, lambda _: f"core::ptr::{address}!({name}).cast::<{element}>()", text)
            if count:
                changes["static-raw-address"] = changes.get("static-raw-address", 0) + count

    if source.stem == "read":
        # The SDK inline helper is not an exported libavif implementation.
        replace(
            '#[no_mangle]\n#[inline]\n#[linkage = "external"]\npub unsafe extern "C" fn isprint',
            '#[inline]\nunsafe extern "C" fn isprint',
            "local-sdk-inline",
        )
    if source.stem in ("diag", "codec_dav1d"):
        # Rust's ABI-compatible VaList replaces the old C2Rust pointer-based
        # representation at both declarations of vsnprintf and the callback.
        replace(
            "pub type va_list = __builtin_va_list;",
            "pub type va_list<'a> = core::ffi::VaList<'a>;",
            "variadic-abi",
        )
        replace(
            "    let mut args_0: va_list = 0 as *mut core::ffi::c_char;\n    args_0 = args.clone();",
            "    let args_0 = args.clone();",
            "variadic-abi",
        )

    registry_path = ROOT / "translations/codec-registry.json"
    if codec_registry and source.stem == "avif" and registry_path.exists():
        registry = json.loads(registry_path.read_text())
        start = text.index("static mut availableCodecs:")
        end = text.index("\nstatic mut availableCodecsCount:", start)
        text = text[:start] + registry["table"] + text[end:]
        text = re.sub(r"\[AvailableCodec; \d+\]", registry["array_type"], text)
        for declaration in registry["declarations"]:
            name = re.search(r"fn (\w+)", declaration).group(1)
            if not re.search(r"\bfn " + name + r"\(", text):
                text = text.replace('extern "C" {', 'extern "C" {\n' + declaration, 1)
        changes["codec-feature-guards"] = len(registry["declarations"]) // 2

    # Retain the original leading copyright/license notice verbatim.
    original = source.read_text()
    notice = re.match(r"(?:(?:\s*//[^\n]*\n)|(?:\s*/\*.*?\*/\s*))*", original, re.S).group().rstrip()
    header = (
        notice + "\n\n// Mechanically translated from "
        + str(source.relative_to(ROOT)) + ".\n"
        + "// C ABI, ownership, pointer validity, and bounds follow the original headers.\n"
    )
    return header + text, changes


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--raw", type=Path, default=ROOT / "build/generated")
    parser.add_argument("--database", type=Path, default=ROOT / "build/translate/compile_commands.json")
    parser.add_argument("--check", action="store_true", help="Compare without writing translated files")
    parser.add_argument("--variant", choices=["default", "mini", "extended-pixi", "mini-extended-pixi"], default="default")
    args = parser.parse_args()
    entries = json.loads(args.database.read_text())
    manifest = []
    for entry in entries:
        source = Path(entry["file"])
        relative = source.relative_to(ROOT / "upstream/libavif")
        generated = args.raw / "src" / relative.with_suffix(".rs")
        target = ROOT / "src" / (relative.name if relative.parts[0] == "src" else "libyuv/" + relative.name)
        target = target.with_suffix(".rs")
        raw = generated.read_text()
        text, changes = translate(source, raw)
        if args.variant != "default" and target.read_text() != text:
            target = ROOT / "src/variants" / args.variant / target.relative_to(ROOT / "src")
        if args.check:
            if target.read_text() != text:
                raise SystemExit(f"Translation is not reproducible: {target}")
        else:
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_text(text)
        manifest.append({
            "source": str(source.relative_to(ROOT)),
            "rust": str(target.relative_to(ROOT)),
            "raw_sha256": hashlib.sha256(raw.encode()).hexdigest(),
            "rust_sha256": hashlib.sha256(text.encode()).hexdigest(),
            "corrections": changes,
        })
    record = {"translator": "c2rust 0.21.0", "llvm": "16.0.6", "variant": args.variant, "units": manifest}
    if not args.check:
        destination = ROOT / "TRANSLATION.json" if args.variant == "default" else ROOT / "translations" / f"{args.variant}.json"
        destination.parent.mkdir(parents=True, exist_ok=True)
        if destination.exists() and args.variant != "default":
            previous = json.loads(destination.read_text())["units"]
            current = {unit["rust"] for unit in manifest}
            for unit in previous:
                if unit["rust"].startswith(f"src/variants/{args.variant}/") and unit["rust"] not in current:
                    (ROOT / unit["rust"]).unlink(missing_ok=True)
        destination.write_text(json.dumps(record, indent=2) + "\n")
    print(f"{'Verified' if args.check else 'Imported'} {len(manifest)} mechanical Rust translation units")


if __name__ == "__main__":
    main()
