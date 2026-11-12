# Unsafe, native, and FFI boundary review

This review covers the dependency closure of the supported v1.0 feature
sets (`cli`, each adapter feature, and `cli,jpeg-xl-own-read`) at commit
`90b638dd` plus this branch's metadata changes, on 2026-11-12. It records which
packages contain `unsafe`, assembly, foreign-function interfaces, or
exported C symbols, what isolates them from Ferrisma callers, and what was
and was not examined. It is evidence for #66, not a soundness proof or a
security audit.

## Method

1. The exact Cargo graphs come from `scripts/production-license-bundle.py`
   for `aarch64-apple-darwin` and `x86_64-unknown-linux-gnu`; both targets
   resolve the same 93 `cli` packages (95 with `jpeg-xl-own-read`).
2. A lexical screen counted `unsafe` blocks, functions and impls, `extern "C"`
   blocks, `no_mangle` exports, `asm!`, `std::arch`/`core::arch` uses, and
   non-Rust sources under each package's `src/` and `build.rs`. Comments and
   strings are not excluded, and macro-generated code is not expanded. The
   script and its full table are reproducible from the bundle inventory; the
   counts below are from that run.
3. The project-owned boundary crates were read for their isolation design:
   crate lint levels, where `unsafe` is permitted, whether every block has a
   documented safety argument, and which safe types callers receive.
4. Third-party registry packages were screened only. No third-party code was
   audited line by line.

Package presence in the Cargo graph does not prove that code is linked into a
binary, and an `unsafe` count says nothing about soundness.

## Engine and project-owned boundaries

| Package | Lint boundary | What is `unsafe` | Isolation |
| --- | --- | --- | --- |
| `ferrisma` | `#![forbid(unsafe_code)]` and `unsafe_code = "forbid"` | Nothing | Public API exposes only owned or borrowed safe types; codec types never appear in signatures (ADR-0017) |
| `ferrisma-kernels` | `unsafe_code = "deny"`; only `dispatch.rs` and `lanes.rs` allow it; Clippy denies undocumented blocks and multiple unsafe operations per block | 40 blocks: `std::arch` NEON/AVX2/AVX-512 intrinsics behind runtime feature detection | Safe `Kernels` API; scalar/SIMD differential tests (`tests/differential.rs`); ADR-0020 |
| `ferrisma-avif-read` | `unsafe_op_in_unsafe_fn` and Clippy `undocumented_unsafe_blocks` denied | 10 blocks calling the generated libavif C ABI (decoder create/destroy, parse, decode, RGB conversion) | Owning `Decoder<'a>` ties the input slice lifetime; caller-owned output buffers are size-checked; codec scratch stays opaque (ADR-0031) |
| `ferrisma-avif-write` | Same as the reader | 14 blocks: image/encoder lifetime, RGB→YUV, encode, output slice | Owning wrappers destroy each libavif object once; output copied into a `Vec` before release (ADR-0037) |
| `ferrisma-jxl-decoder` | Clippy `undocumented_unsafe_blocks` denied | 7 blocks over libjxl-rs's decoder C ABI | Owning decoder; bounded full-frame output; ADR-0032 |
| `libsharpyuv-core` | `#![forbid(unsafe_code)]` | Nothing | Safe conversion core shared by WebP and AVIF (ADR-0033) |
| `libjpeg-turbo-rs` | `#![deny(unsafe_code)]`; allowed with stated reasons only in `neon.rs`, `x86.rs`, and `jsimd.rs`; translated IDCT/upsampling SIMD files `forbid` it | 34 blocks, 9 unsafe fns: SIMD loads/stores and kernel calls | Safe `decode` API used by the engine; codec ADRs 0007 and 0008 |
| `libwebp-rs` | `#![deny(unsafe_code)]`; allowed only on SIMD items in `dsp/mod.rs` and `sharpyuv/mod.rs` | 592 blocks, 157 unsafe fns (SSE2/SSE4.1/AVX2/NEON intrinsics), one `extern "C"` type block in `src/abi` | Engine uses the safe `api::decode`/`api::encode` modules; the C ABI is not reached by the engine; codec ADR 0008 |
| `libjxl-rs` | No crate-level unsafe lint | 1,087 blocks, 413 unsafe fns, 13 unsafe impls, 61 `no_mangle` decoder C-ABI exports, inline `asm!` in Brotli, skcms and CMS helpers | Reached only through `ferrisma-jxl-decoder` (opt-in `jpeg-xl-own-read`); the encoder sources added for JPEG XL output need the same review when their engine adapter lands |
| `libavif-rs`, `libavif-aom`, `libavif-dav1d`, `libavif-sharpyuv` | Mechanically translated C (c2rust); no unsafe lint | Tens of thousands of unsafe fns (`aom` 54,817; `libavif-rs` 15,700; `dav1d` 10,067), hundreds of `extern "C"` blocks, about 17,300 `no_mangle` exports, `asm!` only in Linux-specific AOM CPU dispatch and the dav1d x86 register barrier, nightly features `extern_types`, `core_intrinsics`, `c_variadic` | Reached only through `ferrisma-avif-read` and `ferrisma-avif-write`; byte-verified against pinned C references (codec ADRs 0012, 0024); AOM maps C `longjmp` to typed Rust unwinding through `extern "C-unwind"` (codec ADR 0036) |

