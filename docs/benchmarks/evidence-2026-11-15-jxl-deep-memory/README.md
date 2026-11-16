# Evidence: lossless JPEG XL encode memory above 8 bits, 2026-11-15

Record: [v1.0-jxl-deep-memory-2026-11-15.md](../v1.0-jxl-deep-memory-2026-11-15.md).

- `results.txt`: one line per case: input, tool, the larger peak RSS of two
  fresh processes (`/usr/bin/time -l`, bytes), the first 16 hex digits of
  the written file's SHA-256 (port and `cjxl` equal on every line pair).
- `inputs.sha256`: the inputs, written by `tools/tile.py` from
  `codecs/libavif-rs/upstream/libavif/tests/data/weld_16bit.png` (a 16-bit
  photograph, 1024x684), mirrored to each size; `-8` files take the high
  byte of the same samples (the same content at 8 bits), `a` files add the
  alpha pattern of the 2026-11-15 lossless evidence.
- The port side is `crates/ferrisma-jxl-encoder/examples/encode_pnm.rs`
  (release), the bridge call the engine's JPEG XL sink makes
  (`encode_lossless`, effort 2, default buffering, `uint16` with
  `JXL_BIT_DEPTH_FROM_CODESTREAM` above 8 bits). It holds the input once
  (6 B/px RGB, 8 B/px RGBA at 16 bits).
- The original: `cjxl v0.13.0 a16d32d0 [_SCALAR_]` (the `build-scalar`
  configuration), `-d 0 -e 2 --num_threads=0 -x color_space=RGB_D65_SRG_Rel_SRG`.
- `tools/run_memory.sh <encode_pnm> <cjxl> <inputs> <out>` ran every case
  under the shared gate lock.
