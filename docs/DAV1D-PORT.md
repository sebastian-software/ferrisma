# Portable dav1d port

Status: source and test inputs pinned; reference inventory and translation in progress.

The selected decoder is dav1d 1.5.4 at
`54706fc6bc0cdecab7e9593974a4039cc038fca7`. Its complete immutable source,
including original tests, is under `upstream/dav1d`. The separate original
checkasm framework and decoder corpus are fetched into `build/deps` with all
tracked file hashes recorded in `provenance/dav1d-test-inputs.json`.

```sh
python3 tools/import_dav1d.py --fetch
python3 tools/verify_upstream.py
python3 tools/generate_licenses.py --check
```

The reference configuration includes both bit-depth instantiations, logging,
threading, all scalar DSP functions, no assembly, and `-ffp-contract=off`.
Original CLI muxers, demuxers, test code, and system facilities remain native
test infrastructure. The optional xxHash output muxer is outside the decoder
and is disabled; the original built-in MD5 verifier remains enabled.

Required evidence before integration:

- Exact original test-data expectations for 8/10/12-bit streams, film grain,
  mixed bit depths, scalable streams, and malformed/regression inputs.
- Original checkasm DSP test bodies comparing original C and Rust pointers,
  with nonzero matching function inventories in both bit-depth instantiations.
- Public API, ABI, packet/frame ownership, flush/drain, custom allocation,
  callback, error, and multithreaded behavior compared independently.
- Debug and Release comparisons, instrumented cleanup checks, and native
  macOS ARM64 and Linux x86-64 validation.
- Reproducible source translation, complete licenses, symbol/linkage audit,
  and the original libavif integration suite after dependency replacement.

These are pending gates, not claims of completed decoder parity. The existing
libavif implementation continues to use native dav1d while this crate is built.
