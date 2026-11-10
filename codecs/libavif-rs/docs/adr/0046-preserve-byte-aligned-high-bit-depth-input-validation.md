# Preserve byte-aligned high-bit-depth input validation

[ADR 0048](0048-require-complete-release-and-focused-debug-aom-acceptance.md)
supersedes the full Debug acceptance requirement below. The correction and
focused alignment regression checks remain required.

Use `core::ptr::read_unaligned` for the single external unsigned-short sample
load in `validate_img`, in all four selected libaom translation profiles.
Keep its native byte order, range comparison, bounds, strides, null check,
and error result unchanged. Record this narrow correction in the generator
and translation inventories. Do not disable Rust's alignment checks globally.

The original macOS ARM64 Debug consumer failed
`HighbdEncodeSmallWidthHeight.SmallWidthMultiThreaded` when the linker's
placement of the original `unsigned char kBuffer[]` had an odd address.
The original C consumer passed the same test. This fixture has byte alignment
and calls `aom_img_wrap` with stride alignment one. A translated raw `u16`
dereference adds a Rust alignment requirement to that external byte buffer.
An unaligned read preserves the supported hosts' observed C load without
constructing an unaligned Rust reference. The original C cast's stricter
language-level alignment assumptions are not a general portability guarantee.

Keep every original source and test object unchanged. Also run a separate
supplemental caller that forces both address parities, retains the original
zero-sample fixture behavior, and places an out-of-range sample at the end of
each plane for 8-, 10-, and 12-bit configurations. Compare exact encoded bytes
or rejection diagnostics with the unchanged C reference. This provides no
original-test execution credit and does not claim that arbitrary unaligned
internal pixel buffers are supported. Original high-bit-depth pointer tagging
and all later encoder operations remain unchanged.

Supersede the incomplete full-suite runs after this production change; require
fresh complete Debug and Release acceptance for the corrected source on both
selected native hosts before changing the root default. Preserve the failure
artifacts and partial old results as historical evidence.

The [Rust unaligned-read contract](https://doc.rust-lang.org/core/ptr/fn.read_unaligned.html)
requires valid initialized readable storage without requiring natural alignment.
