# Portable libaom port

Status: implementation in progress. libavif still links the native libaom
engine. Source preparation and the native reference are available; translation,
test correspondence, and acceptance must pass before integration.

The immutable reference is libaom 3.15.2, tag `v3.15.2`, commit
`af3dc9aadc793c1b00edc003e3fc19a44fdeb574`. The complete 1,520-file snapshot,
including original tests and bundled third-party sources, is under
`upstream/aom`. `provenance/aom-source.json` records every SHA-256 digest.
The source BSD-2-Clause license and Alliance for Open Media patent license are
preserved, as are the BSD-3-Clause fastfeat and MIT vector notices.

## Reference boundary

Use upstream's `AOM_TARGET_CPU=generic` build, with both encoder and decoder,
8/10/12-bit support, threads, denoising, quantization matrices, the complete
non-realtime-only encoder, and the original default experiments. Compile with
`-ffp-contract=off`. Debug and Release retain their actual original assertion
and preprocessing differences; Rust development builds remain unoptimized.
SIMD/assembly paths and later optimization are separate work.

Keep `CONFIG_TUNE_VMAF=1`, matching the existing Homebrew engine's optional
VMAF encoding modes. libvmaf is a distinct runtime dependency and remains
native during this milestone. A libaom translation using it must not be
described as a pipeline with no native codec/quality implementations. Its
separate mechanical port and versioned model inputs require their own
contract. Do not silently disable VMAF modes to make that dependency disappear.
The installed package reports libvmaf 3.2.0 through pkg-config despite its
Homebrew package directory being 3.2.1; record both when comparing artifacts.

The library archive contains 168 C translation units, including three generated
configuration/no-op units and four bundled fastfeat/vector units. Bundled
libyuv, WebM I/O, GoogleTest, apps, examples, and the optional C++ rate-control
wrapper are outside this archive. Retain them as original consumers/test
infrastructure and account for their implementation scope explicitly.

## Original test boundary

The initial macOS ARM64 Debug build registers 18,729 main-suite tests, 38
rate-control wrapper tests, and 38 intra-prediction speed tests. Registrations
include upstream-disabled tests; these counts are not executed-pass counts.
Preserve original names, parameters, assertions, disabled status, fixtures,
and expected output. Separate scalar-code coverage from architecture-specific
tests excluded by the declared generic configuration.

Upstream selects 568 test fixtures for this configuration. Their authoritative
checksums are in the immutable `test/test-data.sha1`; the download helper also
records SHA-256 and byte sizes. Do not regenerate goldens. Preserve failed and
skipped inventories, and account separately for header-only C/C++ algorithms
compiled into the test consumer. Linking a test executable alone does not prove
that every original assertion exercises Rust.

## Execution and acceptance

Work proceeds in one checkout with a Solo profile, coherent commits, generated
corrections, narrow comparisons after each correction, and full-suite
checkpoints. No existing consumer switches until standalone acceptance passes.

1. Verify complete source/license hashes, the archive's exact source closure,
   generated headers, configurations, and original test inventories.
2. Prove the representative image/API and encoder/decoder state/error boundary.
   In particular, direct C2Rust `setjmp`/`longjmp` calls are not an accepted Rust
   implementation: retain error targets, diagnostics, return values, cleanup,
   and worker synchronization with an explicit supported control-flow mapping.
3. Import all selected units reproducibly. Preserve field, record, local, and
   static alignment; callback/variadic ABIs; ownership; atomics; and raw C
   address semantics. Record every correction and source/output digest.
4. Run original native and Rust consumers with matching case inventories,
   supplemental exact encode/decode comparisons, defined API errors, allocation
   failures, and instrumented cleanup checks. Audit symbols and link maps for
   original codec fallbacks and accidental symbol interposition.
5. Require Debug and Release evidence on native macOS ARM64 and GNU Linux
   x86-64 before standalone acceptance. Root Linux support remains separate.
6. Replace native libaom in the root archive, audit the full default linkage,
   and rerun original libavif suites and exact AVIF/PNG comparisons. Keep the
   VMAF dependency and any remaining unsupported configurations visible.

```sh
python3 tools/import_aom.py --fetch
python3 tools/verify_upstream.py
python3 tools/generate_licenses.py --check
python3 tools/build_aom.py --debug
python3 tools/build_aom.py
python3 tools/fetch_aom_test_data.py
```

Native build evidence is written to `build/aom-native-{debug,release}`. The
test corpus is cached in `build/deps/aom-test-data`; set
`LIBAOM_TEST_DATA_PATH` to that absolute directory when running original tests.
Downloaded fixtures and disposable builds are not source modifications.
