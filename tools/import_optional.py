#!/usr/bin/env python3
"""Import optional AV1 adapters and translate the C codec registry's feature guards."""
import hashlib
import json
from pathlib import Path
import re

from import_translation import ROOT, translate

raw_root = ROOT / "build/generated-optional/src"
source_root = ROOT / "upstream/libavif/src"
avm_registry = ROOT / "build/generated-avm-registry/src/avif.rs"
avif_raw = avm_registry.read_text() if avm_registry.exists() else (raw_root / "avif.rs").read_text()
avif, _ = translate(source_root / "avif.c", avif_raw, codec_registry=False)
start = avif.index("static mut availableCodecs:")
end = avif.index("\nstatic mut availableCodecsCount:", start)
table = avif[start:end]
features = {"LIBGAV1": "libgav1", "RAV1E": "rav1e", "SVT": "svt"}
if avm_registry.exists():
    features["AVM"] = "avm"
array = "[AvailableCodec; 3 + " + " + ".join(f'cfg!(feature = "{feature}") as usize' for feature in features.values()) + "]"
table = re.sub(r"\[AvailableCodec; \d+\]", array, table)
for choice, feature in features.items():
    marker = "        {\n            let mut init = AvailableCodec {\n                choice: AVIF_CODEC_CHOICE_" + choice + ","
    if marker not in table:
        raise SystemExit(f"Missing original codec registry entry: {choice}")
    table = table.replace(marker, f'        #[cfg(feature = "{feature}")]\n' + marker)
declarations = re.findall(r"    fn (avifCodec(?:Create|Version)(?:Gav1|Rav1e|Svt|Avm))\([^;]+;", avif_raw)
registry = {
    "source": "upstream/libavif/src/avif.c",
    "raw_sha256": hashlib.sha256(avif_raw.encode()).hexdigest(),
    "array_type": array,
    "table": table,
    "declarations": [re.search(r"    fn " + name + r"\([^;]+;", avif_raw).group() for name in declarations],
}
(ROOT / "translations/codec-registry.json").write_text(json.dumps(registry, indent=2) + "\n")

units = []
for name in ["codec_libgav1", "codec_rav1e", "codec_svt", "reformat_libyuv", "scale"]:
    source = source_root / f"{name}.c"
    raw = (raw_root / f"{name}.rs").read_text()
    text, corrections = translate(source, raw)
    target = ROOT / "src" / f"{name}.rs" if name.startswith("codec_") else ROOT / "src/variants/libyuv" / f"{name}.rs"
    target.parent.mkdir(parents=True, exist_ok=True)
    target.write_text(text)
    units.append({"source": str(source.relative_to(ROOT)), "rust": str(target.relative_to(ROOT)),
                  "raw_sha256": hashlib.sha256(raw.encode()).hexdigest(),
                  "rust_sha256": hashlib.sha256(text.encode()).hexdigest(), "corrections": corrections})
(ROOT / "translations/optional-av1.json").write_text(json.dumps({
    "translator": "c2rust 0.21.0", "units": units,
    "libgav1_revision": "c05bf9be660cf170d7c26bd06bb42b3322180e58",
    "rav1e_version": "0.8.1", "svt_version": "4.2.0",
    "libyuv_revision": "26e56be0f984af3dae4d0c0ab7a0bac8ac20e1b0",
    "header_overlay": "Only libyuv's external pointer-only vector records are opaque during translation",
}, indent=2) + "\n")
print(f"Imported {len(units)} optional adapter modules and the original codec registry feature guards")
