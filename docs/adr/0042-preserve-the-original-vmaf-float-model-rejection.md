# Preserve the original VMAF float-model rejection

Status: superseded by [ADR 0057](0057-translate-libaom-without-vmaf-tuning.md).

The pinned libaom passes `"1"` as the `float_motion` extractor's
`motion_force_zero` option, while native libvmaf v3.2.1 accepts only `"true"`
or `"false"` for a boolean. Preserve the original fatal diagnostic and exit
code in the mechanical port; changing the option text belongs to a separate
behavior change. Supplementary checks must establish that the float model
loads without the overload, rejects `"1"` with `-EINVAL`, and accepts `"true"`
before comparing the twelve original C/Rust rejection scenarios. Record those
as error behavior, separately from successful integer-model stream coverage.
