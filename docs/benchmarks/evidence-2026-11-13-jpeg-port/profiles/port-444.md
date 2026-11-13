# prof/port-444.json.gz: total CPU weight 1523873 (thread CPU delta units, us)

## Self (top 25)
| % | function |
| ---: | --- |
| 58.2 | `jpeg::jdhuff::decode_mcu` |
| 12.3 | `jpeg::jidctint_neon::jsimd_idct_islow_neon` |
| 9.5 | `jpeg::jidctint_neon::jsimd_idct_islow_pass2_sparse` |
| 5.8 | `jpeg::jdcoefct::decompress_onepass` |
| 5.4 | `jpeg::jdcolext_neon::jsimd_ycc_rgb_convert_neon::<0, 1, 2, -1, 3>` |
| 3.6 | `jpeg::jidctint_neon::jsimd_idct_islow_pass2_regular` |
| 2.3 | `bench::main` |
| 0.5 | `jpeg::jsimd::jsimd_idct_islow` |
| 0.4 | `jpeg::jdsample::sep_upsample` |
| 0.3 | `__bzero` |
| 0.3 | `_platform_memset` |
| 0.3 | `std::rt::lang_start_internal` |
| 0.3 | `jpeg::jdsample::fullsize_upsample` |
| 0.2 | `libsystem_malloc.dylib+?` |
| 0.1 | `mach_absolute_time` |
| 0.1 | `arm-simd-base+?` |
| 0.1 | `read` |
| 0.0 | `jpeg::jdmarker::read_markers` |
| 0.0 | `jpeg::jdmainct::process_data_simple_main` |
| 0.0 | `libsystem_c.dylib+?` |
| 0.0 | `jpeg::jmemmgr::alloc_small` |
| 0.0 | `OUTLINED_FUNCTION_29` |
| 0.0 | `<jpeg::jdatasrc::MemSourceMethods as jpeg::jpeglib::jpeg_source_methods>::init_source` |
| 0.0 | `<std::sys::sync::once_box::OnceBox<std::sys::pal::unix::sync::mutex::Mutex>>::initialize::<<std::sys::sync::mutex::pthread::Mutex>::get::{closure#0}>` |
| 0.0 | `jpeg::jmemmgr::jinit_memory_mgr` |

## Inclusive (top 25)
| % | function |
| ---: | --- |
| 100.0 | `std::rt::lang_start_internal` |
| 100.0 | `main` |
| 100.0 | `start` |
| 99.7 | `bench::main` |
| 99.7 | `std::sys::backtrace::__rust_begin_short_backtrace::<fn() -> std::process::ExitCode, std::process::ExitCode>` |
| 99.7 | `std::rt::lang_start::<std::process::ExitCode>::{closure#0}` |
| 97.3 | `bench::main::{closure#2}` |
| 96.9 | `jpeg::jdmainct::process_data_simple_main` |
| 96.9 | `jpeg::jdapistd::jpeg_read_scanlines` |
| 96.9 | `<jpeg::decode::Scanlines>::read_rows` |
| 90.5 | `jpeg::jdcoefct::decompress_onepass` |
| 58.2 | `jpeg::jdhuff::decode_mcu` |
| 25.4 | `jpeg::jidctint_neon::jsimd_idct_islow_neon` |
| 9.5 | `jpeg::jidctint_neon::jsimd_idct_islow_pass2_sparse` |
| 6.0 | `jpeg::jdsample::sep_upsample` |
| 5.4 | `jpeg::jdcolext_neon::jsimd_ycc_rgb_convert_neon::<0, 1, 2, -1, 3>` |
| 3.6 | `jpeg::jidctint_neon::jsimd_idct_islow_pass2_regular` |
| 0.5 | `jpeg::jsimd::jsimd_idct_islow` |
| 0.3 | `__bzero` |
| 0.3 | `_platform_memset` |
| 0.3 | `libsystem_malloc.dylib+?` |
| 0.3 | `jpeg::jdsample::fullsize_upsample` |
| 0.1 | `mach_absolute_time` |
| 0.1 | `<jpeg::decode::Decoder>::with_options` |
| 0.1 | `<jpeg::decode::Decoder>::new` |

## Stage shares (leaf-to-root first match)
| stage | % CPU | top self functions in stage |
| --- | ---: | --- |
| jpeg-decode | 97.0 | `jpeg::jdhuff::decode_mcu` 58.2; `jpeg::jidctint_neon::jsimd_idct_islow_neon` 12.3; `jpeg::jidctint_neon::jsimd_idct_islow_pass2_sparse` 9.5; `jpeg::jdcoefct::decompress_onepass` 5.8 |
| other | 3.0 | `bench::main` 2.3; `std::rt::lang_start_internal` 0.3; `__bzero` 0.2; `read` 0.1 |
