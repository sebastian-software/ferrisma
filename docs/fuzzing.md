# Fuzzing and hostile-input policy

**Status:** Fuzz harness prepared; sustained fuzz campaign **not yet run**.

Ferrixel's public processing facade accepts untrusted JPEG, WebP, JPEG XL, and AVIF bytes. The core forbids unsafe Rust; optional decoder, assembler, SIMD and encoder dependencies do **not** inherit that guarantee. The workspace AVIF reader checks dimensions and reserves the emitted RGB frame before pixel decoding, but libavif/AOM still allocate opaque parser and codec buffers outside the engine budget.

## Reproducible fuzz target

Install the pinned `cargo-fuzz` and run the target with the repository's pinned native nightly toolchain:

```sh
cargo install cargo-fuzz --version 0.13.2 --locked
cargo fuzz run transcode -- -max_len=524288 -timeout=10 -rss_limit_mb=1024
```

The target accepts one control byte followed by unchanged encoded image bytes.
Its two low bits choose JPEG (0), default JPEG XL (1), AVIF (2), or WebP (3);
bit 2 selects WebP instead of AVIF output. Bits 3–4 select the profile policy:
zero uses rendered ICC for JPEG XL and an explicit sRGB assumption only when
metadata is absent for the other formats; 1 is strict; 2 assumes sRGB only when
metadata is absent; 3 requests the rendered-ICC transform. Bit 5 selects the
`f64` reference rather than the `f32` executor. The high two bits select a small
output, 8192×1, 1×8192, or 64×64. Existing JPEG and JPEG XL CI seeds keep their
one-byte 0 and 1 prefixes and now reach the transform with the policies their
fixtures require. The chosen policy remains explicit; unsupported color still
fails rather than being silently reinterpreted.

The driver sets limits of 512 KiB compressed input, 1 million input pixels,
512 KiB retained output, and 2 MiB engine-managed memory, and requests one
transform and encoder thread. The extreme aspect-ratio lanes exercise planning
admission; a resource error is an expected result. These limits do not cap codec
scratch, thread/runtime overhead, or total RSS. Run fuzzing in a sandbox with
process-level limits and sanitizer support; preserve and minimize crashes in
`fuzz/artifacts/` or the upstream codec test corpus. The opt-in workspace JPEG
XL decoder still needs a dedicated lane.

The shared driver has a deterministic smoke test using real JPEG, JPEG XL,
AVIF and WebP fixtures, each with both output formats. It checks that every
framed input reaches the decoder and produces encoded output, and checks strict
color rejection and input/geometry limits:

```sh
cargo test --manifest-path fuzz/Cargo.toml --lib
```

The hosted workflow seeds all eight tested input/output pairs. This workflow
configuration is not a completed campaign result; retain the exact run and
revision when reporting hosted coverage.

The default CI also runs **deterministic malformed-header smoke tests** for truncated JPEG/JXL/AVIF files and invalid API options, so regressions surface without an expensive nightly fuzz dependency.

## M1 exit evidence still needed

- [ ] Record fuzz duration, seed corpus, architecture/toolchain, sanitizer configuration, coverage and any unique crashes.
- [ ] Audit decompression bombs and allocation ceilings at the **decoder preflight** stage before an untrusted AVIF frame is fully decoded.
- [ ] Verify JPEG EXIF orientation, JPEG XL orientation/region coordinate mapping and AVIF orientation are not silently lost.
- [ ] Verify ICC parsing and output-profile contradictions with hostile profiles, not only short headers.
- [ ] Recheck memory limits with actual process RSS, not just `MemoryBudget` snapshots.

The existence of a fuzz target alone does not satisfy the M1 hardening acceptance criteria.

## CI bounded smoke

The separate [M1 Decoder Fuzz Smoke](../.github/workflows/m1-fuzz-smoke.yml) uses the root pinned nightly and cargo-fuzz 0.13.2, verifies deterministic seed success, seeds eight input/output pairs from four real JPEG/JXL/AVIF/WebP fixtures, and runs **3,000 bounded attempts** with a 16 KiB maximum input and a 1 GiB process RSS cap. This is useful to catch shallow codec/protocol panics and integration bugs. It does **not** constitute a sustained, coverage-reviewed fuzz campaign and is not proof of process-wide memory safety; actual release acceptance still requires broader corpora, sanitizers and target-specific time budgets.

### Expanded hosted smoke, 2026-11-11

[Workflow run 43072724334](https://github.com/sebastian-software/ferrixel/actions/runs/43072724334)
completed successfully on revision `8d34b360940a3ad7cff19436baf47f5ae75bd1e4`
with Ubuntu 26.04 x86-64, `nightly-2026-09-07-x86_64-unknown-linux-gnu`,
and cargo-fuzz 0.13.2. The deterministic test exercised all eight combinations
of the four input fixtures and two output formats. The fuzz phase ran 3,000
iterations in 16 seconds without a reported crash; libFuzzer reported 406
retained corpus inputs and `rss: 405Mb`. The command specified a 16 KiB input
limit, a five-second per-input timeout, and a 1 GiB RSS limit. The job log does
not print an explicit sanitizer selection, so this record does not assert one.

The uploaded [corpus artifact](https://github.com/sebastian-software/ferrixel/actions/runs/43072724334/artifacts/13878059847)
has SHA-256 `a0a15b4dc3cfdcc637bcdbdb9c453c4c73a1995985f0c876063c51d49088d4dc`.
This single short run does not close the sustained-campaign, coverage review,
independent sanitizer, or security-sensitive release gates above.

## Early input limits

The experimental public facade rejects compressed input exceeding a configurable byte cap (default **64 MiB**) before opening any decoder and rejects image dimensions above a configured **100-megapixel** default. The AVIF adapter checks the selected primary color tiles' AV1 sequence headers **before full-frame decode**, with ICC copies capped at 16 MiB. This mitigates common decompression-bomb cases; it does **not** bound memory used by libavif/AOM, JPEG XL or encoders. Ferrixel's bounded color/orientation parser and libavif each inspect the AVIF container. Always retain process RSS and sandbox limits for hostile inputs. See [ADR-0031](adr/0031-workspace-libavif-input-experiment.md) for the current boundary; the measured fuzz run below predates this reader replacement.

## First measured fuzz result

[Workflow run 41235323267](https://github.com/sebastian-software/ferrixel/actions/runs/41235323267) completed 3,000 seeded/mutated invocations using cargo-fuzz 0.13.2 and a nightly compiler, without a reported crash. The final corpus was 141 inputs; the log's reported peak RSS was about 139 MiB. This is only a shallow smoke. Continuous fuzzing with longer duration, realistic large color profiles, AVIF grids/alpha and independent sanitizers is **still required** for a security-sensitive release.
