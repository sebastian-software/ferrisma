# prof/port-progressive.json.gz: total CPU weight 4049525 (thread CPU delta units, us)

## Self (top 25)
| % | function |
| ---: | --- |
| 46.6 | `jpeg::jdphuff::decode_mcu_AC_refine` |
| 16.6 | `libsystem_malloc.dylib+?` |
| 8.5 | `jpeg::jdphuff::decode_mcu_AC_first` |
| 8.1 | `jpeg::jdcoefct::consume_data` |
| 3.4 | `jpeg::jidctint_neon::jsimd_idct_islow_neon` |
| 3.0 | `jpeg::jdhuff::jpeg_fill_bit_buffer` |
| 2.1 | `jpeg::jdcolext_neon::jsimd_ycc_rgb_convert_neon::<0, 1, 2, -1, 3>` |
| 1.6 | `jpeg::jdsample_neon::jsimd_h2v2_fancy_upsample_neon` |
| 1.5 | `jpeg::jdphuff::decode_mcu_DC_first` |
| 1.4 | `jpeg::jidctint_neon::jsimd_idct_islow_pass2_regular` |
| 1.4 | `jpeg::jidctint_neon::jsimd_idct_islow_pass2_sparse` |
| 1.1 | `jpeg::jdphuff::decode_mcu_DC_refine` |
| 1.0 | `bench::main` |
| 0.8 | `jpeg::jmemmgr::alloc_barray` |
| 0.7 | `arm-simd-base+?` |
| 0.4 | `__rustc::__rdl_alloc` |
| 0.3 | `jpeg::jdhuff::jpeg_huff_decode` |
| 0.3 | `jpeg::jdcoefct::decompress_data` |
| 0.2 | `mach_absolute_time` |
| 0.2 | `__bzero` |
| 0.2 | `_platform_memset` |
| 0.2 | `jpeg::jsimd::jsimd_idct_islow` |
| 0.1 | `__rustc::__rust_no_alloc_shim_is_unstable_v2` |
| 0.1 | `jpeg::jmemmgr::access_virt_barray` |
| 0.0 | `jpeg::jdsample::sep_upsample` |

## Inclusive (top 25)
| % | function |
| ---: | --- |
| 100.0 | `bench::main` |
| 100.0 | `std::sys::backtrace::__rust_begin_short_backtrace::<fn() -> std::process::ExitCode, std::process::ExitCode>` |
| 100.0 | `std::rt::lang_start::<std::process::ExitCode>::{closure#0}` |
| 100.0 | `std::rt::lang_start_internal` |
| 100.0 | `main` |
| 100.0 | `start` |
| 99.0 | `bench::main::{closure#2}` |
| 88.2 | `jpeg::jdapistd::jpeg_start_decompress` |
| 88.2 | `<jpeg::decode::Decoder>::start` |
| 87.4 | `jpeg::jdcoefct::consume_data` |
| 48.7 | `jpeg::jdphuff::decode_mcu_AC_refine` |
| 16.8 | `libsystem_malloc.dylib+?` |
| 10.6 | `<jpeg::decode::Scanlines>::read_rows` |
| 10.6 | `jpeg::jdapistd::jpeg_read_scanlines` |
| 10.6 | `jpeg::jdmainct::process_data_context_main` |
| 9.4 | `jpeg::jdphuff::decode_mcu_AC_first` |
| 6.7 | `jpeg::jdcoefct::decompress_data` |
| 6.2 | `jpeg::jidctint_neon::jsimd_idct_islow_neon` |
| 3.7 | `jpeg::jdsample::sep_upsample` |
| 3.0 | `jpeg::jdhuff::jpeg_fill_bit_buffer` |
| 2.1 | `jpeg::jdcolext_neon::jsimd_ycc_rgb_convert_neon::<0, 1, 2, -1, 3>` |
| 1.7 | `jpeg::jdphuff::decode_mcu_DC_first` |
| 1.6 | `jpeg::jsimd::jsimd_h2v2_fancy_upsample` |
| 1.6 | `jpeg::jdsample_neon::jsimd_h2v2_fancy_upsample_neon` |
| 1.4 | `jpeg::jidctint_neon::jsimd_idct_islow_pass2_regular` |

## Stage shares (leaf-to-root first match)
| stage | % CPU | top self functions in stage |
| --- | ---: | --- |
| jpeg-decode | 98.8 | `jpeg::jdphuff::decode_mcu_AC_refine` 46.6; `libsystem_malloc.dylib+?` 16.6; `jpeg::jdphuff::decode_mcu_AC_first` 8.5; `jpeg::jdcoefct::consume_data` 8.1 |
| other | 1.2 | `bench::main` 1.0; `__bzero` 0.1; `write` 0.0; `<jpeg::decode::Scanlines>::read_rows` 0.0 |
