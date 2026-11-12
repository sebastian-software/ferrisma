# prof/jpegc-beach.json.gz: total CPU weight 1425623 (thread CPU delta units, us)

## Self (top 25)
| % | function |
| ---: | --- |
| 44.3 | `decode_mcu` |
| 20.0 | `jsimd_idct_islow_neon` |
| 17.2 | `main` |
| 5.9 | `jpeg_fill_bit_buffer` |
| 5.9 | `jsimd_ycc_rgb_convert_neon` |
| 3.7 | `decompress_onepass` |
| 1.2 | `jpeg_huff_decode` |
| 0.6 | `jsimd_idct_islow` |
| 0.4 | `_platform_memset` |
| 0.4 | `__read_nocancel` |
| 0.1 | `__bzero` |
| 0.1 | `libjpeg.8.3.2.dylib+?` |
| 0.1 | `process_data_simple_main` |
| 0.0 | `jzero_far` |
| 0.0 | `pthread_mutex_unlock` |
| 0.0 | `_platform_memmove` |
| 0.0 | `fstat` |
| 0.0 | `fullsize_upsample` |
| 0.0 | `jinit_input_controller` |
| 0.0 | `jpeg_read_scanlines` |
| 0.0 | `sep_upsample` |
| 0.0 | `read_markers` |
| 0.0 | `jinit_upsampler` |

## Inclusive (top 25)
| % | function |
| ---: | --- |
| 100.0 | `main` |
| 100.0 | `start` |
| 82.4 | `decode` |
| 82.3 | `jpeg_read_scanlines` |
| 82.3 | `process_data_simple_main` |
| 76.4 | `decompress_onepass` |
| 51.4 | `decode_mcu` |
| 20.0 | `jsimd_idct_islow_neon` |
| 5.9 | `sep_upsample` |
| 5.9 | `jpeg_fill_bit_buffer` |
| 5.9 | `jsimd_ycc_rgb_convert_neon` |
| 1.2 | `jpeg_huff_decode` |
| 0.6 | `jsimd_idct_islow` |
| 0.4 | `_platform_memset` |
| 0.4 | `ProbeReadInput` |
| 0.4 | `libsystem_c.dylib+?` |
| 0.4 | `__read_nocancel` |
| 0.1 | `__bzero` |
| 0.1 | `libjpeg.8.3.2.dylib+?` |
| 0.0 | `jzero_far` |
| 0.0 | `pthread_mutex_unlock` |
| 0.0 | `_platform_memmove` |
| 0.0 | `libsystem_malloc.dylib+?` |
| 0.0 | `fstat` |
| 0.0 | `fullsize_upsample` |

## Stage shares (leaf-to-root first match)
| stage | % CPU | top self functions in stage |
| --- | ---: | --- |
| other | 100.0 | `decode_mcu` 44.3; `jsimd_idct_islow_neon` 20.0; `main` 17.2; `jpeg_fill_bit_buffer` 5.9 |
