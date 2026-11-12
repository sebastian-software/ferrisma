# prof/jpegport-beach.json.gz: total CPU weight 1635220 (thread CPU delta units, us)

## Self (top 25)
| % | function |
| ---: | --- |
| 41.7 | `jpeg::jdhuff::decode_mcu` |
| 15.1 | `bench::main` |
| 12.3 | `jpeg::jidctint_neon::jsimd_idct_islow_neon` |
| 9.4 | `jpeg::jidctint_neon::jsimd_idct_islow_pass2_sparse` |
| 5.8 | `jpeg::jdcoefct::decompress_onepass` |
| 4.8 | `jpeg::jdhuff::jpeg_fill_bit_buffer` |
| 4.6 | `jpeg::jdcolext_neon::jsimd_ycc_rgb_convert_neon::<0, 1, 2, -1, 3>` |
| 2.5 | `jpeg::jidctint_neon::jsimd_idct_islow_pass2_regular` |
| 0.9 | `jpeg::jdhuff::jpeg_huff_decode` |
| 0.9 | `__bzero` |
| 0.5 | `jpeg::jsimd::jsimd_idct_islow` |
| 0.4 | `_platform_memset` |
| 0.3 | `read` |
| 0.2 | `jpeg::jdsample::sep_upsample` |
| 0.2 | `jpeg_bench_inline+?` |
| 0.1 | `jpeg::jdmainct::process_data_simple_main` |
| 0.1 | `<jpeg::decode::Decoder>::new` |
| 0.1 | `libsystem_malloc.dylib+?` |
| 0.0 | `<jpeg::decode::Recorder as jpeg::jerror::jpeg_error_methods>::emit_message` |
| 0.0 | `jpeg::jdsample::fullsize_upsample` |
| 0.0 | `jpeg::jdmarker::jpeg_save_markers` |
| 0.0 | `mach_absolute_time` |
| 0.0 | `<jpeg::jdatasrc::MemSourceMethods as jpeg::jpeglib::jpeg_source_methods>::init_source` |
| 0.0 | `OUTLINED_FUNCTION_29` |
| 0.0 | `jpeg::jdapistd::jpeg_read_scanlines` |

## Inclusive (top 25)
| % | function |
| ---: | --- |
| 100.0 | `bench::main` |
| 100.0 | `std::sys::backtrace::__rust_begin_short_backtrace::<fn() -> std::process::ExitCode, std::process::ExitCode>` |
| 100.0 | `std::rt::lang_start::<std::process::ExitCode>::{closure#0}` |
| 100.0 | `std::rt::lang_start_internal` |
| 100.0 | `main` |
| 100.0 | `start` |
| 84.6 | `bench::main::{closure#2}` |
| 83.7 | `jpeg::jdapistd::jpeg_read_scanlines` |
| 83.7 | `<jpeg::decode::Scanlines>::read_rows` |
| 83.7 | `jpeg::jdmainct::process_data_simple_main` |
| 78.7 | `jpeg::jdcoefct::decompress_onepass` |
| 47.5 | `jpeg::jdhuff::decode_mcu` |
| 24.2 | `jpeg::jidctint_neon::jsimd_idct_islow_neon` |
| 9.4 | `jpeg::jidctint_neon::jsimd_idct_islow_pass2_sparse` |
| 4.8 | `jpeg::jdhuff::jpeg_fill_bit_buffer` |
| 4.8 | `jpeg::jdsample::sep_upsample` |
| 4.6 | `jpeg::jdcolext_neon::jsimd_ycc_rgb_convert_neon::<0, 1, 2, -1, 3>` |
| 2.5 | `jpeg::jidctint_neon::jsimd_idct_islow_pass2_regular` |
| 1.0 | `jpeg::jdhuff::jpeg_huff_decode` |
| 0.9 | `__bzero` |
| 0.5 | `jpeg::jsimd::jsimd_idct_islow` |
| 0.4 | `_platform_memset` |
| 0.3 | `read` |
| 0.3 | `io::read::default_read_to_end::<std::fs::File>` |
| 0.3 | `std::fs::read::inner` |

## Stage shares (leaf-to-root first match)
| stage | % CPU | top self functions in stage |
| --- | ---: | --- |
| jpeg-decode | 83.7 | `jpeg::jdhuff::decode_mcu` 41.7; `jpeg::jidctint_neon::jsimd_idct_islow_neon` 12.3; `jpeg::jidctint_neon::jsimd_idct_islow_pass2_sparse` 9.4; `jpeg::jdcoefct::decompress_onepass` 5.8 |
| other | 16.3 | `bench::main` 15.1; `__bzero` 0.8; `read` 0.3; `<jpeg::decode::Decoder>::new` 0.1 |