The engine crate itself contains no `unsafe`. Every unsafe operation the
engine reaches sits in a separate package whose safe API the engine calls.

## Third-party registry code (screen only)

| Package | Unsafe/native content in the screen | Reached through |
| --- | --- | --- |
| `rav1e` 0.8.1 | 144 unsafe blocks, 86 unsafe fns, 94 assembly sources built by `cc` (aarch64) or NASM (x86-64) | `avif-write` (default AVIF encoder) |
| `av-scenechange` 0.14.1 | 27 native sources, `extern "C"` blocks | `rav1e` |
| `moxcms` 0.9.1, `pxfm` 0.1.30 | SIMD intrinsics (240 and 123 unsafe blocks) | `icc` |
| `jxl-oxide` family (`jxl-render`, `jxl-grid`, `jxl-color`, `jxl-modular`, …) | SIMD intrinsics, 21–46 unsafe blocks per package | `jpeg-xl-read` |
| `libc`, `crossbeam-*`, `rayon-core`, `bytemuck`, `memchr`, `once_cell`, `aligned-vec`, `arrayvec` | Common runtime unsafe | Several features |
| `cc`, `nasm-rs`, `jobserver`, `find-msvc-tools`, `shlex` | Host-executed build code | Build-only edges of `rav1e` |

## Findings

1. **Consumer profile requirements (release blocker for published AVIF
   features).** The generated AVIF packages rely on wrapping arithmetic and
   on unwinding: the workspace sets `overflow-checks = false` for
   `libavif-rs`, `libavif-aom`, `libavif-dav1d`, and `libavif-sharpyuv`, and
   `panic = "unwind"` (root `Cargo.toml`, ADR-0026). Cargo profiles of a
   dependency's workspace do not apply to its dependents. A crates.io
   consumer that enables `avif-read`, `avif-own-write`, or `cli` would build
   those packages with overflow checks in its `dev` and `test` profiles, and
   AOM's error recovery would abort under `panic = "abort"`. The AVIF ABI
   probes (`codecs/libavif-rs/tools/check_aom_abi.py`, `check_abi.py`) set
   exactly these profile keys for their own consumers. This was derived from
   the configuration and tooling; a debug-profile failure in an external
   consumer was not reproduced here. Until the generated code no longer needs
   the exception, the published documentation must state the required
   consumer profile (see [publishing](publishing.md)).
2. **Global C symbol namespace.** The AVIF packages export about 17,300
   unmangled C symbols (`aom_*`, `dav1d_*`, `avif*`, `SharpYuv*`); libjxl-rs
   exports 61 decoder symbols. A binary that also links the original C
   libraries, or another crate exporting the same names, fails to link or
   binds unpredictably. ADR-0031 removed the `rav1d` collision inside
   Ferrisma; downstream combinations remain the consumer's risk.
3. **Nightly-only code.** The AVIF packages use unstable features. Stable
   consumers must not enable `avif-read`, `avif-own-write`, or `cli`.
4. **Libjxl-rs has no crate-level unsafe lint**, unlike the JPEG and WebP
   ports. Its decoder is opt-in today. The JPEG XL output work adds encoder
   sources to the same package; the engine adapter for it should be a
   separate safe wrapper like `ferrisma-jxl-decoder`, and this review must be
   re-run for that graph.
5. **Every project-owned boundary block is documented.** The three bridge
   crates deny undocumented unsafe blocks, and `ferrisma-kernels` denies them
   too. That establishes that each block states its invariant, not that the
   invariant holds.

## Not done

- No line-by-line soundness audit of any crate, including the bridges.
- No Miri, sanitizer, or fuzz run as part of this review; the existing
  sanitizer and fuzz evidence is recorded in [fuzzing](fuzzing.md) and the
  codec validation tiers.
- No review of linked-binary contents (which `no_mangle` symbols survive
  linking, which assembly is selected at run time).
- No independent review of third-party unsafe code, including `rav1e`
  assembly and `moxcms` SIMD.
- The x86-64 Linux graph was resolved but not built for this review.

## Re-run

```sh
python3 scripts/production-license-bundle.py --target aarch64-apple-darwin \
  --features cli,jpeg-xl-own-read --output target/license-bundle-review
```

Then repeat the lexical screen over the packages in
`target/license-bundle-review/inventory.json`. Re-run when a dependency, the
lockfile, a codec port's unsafe policy, or the published feature set changes,
and before every release.
