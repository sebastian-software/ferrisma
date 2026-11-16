#!/usr/bin/env python3
"""Import optional AV1 adapters and translate the C codec registry's feature guards.

The registry (`translations/codec-registry.json`) is the original codec table of
`avif.c` translated with every optional guard (`tools/translate_registry.py`),
each optional entry behind its Cargo feature. The root is libavif with
AVIF_CODEC_DAV1D=OFF (port ADR 0061), so the table has no dav1d entry and its
unconditional entries are AOM and the AUTO terminator.

`--registry-only` imports the registry without the optional adapters (which
need the optional engine build of `tools/translate_optional.py`). `--check`
compares instead of writing. The registry comparison covers what the
translation compiles (table, array type, declarations); `raw_sha256` names the
host that produced the committed record and is reported, not compared, because
the C2Rust output of the whole unit differs between macOS and glibc headers.
"""
import argparse
import hashlib
import json
from pathlib import Path
import re

from import_translation import ROOT, translate

FEATURES = {"LIBGAV1": "libgav1", "RAV1E": "rav1e", "SVT": "svt", "AVM": "avm"}
ENTRY = "        {\n            let mut init = AvailableCodec {\n                choice: AVIF_CODEC_CHOICE_"


def registry(avif_raw, source_root=ROOT / "upstream/libavif/src"):
    """Return the feature-guarded registry record for one C2Rust translation of avif.c."""
    avif, _ = translate(source_root / "avif.c", avif_raw, codec_registry=False)
    start = avif.index("static mut availableCodecs:")
    end = avif.index("\nstatic mut availableCodecsCount:", start)
    table = avif[start:end]
    choices = re.findall(r"^                choice: AVIF_CODEC_CHOICE_(\w+),$", table, re.M)
    if "DAV1D" in choices:
        raise SystemExit("The original codec registry has a dav1d entry; translate it with AVIF_CODEC_DAV1D=OFF")
    if set(FEATURES) - set(choices):
        raise SystemExit("Missing original codec registry entries: " + ", ".join(sorted(set(FEATURES) - set(choices))))
    unconditional = [choice for choice in choices if choice not in FEATURES]
    if unconditional != ["AOM", "AUTO"]:
        raise SystemExit(f"Unexpected unconditional codec registry entries: {unconditional}")
    array = (f"[AvailableCodec; {len(unconditional)} + "
             + " + ".join(f'cfg!(feature = "{feature}") as usize' for feature in FEATURES.values()) + "]")
    table, count = re.subn(r"\[AvailableCodec; \d+\]", array, table)
    if count != 1:
        raise SystemExit("Unexpected original codec registry array type")
    for choice, feature in FEATURES.items():
        marker = ENTRY + choice + ","
        if table.count(marker) != 1:
            raise SystemExit(f"Missing original codec registry entry: {choice}")
        table = table.replace(marker, f'        #[cfg(feature = "{feature}")]\n' + marker)
    declarations = re.findall(r"    fn (avifCodec(?:Create|Version)(?:Gav1|Rav1e|Svt|Avm))\([^;]+;", avif_raw)
    return {
        "source": "upstream/libavif/src/avif.c",
        "raw_sha256": hashlib.sha256(avif_raw.encode()).hexdigest(),
        "array_type": array,
        "table": table,
        "declarations": [re.search(r"    fn " + name + r"\([^;]+;", avif_raw).group() for name in declarations],
    }


def compared(record):
    return {key: value for key, value in record.items() if key != "raw_sha256"}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--registry-only", action="store_true", help="Import only the codec registry")
    parser.add_argument("--check", action="store_true", help="Compare without writing")
    parser.add_argument("--registry-raw", type=Path, default=ROOT / "build/generated-avm-registry/src/avif.rs",
                        help="C2Rust output of tools/translate_registry.py")
    args = parser.parse_args()
    raw_root = ROOT / "build/generated-optional/src"
    source_root = ROOT / "upstream/libavif/src"
    if not args.registry_raw.exists():
        raise SystemExit("Generate the full registry with tools/translate_registry.py before importing optional adapters")
    record = registry(args.registry_raw.read_text(), source_root)
    destination = ROOT / "translations/codec-registry.json"
    if args.check:
        committed = json.loads(destination.read_text())
        if compared(committed) != compared(record):
            raise SystemExit(f"The codec registry is not reproducible: {destination}")
        same_host = committed["raw_sha256"] == record["raw_sha256"]
        print("Verified the original codec registry feature guards"
              + ("" if same_host else " (C2Rust unit from another host's headers; the raw digest differs)"))
    else:
        destination.write_text(json.dumps(record, indent=2) + "\n")
        print("Imported the original codec registry feature guards")
    if args.registry_only:
        return

    units = []
    for name in ["codec_libgav1", "codec_rav1e", "codec_svt", "reformat_libyuv", "scale"]:
        source = source_root / f"{name}.c"
        raw = (raw_root / f"{name}.rs").read_text()
        text, corrections = translate(source, raw)
        target = ROOT / "src" / f"{name}.rs" if name.startswith("codec_") else ROOT / "src/variants/libyuv" / f"{name}.rs"
        if args.check:
            if target.read_text() != text:
                raise SystemExit(f"Optional adapter translation is not reproducible: {target}")
        else:
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_text(text)
        units.append({"source": str(source.relative_to(ROOT)), "rust": str(target.relative_to(ROOT)),
                      "raw_sha256": hashlib.sha256(raw.encode()).hexdigest(),
                      "rust_sha256": hashlib.sha256(text.encode()).hexdigest(), "corrections": corrections})
    manifest = {
        "translator": "c2rust 0.21.0", "units": units,
        "libgav1_revision": "c05bf9be660cf170d7c26bd06bb42b3322180e58",
        "rav1e_version": "0.8.1", "svt_version": "4.2.0",
        "libyuv_revision": "26e56be0f984af3dae4d0c0ab7a0bac8ac20e1b0",
        "header_overlay": "Only libyuv's external pointer-only vector records are opaque during translation",
    }
    path = ROOT / "translations/optional-av1.json"
    if args.check:
        if json.loads(path.read_text()) != manifest:
            raise SystemExit(f"The optional adapter manifest is not reproducible: {path}")
    else:
        path.write_text(json.dumps(manifest, indent=2) + "\n")
    print(f"{'Verified' if args.check else 'Imported'} {len(units)} optional adapter modules")


if __name__ == "__main__":
    main()
