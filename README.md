# libAWFRS

A mechanical Rust port of [libavif](https://github.com/AOMediaCodec/libavif).

The initial goal is source and behavior correspondence, preserving the C ABI,
control flow, data layout, allocation rules, diagnostics, and upstream tests.
Idiomatic Rust APIs and optimizations are deferred until parity is established.

The complete upstream snapshot, including its tests and fixtures, lives in
`upstream/libavif`. See `UPSTREAM.json` for the exact source revision and
`docs/PORTING.md` for the migration contract and validation status.

This repository is a port in progress. Build and parity evidence must be read
with its recorded configuration; untested configurations are not certified.

## License

The upstream BSD-2-Clause license, authorship, and third-party notices are
preserved in the upstream snapshot. Translated files retain their source's
license and attribution.
