# prof/c2-progressive.json.gz: total CPU weight 2177045 (thread CPU delta units, us)

## Self (top 25)
| % | function |
| ---: | --- |
| 64.1 | `decode_mcu_AC_refine` |
| 12.1 | `decode_mcu_AC_first` |
| 6.8 | `jsimd_idct_islow_neon` |
| 3.1 | `jsimd_ycc_rgb_convert_neon` |
| 2.9 | `jpeg_fill_bit_buffer` |
| 2.9 | `consume_data` |
| 2.0 | `decode_mcu_DC_first` |
| 1.6 | `main` |
| 1.5 | `jsimd_h2v2_fancy_upsample_neon` |
| 1.3 | `decode_mcu_DC_refine` |
| 0.4 | `__bzero` |
| 0.3 | `jpeg_huff_decode` |
| 0.3 | `_platform_memset` |
| 0.2 | `__read_nocancel` |
| 0.2 | `decompress_data` |
| 0.1 | `jsimd_idct_islow` |
| 0.0 | `sep_upsample` |
| 0.0 | `libsystem_malloc.dylib+?` |
| 0.0 | `jpeg_make_d_derived_tbl` |
| 0.0 | `access_virt_barray` |
| 0.0 | `emit_message` |
| 0.0 | `fullsize_upsample` |
| 0.0 | `bench+?` |
| 0.0 | `init_simd` |
| 0.0 | `start_input_pass` |

## Inclusive (top 25)
| % | function |
| ---: | --- |
| 100.0 | `main` |
| 100.0 | `start` |
| 98.2 | `decode` |
| 86.4 | `jpeg_start_decompress` |
| 86.3 | `consume_data` |
| 65.8 | `decode_mcu_AC_refine` |
| 13.3 | `decode_mcu_AC_first` |
| 11.8 | `process_data_context_main` |
| 11.8 | `jpeg_read_scanlines` |
| 7.1 | `decompress_data` |
| 6.8 | `jsimd_idct_islow_neon` |
| 4.7 | `sep_upsample` |
| 3.1 | `jsimd_ycc_rgb_convert_neon` |
| 2.9 | `jpeg_fill_bit_buffer` |
| 2.3 | `decode_mcu_DC_first` |
| 1.5 | `jsimd_h2v2_fancy_upsample_neon` |
| 1.3 | `decode_mcu_DC_refine` |
| 0.7 | `access_virt_barray` |
| 0.4 | `__bzero` |
| 0.3 | `jpeg_huff_decode` |
| 0.3 | `_platform_memset` |
| 0.2 | `__read_nocancel` |
| 0.2 | `libsystem_c.dylib+?` |
| 0.2 | `ProbeReadInput` |
| 0.1 | `jsimd_idct_islow` |

## Stage shares (leaf-to-root first match)
| stage | % CPU | top self functions in stage |
| --- | ---: | --- |
| other | 100.0 | `decode_mcu_AC_refine` 64.1; `decode_mcu_AC_first` 12.1; `jsimd_idct_islow_neon` 6.8; `jsimd_ycc_rgb_convert_neon` 3.1 |
