# prof/port-420.json.gz: total CPU weight 1008957 (thread CPU delta units, us)

## Self (top 25)
| % | function |
| ---: | --- |
| 58.7 | `jpeg::jdhuff::decode_mcu` |
| 11.2 | `jpeg::jidctint_neon::jsimd_idct_islow_neon` |
| 7.5 | `jpeg::jdcolext_neon::jsimd_ycc_rgb_convert_neon::<0, 1, 2, -1, 3>` |
| 5.2 | `jpeg::jidctint_neon::jsimd_idct_islow_pass2_regular` |
| 4.8 | `jpeg::jidctint_neon::jsimd_idct_islow_pass2_sparse` |
| 3.5 | `bench::main` |
| 3.4 | `jpeg::jdsample_neon::jsimd_h2v2_fancy_upsample_neon` |
| 2.8 | `jpeg::jdcoefct::decompress_onepass` |
| 0.6 | `_platform_memset` |
| 0.4 | `jpeg::jsimd::jsimd_idct_islow` |
| 0.4 | `read` |
| 0.4 | `__bzero` |
| 0.3 | `jpeg::jdsample::sep_upsample` |
| 0.3 | `libsystem_malloc.dylib+?` |
| 0.1 | `arm-simd-base+?` |
| 0.1 | `jpeg::jdsample::fullsize_upsample` |
| 0.0 | `jpeg::jdhuff::jpeg_make_d_derived_tbl` |
| 0.0 | `jpeg::jdmaster::jinit_master_decompress` |
| 0.0 | `write` |
| 0.0 | `jpeg::jsimd::jsimd_h2v2_fancy_upsample` |
| 0.0 | `__rustc::__rdl_alloc` |
| 0.0 | `<jpeg::jpeglib::RowListMut<u8>>::two_rows_mut` |
| 0.0 | `jpeg::jdhuff::jpeg_huff_decode` |
| 0.0 | `jpeg::jdmainct::post_input` |
| 0.0 | `ptr::drop_glue::<jpeg::jpeglib::jpeg_decompress_struct>` |

## Inclusive (top 25)
| % | function |
| ---: | --- |
| 100.0 | `bench::main` |
| 100.0 | `std::sys::backtrace::__rust_begin_short_backtrace::<fn() -> std::process::ExitCode, std::process::ExitCode>` |
| 100.0 | `std::rt::lang_start::<std::process::ExitCode>::{closure#0}` |
| 100.0 | `std::rt::lang_start_internal` |
| 100.0 | `main` |
| 100.0 | `start` |
| 96.1 | `bench::main::{closure#2}` |
| 95.7 | `jpeg::jdmainct::process_data_context_main` |
| 95.7 | `jpeg::jdapistd::jpeg_read_scanlines` |
| 95.7 | `<jpeg::decode::Scanlines>::read_rows` |
| 84.0 | `jpeg::jdcoefct::decompress_onepass` |
| 58.7 | `jpeg::jdhuff::decode_mcu` |
| 21.2 | `jpeg::jidctint_neon::jsimd_idct_islow_neon` |
| 11.4 | `jpeg::jdsample::sep_upsample` |
| 7.5 | `jpeg::jdcolext_neon::jsimd_ycc_rgb_convert_neon::<0, 1, 2, -1, 3>` |
| 5.2 | `jpeg::jidctint_neon::jsimd_idct_islow_pass2_regular` |
| 4.8 | `jpeg::jidctint_neon::jsimd_idct_islow_pass2_sparse` |
| 3.5 | `jpeg::jsimd::jsimd_h2v2_fancy_upsample` |
| 3.5 | `jpeg::jdsample_neon::jsimd_h2v2_fancy_upsample_neon` |
| 0.6 | `_platform_memset` |
| 0.4 | `jpeg::jsimd::jsimd_idct_islow` |
| 0.4 | `read` |
| 0.4 | `io::read::default_read_to_end::<std::fs::File>` |
| 0.4 | `std::fs::read::inner` |
| 0.4 | `__bzero` |

## Stage shares (leaf-to-root first match)
| stage | % CPU | top self functions in stage |
| --- | ---: | --- |
| jpeg-decode | 95.8 | `jpeg::jdhuff::decode_mcu` 58.7; `jpeg::jidctint_neon::jsimd_idct_islow_neon` 11.2; `jpeg::jdcolext_neon::jsimd_ycc_rgb_convert_neon::<0, 1, 2, -1, 3>` 7.5; `jpeg::jidctint_neon::jsimd_idct_islow_pass2_regular` 5.2 |
| other | 4.2 | `bench::main` 3.5; `read` 0.4; `__bzero` 0.2; `write` 0.0 |
