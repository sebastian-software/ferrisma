# prof/jpegc-mountain.json.gz: total CPU weight 1531880 (thread CPU delta units, us)

## Self (top 25)
| % | function |
| ---: | --- |
| 67.2 | `decode_mcu` |
| 14.7 | `jsimd_idct_islow_neon` |
| 6.4 | `main` |
| 5.6 | `jsimd_ycc_rgb_convert_neon` |
| 2.9 | `jsimd_h2v2_fancy_upsample_neon` |
| 2.2 | `decompress_onepass` |
| 0.3 | `__read_nocancel` |
| 0.3 | `_platform_memset` |
| 0.2 | `jsimd_idct_islow` |
| 0.1 | `libjpeg.8.3.2.dylib+?` |
| 0.1 | `__bzero` |
| 0.0 | `jpeg_CreateDecompress` |
| 0.0 | `libsystem_malloc.dylib+?` |
| 0.0 | `_platform_memmove` |
| 0.0 | `sep_upsample` |
| 0.0 | `__commpage_gettimeofday` |
| 0.0 | `next_marker` |
| 0.0 | `read_markers` |
| 0.0 | `jsimd_h2v2_fancy_upsample` |

## Inclusive (top 25)
| % | function |
| ---: | --- |
| 100.0 | `main` |
| 100.0 | `start` |
| 93.3 | `decode` |
| 93.2 | `process_data_context_main` |
| 93.2 | `jpeg_read_scanlines` |
| 84.7 | `decompress_onepass` |
| 67.2 | `decode_mcu` |
| 14.7 | `jsimd_idct_islow_neon` |
| 8.5 | `sep_upsample` |
| 5.6 | `jsimd_ycc_rgb_convert_neon` |
| 2.9 | `jsimd_h2v2_fancy_upsample_neon` |
| 0.3 | `ProbeReadInput` |
| 0.3 | `libsystem_c.dylib+?` |
| 0.3 | `__read_nocancel` |
| 0.3 | `_platform_memset` |
| 0.2 | `jsimd_idct_islow` |
| 0.1 | `libjpeg.8.3.2.dylib+?` |
| 0.1 | `__bzero` |
| 0.0 | `libsystem_malloc.dylib+?` |
| 0.0 | `jpeg_CreateDecompress` |
| 0.0 | `read_markers` |
| 0.0 | `consume_markers` |
| 0.0 | `_platform_memmove` |
| 0.0 | `__commpage_gettimeofday` |
| 0.0 | `next_marker` |

## Stage shares (leaf-to-root first match)
| stage | % CPU | top self functions in stage |
| --- | ---: | --- |
| other | 100.0 | `decode_mcu` 67.2; `jsimd_idct_islow_neon` 14.7; `main` 6.4; `jsimd_ycc_rgb_convert_neon` 5.6 |
