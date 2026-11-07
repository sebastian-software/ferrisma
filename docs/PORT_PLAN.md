# Port plan

The decoder is ported in work packages. Each package translates its units
([ADR 0001](adr/0001-mechanical-port.md)), proves them against the C oracle
([ADR 0002](adr/0002-differential-oracle.md)) and records what is verified in
[PORT_STATUS.md](PORT_STATUS.md). A package starts only when the packages it
depends on are merged.

| WP | Content | Upstream units | Depends on | Done when |
| --- | --- | --- | --- | --- |
| 0 | Tooling and representation | oracle build script, probe runner, unit map from upstream files to Rust modules and packages, module skeleton, check script, CI oracle job; an ADR for the Rust representation of the libjpeg object model (method tables, the pool allocator, `JSAMPARRAY` row pointers, error exits and warnings, suspending data sources): [ADR 0005](adr/0005-rust-representation.md) | — | The oracle builds on both CI hosts; the 8-bit `djpeg` bit tests pass against the oracle and are inventoried with their scope |
| 1 | Common infrastructure | `jerror.c`, `jcomapi.c` (except `jpeg_abort`), `jutils.c`, `jmemmgr.c`, `jmemnobs.c`, types from `jpeglib.h`, `jpegint.h`, `jmorecfg.h`, `jerror.h` | 0 | Error codes, messages and allocation limits match the oracle |
| 2 | Input and markers | `jdapimin.c` (except `jpeg_finish_decompress`), `jdatasrc.c` (`jpeg_mem_src`), `jdinput.c`, `jdmarker.c`, `jdicc.c`, `jpeg_abort` from `jcomapi.c`, the struct of `jdmaster.h` | 1 | `jpeg_read_header` results, ICC extraction, warnings and errors match on the test images and the fuzz corpus |
| 3 | Entropy decoding | `jdhuff.c`, `jdphuff.c`, `jaricom.c`, `jdarith.c` | 2 | Decoded coefficient blocks match for baseline, progressive and arithmetic files, including corrupt-data recovery |
| 4 | Coefficients and IDCT | `jdcoefct.c`, `jddctmgr.c`, `jidctint.c`, `jidctfst.c`, `jidctflt.c`, `jidctred.c` | 3 | Sample blocks match for every DCT method and every scaled size |
| 5 | Upsampling, color, buffers | `jdsample.c`, `jdmerge.c`, `jdmrgext.c`, `jdcolor.c`, `jdcolext.c`, `jdpostct.c`, `jdmainct.c` | 4 | Output rows match for every in-scope color space and sampling factor, with and without fancy upsampling |
| 6 | Control and API | `jdmaster.c`, `jdapistd.c`, `jpeg_finish_decompress` from `jdapimin.c` | 5 | Full decodes match byte for byte, including `jpeg_crop_scanline`, `jpeg_skip_scanlines` and scaled output |
| 7 | C ABI and upstream tests | exported libjpeg decompression API (needs its unsafe ADR) | 6 | Unmodified `djpeg` linked against the port passes the in-scope bit tests with upstream's hashes; decompression fuzzers run clean against both libraries |
| 8 | Safe Rust API for Ferrixel | row-streaming decoder with scale, crop, ICC and EXIF access, `Result` errors | 6 | Ferrixel's `jpeg-read` adapter can replace `zune-jpeg` behind ADR-0025's gates |
| 9 | NEON SIMD | `simd/arm` intrinsics (needs the SIMD unsafe ADR) | 6 | Bit-identical to the `WITH_SIMD=1` oracle on ARM64; speed measured against libjpeg-turbo |
| 10 | x86-64 SIMD | intrinsics rewritten from `simd/x86_64` NASM ([ADR 0004](adr/0004-simd-after-the-portable-port.md)) | 6 | Bit-identical to the `WITH_SIMD=1` oracle on x86-64; speed measured per kernel |
| 11 | Conformance | ISO/IEC 10918-2 compliance data for baseline, extended 8-bit and progressive processes | 6 | Recorded conformance run for the processes in scope |

Packages 1 to 5 can overlap once package 0 has fixed the module skeleton and
the representation ADR; their tests depend on the earlier stages only through
probes. Each package adds the `jpegint.h` interface, the private struct and the
`jpeg_decompress_struct` slot of every module it translates, with the object
fields that depend on that module ([ADR 0005](adr/0005-rust-representation.md)),
so a package whose code calls another module's methods follows the package that
adds that module.

## Not planned

Encoder, transcoding, 12- and 16-bit samples, lossless JPEG, color
quantization, RGB565 output and the TurboJPEG API
([ADR 0001](adr/0001-mechanical-port.md)).
