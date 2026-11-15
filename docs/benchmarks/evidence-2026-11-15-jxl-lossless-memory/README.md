# Evidence: lossless JPEG XL encode memory, 2026-11-15 (issue #223)

Record: [v1.0-jxl-lossless-chunked-memory-2026-11-15.md](../v1.0-jxl-lossless-chunked-memory-2026-11-15.md).

- `results.txt`, `results-small.txt`: one line per fresh process
  (`/usr/bin/time -l` maximum resident set size; the probe's heap peak),
  two runs per case, and the SHA-256 of every written file.
- `inputs.sha256`: the inputs. `photo24.ppm` (6000x4000) is the 2026-11-12
  `mountain.ppm` (same hash), here decoded by the pinned `djxl` from
  `photo24-d0e7.jxl`; `photo12.ppm` (4032x3024) is the CI `photo12` rows of
  ADR-0039's 2026-11-15 amendment. `c<W>x<H>.ppm` are top-left crops of
  `photo24.ppm` (`tools/crop.py`); every `*a.pam` adds the alpha pattern of
  `tools/rgba.py` (column 0 transparent).
- `tools/lossless_memory_probe.rs`: the port side. Built from the repository
  root, as the engine builds (auto-vectorization on), with a separate target
  directory:

  ```sh
  cp tools/lossless_memory_probe.rs ../../../codecs/libjxl-rs/examples/
  CARGO_TARGET_DIR=target/memprobe CARGO_PROFILE_RELEASE_DEBUG=line-tables-only \
    cargo build --locked --offline --release -p libjxl-rs --example lossless_memory_probe
  ```

- The original: `cjxl v0.13.0 a16d32d [_SCALAR_]` (libjxl `a16d32d0`, the
  `build-scalar` configuration of `scripts/bench/ci-perf/codec_jxl.sh`:
  Clang, CMake Release, `-DHWY_COMPILE_ONLY_SCALAR=1`), run with
  `-d 0 -e 2 --num_threads=0 -x color_space=RGB_D65_SRG_Rel_SRG`, with and
  without `--buffering=0`.
- `tools/run_memory.sh probe cjxl inputs out` runs every case; it was run
  under the shared gate lock. `tools/table.py results.txt` prints the table
  of the record.
