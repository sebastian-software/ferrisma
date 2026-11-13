# prof/f2-progressive.json.gz: total CPU weight 2747013 (thread CPU delta units, us)

## Self (top 25)
| % | function |
| ---: | --- |
| 58.6 | `jpeg::jdphuff::decode_mcu_AC_refine` |
| 10.3 | `jpeg::jdphuff::decode_mcu_AC_first` |
| 9.5 | `jpeg::jdcoefct::consume_data` |
| 7.0 | `jpeg::jidctint_neon::jsimd_idct_islow_neon` |
| 3.2 | `jpeg::jdhuff::jpeg_fill_bit_buffer` |
| 2.9 | `jpeg::jdcolext_neon::jsimd_ycc_rgb_convert_neon::<0, 1, 2, -1, 3>` |
| 1.8 | `jpeg::jdphuff::decode_mcu_DC_first` |
| 1.5 | `jpeg::jdsample_neon::jsimd_h2v2_fancy_upsample_neon` |
| 1.3 | `bench::main` |
| 1.1 | `jpeg::jdphuff::decode_mcu_DC_refine` |
| 0.8 | `jpeg::jmemmgr::alloc_barray` |
| 0.4 | `__bzero` |
| 0.3 | `jpeg::jdhuff::jpeg_huff_decode` |
| 0.3 | `_platform_memset` |
| 0.2 | `jpeg::jdcoefct::decompress_data` |
| 0.2 | `jpeg::jsimd::jsimd_idct_islow` |
| 0.1 | `read` |
| 0.1 | `libsystem_malloc.dylib+?` |
| 0.1 | `jpeg::jmemmgr::access_virt_barray` |
| 0.1 | `jpeg::jdsample::sep_upsample` |
| 0.0 | `jpeg::jdhuff::jpeg_make_d_derived_tbl` |
| 0.0 | `jpeg::jdmarker::read_markers` |
| 0.0 | `jpeg::jdmainct::process_data_context_main` |
| 0.0 | `jpeg::jsimd::jsimd_h2v2_fancy_upsample` |
| 0.0 | `<jpeg::decode::Scanlines>::read_rows` |

## Inclusive (top 25)
| % | function |
| ---: | --- |
| 100.0 | `bench::main` |
| 100.0 | `std::sys::backtrace::__rust_begin_short_backtrace::<fn() -> std::process::ExitCode, std::process::ExitCode>` |
| 100.0 | `std::rt::lang_start::<std::process::ExitCode>::{closure#0}` |
| 100.0 | `std::rt::lang_start_internal` |
| 100.0 | `main` |
| 100.0 | `start` |
| 98.5 | `bench::main::{closure#2}` |
| 86.4 | `jpeg::jdapistd::jpeg_start_decompress` |
| 86.4 | `<jpeg::decode::Decoder>::start` |
| 85.5 | `jpeg::jdcoefct::consume_data` |
| 60.5 | `jpeg::jdphuff::decode_mcu_AC_refine` |
| 12.0 | `<jpeg::decode::Scanlines>::read_rows` |
| 12.0 | `jpeg::jdmainct::process_data_context_main` |
| 12.0 | `jpeg::jdapistd::jpeg_read_scanlines` |
| 11.5 | `jpeg::jdphuff::decode_mcu_AC_first` |
| 7.4 | `jpeg::jdcoefct::decompress_data` |
| 7.0 | `jpeg::jidctint_neon::jsimd_idct_islow_neon` |
| 4.5 | `jpeg::jdsample::sep_upsample` |
| 3.2 | `jpeg::jdhuff::jpeg_fill_bit_buffer` |
| 2.9 | `jpeg::jdcolext_neon::jsimd_ycc_rgb_convert_neon::<0, 1, 2, -1, 3>` |
| 2.1 | `jpeg::jdphuff::decode_mcu_DC_first` |
| 1.5 | `jpeg::jsimd::jsimd_h2v2_fancy_upsample` |
| 1.5 | `jpeg::jdsample_neon::jsimd_h2v2_fancy_upsample_neon` |
| 1.2 | `jpeg::jdphuff::decode_mcu_DC_refine` |
| 0.8 | `jpeg::jdmaster::jinit_master_decompress` |

## Stage shares (leaf-to-root first match)
| stage | % CPU | top self functions in stage |
| --- | ---: | --- |
| jpeg-decode | 98.4 | `jpeg::jdphuff::decode_mcu_AC_refine` 58.6; `jpeg::jdphuff::decode_mcu_AC_first` 10.3; `jpeg::jdcoefct::consume_data` 9.5; `jpeg::jidctint_neon::jsimd_idct_islow_neon` 7.0 |
| other | 1.6 | `bench::main` 1.3; `read` 0.1; `__bzero` 0.1; `<jpeg::decode::Scanlines>::read_rows` 0.0 |
