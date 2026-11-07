# Retain libaom's optional VMAF modes

Status: superseded by [ADR 0057](0057-translate-libaom-without-vmaf-tuning.md).

Pin libaom 3.15.2 with upstream's complete generic encoder/decoder configuration
and `CONFIG_TUNE_VMAF=1`, preserving the optional encoding modes exposed by the
existing native dependency. Keep libvmaf as an explicitly inventoried native
quality-engine dependency until a separate port establishes its own source,
model, and test contract; disabling those modes would shrink the behavior
boundary, and a libaom milestone using native VMAF must not claim an entirely
Rust implementation of that transitive closure.
