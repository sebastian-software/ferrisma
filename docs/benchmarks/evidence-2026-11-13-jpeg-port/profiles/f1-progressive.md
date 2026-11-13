# prof/f1-progressive.json.gz: total CPU weight 2598465 (thread CPU delta units, us)

## Self (top 25)
| % | function |
| ---: | --- |
| 58.1 | `jpeg::jdphuff::decode_mcu_AC_refine` |
| 10.2 | `jpeg::jdphuff::decode_mcu_AC_first` |
| 9.4 | `jpeg::jdcoefct::consume_data` |
| 3.8 | `jpeg::jidctint_neon::jsimd_idct_islow_neon` |
| 3.6 | `jpeg::jdhuff::jpeg_fill_bit_buffer` |
| 2.6 | `jpeg::jdcolext_neon::jsimd_ycc_rgb_convert_neon::<0, 1, 2, -1, 3>` |
| 2.1 | `jpeg::jidctint_neon::jsimd_idct_islow_pass2_sparse` |
| 1.9 | `jpeg::jidctint_neon::jsimd_idct_islow_pass2_regular` |
| 1.7 | `jpeg::jdphuff::decode_mcu_DC_first` |
| 1.7 | `jpeg::jdsample_neon::jsimd_h2v2_fancy_upsample_neon` |
| 1.4 | `bench::main` |
| 1.0 | `jpeg::jdphuff::decode_mcu_DC_refine` |
| 0.9 | `jpeg::jmemmgr::alloc_barray` |
| 0.4 | `__bzero` |
| 0.3 | `jpeg::jdhuff::jpeg_huff_decode` |
| 0.2 | `jpeg::jsimd::jsimd_idct_islow` |
| 0.2 | `_platform_memset` |
| 0.2 | `jpeg::jdcoefct::decompress_data` |
| 0.1 | `libsystem_malloc.dylib+?` |
| 0.0 | `jpeg::jdsample::sep_upsample` |
| 0.0 | `mach_absolute_time` |
| 0.0 | `jpeg::jmemmgr::access_virt_barray` |
| 0.0 | `jpeg::jdmainct::process_data_context_main` |
| 0.0 | `arm-simd-f1+?` |
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
| 98.6 | `bench::main::{closure#2}` |
| 85.9 | `jpeg::jdapistd::jpeg_start_decompress` |
| 85.9 | `<jpeg::decode::Decoder>::start` |
| 84.9 | `jpeg::jdcoefct::consume_data` |
| 60.4 | `jpeg::jdphuff::decode_mcu_AC_refine` |
| 12.7 | `<jpeg::decode::Scanlines>::read_rows` |
| 12.7 | `jpeg::jdapistd::jpeg_read_scanlines` |
| 12.7 | `jpeg::jdmainct::process_data_context_main` |
| 11.5 | `jpeg::jdphuff::decode_mcu_AC_first` |
| 8.2 | `jpeg::jdcoefct::decompress_data` |
| 7.8 | `jpeg::jidctint_neon::jsimd_idct_islow_neon` |
| 4.3 | `jpeg::jdsample::sep_upsample` |
| 3.6 | `jpeg::jdhuff::jpeg_fill_bit_buffer` |
| 2.6 | `jpeg::jdcolext_neon::jsimd_ycc_rgb_convert_neon::<0, 1, 2, -1, 3>` |
| 2.1 | `jpeg::jidctint_neon::jsimd_idct_islow_pass2_sparse` |
| 2.0 | `jpeg::jdphuff::decode_mcu_DC_first` |
| 1.9 | `jpeg::jidctint_neon::jsimd_idct_islow_pass2_regular` |
| 1.7 | `jpeg::jdsample_neon::jsimd_h2v2_fancy_upsample_neon` |
| 1.7 | `jpeg::jsimd::jsimd_h2v2_fancy_upsample` |

## Stage shares (leaf-to-root first match)
| stage | % CPU | top self functions in stage |
| --- | ---: | --- |
| jpeg-decode | 98.6 | `jpeg::jdphuff::decode_mcu_AC_refine` 58.1; `jpeg::jdphuff::decode_mcu_AC_first` 10.2; `jpeg::jdcoefct::consume_data` 9.4; `jpeg::jidctint_neon::jsimd_idct_islow_neon` 3.8 |
| other | 1.4 | `bench::main` 1.4; `__bzero` 0.1; `libsystem_malloc.dylib+?` 0.0; `<jpeg::decode::Scanlines>::read_rows` 0.0 |
