# Build the root with AVIF_CODEC_DAV1D=OFF

Supersedes [ADR 0032](0032-deliver-dav1d-through-the-workspace-crate.md).
Context: [engine ADR-0031](../../../../docs/adr/0031-workspace-libavif-input-experiment.md),
amendment of 2026-11-15 (owner decision on issue #54): dav1d leaves the
runtime and the published package chain, as rav1e and the jxl-oxide reader did,
and may stay in the tests as an independent cross-check.

## Decision

The root translation is the translation of upstream libavif built with
`AVIF_CODEC_DAV1D=OFF`. There is no Cargo feature for dav1d.

- `libavif-rs` does not depend on `libavif-dav1d`. The `codec_dav1d` unit and
  its entry in the codec table leave all 16 root profiles, the macOS base tree
  and the variants. AV1 decoding goes through the libaom translation, which is
  the AV1 reference implementation (ADR 0049). Upstream reads the define in one
  place only, the codec table of `src/avif.c`; the AUTO choice then decodes
  with AOM (with libgav1 in the configurations that select that optional
  engine, which precedes AOM in the table), in the original and in the port
  alike.
- The C reference the root is compared with is built the same way
  (`tools/build.py` passes `-DAVIF_CODEC_DAV1D=OFF`; the root references of
  `tools/build_root_references.py` contain SharpYUV only), so the original
  stays the reference for this configuration and ADR-0041's bit identity is
  checked against it.
- The change goes through the generators, not by hand: `root_variants.EXCLUDED`
  removes `codec_dav1d.c` from every closure, the codec registry is translated
  from a DAV1D=OFF reference (`tools/translate_registry.py` rejects a dav1d
  define; `tools/import_optional.py` rejects a dav1d entry and has a `--check`
  mode), `tools/transpile_root.py --replace` (through
  `tools/root_pipeline.py --regenerate`) replaces the profiles, and
  `tools/import_translation.py --generation` imports the base tree and the
  non-AVM variants from the macOS Release generations, whose units equal the
  base tree's. `tools/check_aom_root_linkage.py` rejects any dav1d owner in a
  root consumer; `tools/check_dav1d_linkage.py --root-library` is gone.
- `libavif-dav1d` stays in the workspace as an unpublished crate with its own
  parity suite (`avif-dav1d.yml`) as the independent cross-check.
- `LICENSE` keeps its VideoLAN paragraph: libavif's own `src/obu.c` is derived
  from dav1d and stays in the root.

## Consequences

- The original upstream tests and applications run against the DAV1D=OFF
  build on both sides; `avifcodectest`'s dav1d cases skip on both sides.
  `tools/differential.py`'s default pass decodes with AOM, like its explicit
  `aom` pass.
- Decoding speed: the port's AVIF decode uses the libaom translation. Stock
  libavif with dav1d and its assembly is faster; the hosted benchmark keeps a
  C libavif with dav1d context column (`c-dav1d`) for that comparison.

## Status

Accepted 2026-11-16. Implementation in progress: the tools are changed; the
profiles, the base tree and the variants are regenerated from hosted native
runs, and the AVM variants, which only a local build with AVM v1.0.0
regenerates, are still open. This record is completed when the removal lands.
