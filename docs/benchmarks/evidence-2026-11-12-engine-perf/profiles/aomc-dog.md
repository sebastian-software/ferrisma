# prof/aomc-dog.json.gz: total CPU weight 1502405 (thread CPU delta units, us)

## Self (top 40)
| % | function |
| ---: | --- |
| 11.9 | `highbd_quantize_fp_helper_c` |
| 11.9 | `av1_optimize_txb` |
| 3.7 | `av1_predict_intra_block` |
| 3.0 | `compute_gradient_info_sb` |
| 3.0 | `search_tx_type` |
| 2.7 | `av1_fwd_txfm2d_8x8_neon` |
| 2.3 | `highbd_fdct16_x4_neon` |
| 2.2 | `dist_block_tx_domain` |
| 2.2 | `aom_highbd_quantize_b_helper_c` |
| 2.0 | `_platform_memmove` |
| 1.9 | `av1_get_txb_entropy_context` |
| 1.7 | `inv_txfm2d_add_universe_neon` |
| 1.6 | `idct16x16_neon` |
| 1.5 | `avifImageRGBToYUV` |
| 1.5 | `av1_rd_pick_intra_sby_mode` |
| 1.4 | `av1_cost_coeffs_txb` |
| 1.2 | `av1_highbd_quantize_fp_neon` |
| 1.1 | `sigprocmask` |
| 1.1 | `av1_fwd_txfm2d_16x16_neon` |
| 1.1 | `cdef_filter_16_0_neon` |
| 1.0 | `libsystem_malloc.dylib+?` |
| 0.9 | `prune_intra_mode_with_hog` |
| 0.8 | `av1_nn_predict_neon` |
| 0.7 | `highbd_fdct8_row_rect_many_neon` |
| 0.7 | `av1_cost_tokens_from_cdf` |
| 0.7 | `idct8x8_new_neon` |
| 0.7 | `idct8x8_neon` |
| 0.7 | `av1_cnn_convolve_no_maxpool_padding_valid_neon` |
| 0.6 | `highbd_fdct8_col_many_neon` |
| 0.6 | `highbd_fdct16_row_rect_many_neon` |
| 0.6 | `libaom.3.15.3.dylib+?` |
| 0.6 | `aom_highbd_subtract_block_neon` |
| 0.6 | `__sigaltstack` |
| 0.6 | `iadst16x16_neon` |
| 0.6 | `_platform_memset` |
| 0.6 | `av1_foreach_transformed_block_in_plane` |
| 0.6 | `block_rd_txfm` |
| 0.6 | `av1_txb_init_levels_neon` |
| 0.5 | `get_txb_ctx` |
| 0.5 | `update_coeff_general` |

## Inclusive (top 40)
| % | function |
| ---: | --- |
| 100.0 | `main` |
| 100.0 | `start` |
| 97.7 | `avifEncoderWrite` |
| 97.7 | `avifEncoderAddImageInternal` |
| 97.7 | `aomCodecEncodeImage` |
| 97.6 | `encoder_encode` |
| 97.6 | `aom_codec_encode` |
| 97.2 | `av1_encode_strategy` |
| 97.2 | `av1_get_compressed_data` |
| 96.6 | `av1_encode` |
| 96.5 | `encode_with_recode_loop_and_filter` |
| 91.7 | `encode_frame_internal` |
| 91.7 | `av1_encode_frame` |
| 91.7 | `av1_encode_sb_row` |
| 91.7 | `av1_encode_tile` |
| 87.7 | `av1_rd_pick_partition` |
| 66.0 | `pick_sb_modes` |
| 65.6 | `av1_rd_pick_intra_mode_sb` |
| 48.0 | `av1_txfm_rd_in_plane` |
| 47.3 | `av1_rd_pick_intra_sby_mode` |
| 47.2 | `block_rd_txfm` |
| 43.1 | `search_tx_type` |
| 39.9 | `av1_pick_uniform_tx_size_type_yrd` |
| 27.9 | `intra_block_yrd` |
| 26.3 | `rd_pick_rect_partition` |
| 18.2 | `av1_rd_pick_intra_sbuv_mode` |
| 16.9 | `av1_foreach_transformed_block_in_plane` |
| 16.1 | `av1_quant` |
| 15.9 | `av1_optimize_b` |
| 15.7 | `av1_encode_intra_block_plane` |
| 15.5 | `av1_optimize_txb` |
| 15.1 | `encode_block_intra_and_set_context` |
| 14.6 | `encode_superblock` |
| 13.4 | `av1_highbd_quantize_fp_facade` |
| 13.3 | `encode_sb` |
| 13.3 | `encode_b` |
| 11.9 | `highbd_quantize_fp_helper_c` |
| 8.5 | `intra_model_rd` |
| 7.5 | `av1_predict_intra_block_facade` |
| 7.3 | `av1_inverse_transform_block` |

## Stage shares (leaf-to-root first match)
| stage | % CPU | top self functions in stage |
| --- | ---: | --- |
| aom-encode | 97.6 | `highbd_quantize_fp_helper_c` 11.9; `av1_optimize_txb` 11.9; `av1_predict_intra_block` 3.7; `compute_gradient_info_sb` 3.0 |
| rgb2yuv | 1.7 | `avifImageRGBToYUV` 1.5; `avifRoundf` 0.2 |
| other | 0.7 | `__read_nocancel` 0.3; `main` 0.3; `__bzero` 0.0; `libsystem_malloc.dylib+?` 0.0 |
