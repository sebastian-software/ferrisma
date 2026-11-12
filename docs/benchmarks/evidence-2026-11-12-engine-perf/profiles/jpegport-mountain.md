# prof/jpegport-mountain.json.gz: total CPU weight 2341243 (thread CPU delta units, us)

## Self (top 25)
| % | function |
| ---: | --- |
| 47.0 | `jpeg::jdhuff::decode_mcu` |
| 25.3 | `jpeg::jdhuff::HUFF_DECODE_FAST` |
| 7.1 | `jpeg::jidctint_neon::jsimd_idct_islow_neon` |
| 4.2 | `bench::main` |
| 4.1 | `jpeg::jdcolext_neon::jsimd_ycc_rgb_convert_neon::<0, 1, 2, -1, 3>` |
| 4.1 | `jpeg::jidctint_neon::jsimd_idct_islow_pass2_regular` |
| 2.6 | `jpeg::jdsample_neon::jsimd_h2v2_fancy_upsample_neon` |
| 2.5 | `jpeg::jidctint_neon::jsimd_idct_islow_pass2_sparse` |
| 1.9 | `jpeg::jdcoefct::decompress_onepass` |
| 0.3 | `jpeg::jsimd::jsimd_idct_islow` |
| 0.3 | `_platform_memset` |
| 0.3 | `__bzero` |
| 0.2 | `read` |
| 0.1 | `jpeg::jdsample::sep_upsample` |
| 0.1 | `libsystem_malloc.dylib+?` |
| 0.0 | `jpeg::jdmainct::process_data_context_main` |
| 0.0 | `<jpeg::decode::Decoder>::with_options` |
| 0.0 | `mach_absolute_time` |
| 0.0 | `jpeg_bench_port+?` |
| 0.0 | `jpeg::jmemmgr::free_pool` |
| 0.0 | `_platform_memmove` |
| 0.0 | `jpeg::jdsample::fullsize_upsample` |
| 0.0 | `jpeg::jdapistd::jpeg_read_scanlines` |
| 0.0 | `write` |
| 0.0 | `jpeg::jdapimin::jpeg_CreateDecompress` |

## Inclusive (top 25)
| % | function |
| ---: | --- |
| 100.0 | `bench::main` |
| 100.0 | `std::sys::backtrace::__rust_begin_short_backtrace::<fn() -> std::process::ExitCode, std::process::ExitCode>` |
| 100.0 | `std::rt::lang_start::<std::process::ExitCode>::{closure#0}` |
| 100.0 | `std::rt::lang_start_internal` |
| 100.0 | `main` |
| 100.0 | `start` |
| 95.7 | `bench::main::{closure#2}` |
| 95.3 | `jpeg::jdapistd::jpeg_read_scanlines` |
| 95.3 | `<jpeg::decode::Scanlines>::read_rows` |
| 95.3 | `jpeg::jdmainct::process_data_context_main` |
| 88.4 | `jpeg::jdcoefct::decompress_onepass` |
| 72.3 | `jpeg::jdhuff::decode_mcu` |
| 25.3 | `jpeg::jdhuff::HUFF_DECODE_FAST` |
| 13.6 | `jpeg::jidctint_neon::jsimd_idct_islow_neon` |
| 6.8 | `jpeg::jdsample::sep_upsample` |
| 4.1 | `jpeg::jdcolext_neon::jsimd_ycc_rgb_convert_neon::<0, 1, 2, -1, 3>` |
| 4.1 | `jpeg::jidctint_neon::jsimd_idct_islow_pass2_regular` |
| 2.6 | `jpeg::jsimd::jsimd_h2v2_fancy_upsample` |
| 2.6 | `jpeg::jdsample_neon::jsimd_h2v2_fancy_upsample_neon` |
| 2.5 | `jpeg::jidctint_neon::jsimd_idct_islow_pass2_sparse` |
| 0.3 | `jpeg::jsimd::jsimd_idct_islow` |
| 0.3 | `_platform_memset` |
| 0.3 | `__bzero` |
| 0.2 | `read` |
| 0.2 | `io::read::default_read_to_end::<std::fs::File>` |

## Stage shares (leaf-to-root first match)
| stage | % CPU | top self functions in stage |
| --- | ---: | --- |
| jpeg-decode | 95.4 | `jpeg::jdhuff::decode_mcu` 47.0; `jpeg::jdhuff::HUFF_DECODE_FAST` 25.3; `jpeg::jidctint_neon::jsimd_idct_islow_neon` 7.1; `jpeg::jdcolext_neon::jsimd_ycc_rgb_convert_neon::<0, 1, 2, -1, 3>` 4.1 |
| other | 4.6 | `bench::main` 4.2; `__bzero` 0.2; `read` 0.2; `<jpeg::decode::Decoder>::with_options` 0.0 |
