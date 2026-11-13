# prof/c-progressive.json.gz: total CPU weight 2682366 (thread CPU delta units, us)

## Self (top 25)
| % | function |
| ---: | --- |
| 63.2 | `decode_mcu_AC_refine` |
| 11.1 | `decode_mcu_AC_first` |
| 7.3 | `jsimd_idct_islow_neon` |
| 4.0 | `consume_data` |
| 3.5 | `jpeg_fill_bit_buffer` |
| 3.1 | `jsimd_ycc_rgb_convert_neon` |
| 2.1 | `decode_mcu_DC_first` |
| 1.5 | `main` |
| 1.3 | `jsimd_h2v2_fancy_upsample_neon` |
| 1.2 | `decode_mcu_DC_refine` |
| 0.3 | `jpeg_huff_decode` |
| 0.3 | `_platform_memset` |
| 0.3 | `__bzero` |
| 0.2 | `decompress_data` |
| 0.1 | `__open_nocancel` |
| 0.1 | `sep_upsample` |
| 0.0 | `jsimd_idct_islow` |
| 0.0 | `access_virt_barray` |
| 0.0 | `__read_nocancel` |
| 0.0 | `jpeg_make_d_derived_tbl` |
| 0.0 | `read_markers` |
| 0.0 | `jpeg_read_scanlines` |
| 0.0 | `decode` |
| 0.0 | `process_data_context_main` |
| 0.0 | `start_pass_phuff_decoder` |

## Inclusive (top 25)
| % | function |
| ---: | --- |
| 100.0 | `main` |
| 100.0 | `start` |
| 98.3 | `decode` |
| 86.2 | `jpeg_start_decompress` |
| 86.1 | `consume_data` |
| 65.4 | `decode_mcu_AC_refine` |
| 12.5 | `decode_mcu_AC_first` |
| 12.0 | `jpeg_read_scanlines` |
| 12.0 | `process_data_context_main` |
| 7.5 | `decompress_data` |
| 7.3 | `jsimd_idct_islow_neon` |
| 4.5 | `sep_upsample` |
| 3.5 | `jpeg_fill_bit_buffer` |
| 3.1 | `jsimd_ycc_rgb_convert_neon` |
| 2.3 | `decode_mcu_DC_first` |
| 1.3 | `jsimd_h2v2_fancy_upsample_neon` |
| 1.2 | `decode_mcu_DC_refine` |
| 0.7 | `access_virt_barray` |
| 0.3 | `jpeg_huff_decode` |
| 0.3 | `_platform_memset` |
| 0.3 | `__bzero` |
| 0.2 | `libsystem_c.dylib+?` |
| 0.2 | `ProbeReadInput` |
| 0.1 | `__open_nocancel` |
| 0.1 | `open$NOCANCEL` |

## Stage shares (leaf-to-root first match)
| stage | % CPU | top self functions in stage |
| --- | ---: | --- |
| other | 100.0 | `decode_mcu_AC_refine` 63.2; `decode_mcu_AC_first` 11.1; `jsimd_idct_islow_neon` 7.3; `consume_data` 4.0 |
