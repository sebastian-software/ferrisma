# prof/webpenc-c.json.gz: total CPU weight 1874368 (thread CPU delta units, us)

## Self (top 30)
| % | function |
| ---: | --- |
| 11.6 | `GetResidualCost_NEON` |
| 9.9 | `FTransform_NEON` |
| 8.5 | `VP8Decimate` |
| 7.5 | `libz.1.dylib+?` |
| 7.5 | `VP8PutBit` |
| 6.3 | `ITransformOne_NEON` |
| 6.1 | `CollectHistogram_NEON` |
| 5.9 | `Disto4x4_NEON` |
| 5.8 | `ReconstructIntra4` |
| 4.2 | `QuantizeBlock_NEON` |
| 3.9 | `png_read_filter_row_paeth3_neon` |
| 3.4 | `VP8RecordCoeffTokens` |
| 1.4 | `ITransform_NEON` |
| 1.4 | `VP8SetHistogramData` |
| 1.3 | `VP8EmitTokens` |
| 1.3 | `_platform_memmove` |
| 1.2 | `AccumulateRGB` |
| 1.2 | `SSE4x4_NEON` |
| 1.0 | `Flush` |
| 0.9 | `IntraChromaPreds_C` |
| 0.7 | `ReconstructIntra16` |
| 0.7 | `png_read_filter_row_avg3_neon` |
| 0.7 | `QuantizeSingle` |
| 0.6 | `VP8GetCostLuma16` |
| 0.5 | `VP8GetCostLuma4` |
| 0.5 | `FTransform2_C` |
| 0.4 | `Quantize2Blocks_NEON` |
| 0.4 | `VP8IteratorRotateI4` |
| 0.4 | `ImportBlock` |
| 0.4 | `SetResidualCoeffs_NEON` |

## Inclusive (top 30)
| % | function |
| ---: | --- |
| 100.0 | `main` |
| 100.0 | `start` |
| 85.1 | `WebPEncode` |
| 73.0 | `VP8EncTokenLoop` |
| 60.0 | `VP8Decimate` |
| 19.8 | `ReconstructIntra4` |
| 14.9 | `ReadPicture` |
| 14.6 | `ReadPNG` |
| 12.6 | `png_read_row` |
| 12.6 | `png_read_rows` |
| 11.6 | `GetResidualCost_NEON` |
| 10.5 | `DoSegmentsJob` |
| 10.5 | `Execute` |
| 10.5 | `VP8EncAnalyze` |
| 9.9 | `FTransform_NEON` |
| 9.1 | `CollectHistogram_NEON` |
| 8.6 | `VP8EmitTokens` |
| 8.6 | `VP8PutBit` |
| 7.6 | `png_read_IDAT_data` |
| 7.6 | `VP8GetCostLuma4` |
| 7.6 | `libz.1.dylib+?` |
| 6.4 | `ITransform_NEON` |
| 6.3 | `ITransformOne_NEON` |
| 5.9 | `Disto4x4_NEON` |
| 5.6 | `ReconstructIntra16` |
| 4.2 | `QuantizeBlock_NEON` |
| 4.0 | `VP8GetCostLuma16` |
| 3.9 | `png_read_filter_row_paeth3_neon` |
| 3.4 | `VP8RecordCoeffTokens` |
| 3.1 | `ReconstructUV` |

## Stage shares (leaf-to-root first match)
| stage | % CPU | top self functions in stage |
| --- | ---: | --- |
| webp-decode | 85.1 | `GetResidualCost_NEON` 11.6; `FTransform_NEON` 9.9; `VP8Decimate` 8.5; `VP8PutBit` 7.5 |
| other | 13.2 | `libz.1.dylib+?` 7.5; `png_read_filter_row_paeth3_neon` 3.9; `png_read_filter_row_avg3_neon` 0.7; `_platform_memmove` 0.5 |
| rgb2yuv | 1.7 | `AccumulateRGB` 1.2; `ConvertRGB24ToY_NEON` 0.4; `ConvertRGBA32ToUV_NEON` 0.2 |
