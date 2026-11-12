## Per-thread CPU (top 6 threads)
| thread | % CPU of process | top self functions |
| --- | ---: | --- |
| ferrisma-base | 15.0 | `jpeg::jdhuff::decode_mcu` 27%; `jpeg::jdhuff::HUFF_DECODE_FAST` 13%; `avifImageRGBToYUV` 8%; `__psynch_cvwait` 4%; `jpeg::jidctint_neon::jsimd_idct_islow_neon` 3% |
| aom enc worker | 10.1 | `av1_optimize_txb` 18%; `aom::av1__encoder__encodeframe::highbd_compute_gradient_info` 18%; `_platform_memset` 10%; `av1_highbd_quantize_fp_facade` 10%; `<u8 as c2rust_bitfields::FieldType>::get_field` 3% |
| aom enc worker | 6.9 | `_platform_memset` 20%; `av1_optimize_txb` 15%; `__psynch_cvwait` 10%; `av1_highbd_quantize_fp_facade` 8%; `<u8 as c2rust_bitfields::FieldType>::get_field` 7% |
| aom enc worker | 5.5 | `av1_optimize_txb` 18%; `__psynch_cvwait` 14%; `av1_highbd_quantize_fp_facade` 7%; `_platform_memset` 6%; `av1_rd_pick_partition` 6% |
| aom enc worker | 4.9 | `av1_optimize_txb` 22%; `__psynch_cvwait` 20%; `libsystem_malloc.dylib+?` 16%; `_platform_memset` 8%; `av1_highbd_quantize_fp_facade` 7% |
| aom enc worker | 4.4 | `av1_optimize_txb` 16%; `__psynch_cvwait` 11%; `aom::av1__encoder__tx_search::search_tx_type` 10%; `av1_highbd_quantize_fp_facade` 9%; `_platform_memmove` 6% |

threads with samples: 40
