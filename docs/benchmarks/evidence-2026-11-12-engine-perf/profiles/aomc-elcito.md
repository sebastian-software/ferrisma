# prof/aomc-elcito.json.gz: total CPU weight 1780809 (thread CPU delta units, us)

## Self (top 40)
| % | function |
| ---: | --- |
| 15.7 | `cost_and_tokenize_map` |
| 9.8 | `av1_optimize_txb` |
| 8.3 | `av1_k_means_dim2` |
| 5.1 | `highbd_quantize_fp_helper_c` |
| 4.0 | `aom_highbd_quantize_b_helper_c` |
| 3.8 | `av1_k_means_dim1` |
| 3.6 | `av1_calc_indices_dim2_neon` |
| 2.7 | `av1_cost_coeffs_txb` |
| 2.4 | `av1_count_colors_highbd` |
| 1.9 | `av1_predict_intra_block` |
| 1.8 | `av1_rd_pick_palette_intra_sby` |
| 1.7 | `highbd_fdct32_x4_neon` |
| 1.7 | `search_tx_type` |
| 1.4 | `av1_fwd_txfm2d_8x8_neon` |
| 1.3 | `idct32x32_neon` |
| 1.3 | `compute_gradient_info_sb` |
| 1.2 | `av1_calc_indices_dim1_neon` |
| 1.2 | `av1_get_txb_entropy_context` |
| 1.1 | `highbd_fdct16_x4_neon` |
| 1.1 | `_platform_memmove` |
| 1.1 | `inv_txfm2d_add_universe_neon` |
| 1.0 | `idct16x16_neon` |
| 0.7 | `sigprocmask` |
| 0.7 | `libsystem_malloc.dylib+?` |
| 0.7 | `av1_highbd_quantize_fp_neon` |
| 0.7 | `dist_block_tx_domain` |
| 0.6 | `avifImageRGBToYUV` |
| 0.6 | `av1_fwd_txfm2d_16x16_neon` |
| 0.6 | `av1_rd_pick_intra_sby_mode` |
| 0.6 | `idct8x8_neon` |
| 0.5 | `av1_rd_pick_palette_intra_sbuv` |
| 0.5 | `libaom.3.15.3.dylib+?` |
| 0.5 | `av1_txb_init_levels_neon` |
| 0.4 | `av1_update_and_record_txb_context` |
| 0.4 | `libsystem_c.dylib+?` |
| 0.4 | `aom_highbd_subtract_block_neon` |
| 0.4 | `av1_cost_tokens_from_cdf` |
| 0.4 | `__sigaltstack` |
| 0.4 | `av1_cnn_convolve_no_maxpool_padding_valid_neon` |
| 0.4 | `_platform_memcmp` |

## Inclusive (top 40)
| % | function |
| ---: | --- |
| 100.0 | `main` |
| 100.0 | `start` |
| 98.9 | `aomCodecEncodeImage` |
| 98.9 | `avifEncoderAddImageInternal` |
| 98.9 | `avifEncoderWrite` |
| 98.8 | `encoder_encode` |
| 98.8 | `aom_codec_encode` |
| 98.7 | `av1_encode_strategy` |
| 98.7 | `av1_get_compressed_data` |
| 98.3 | `av1_encode` |
| 98.3 | `encode_with_recode_loop_and_filter` |
| 95.8 | `encode_frame_internal` |
| 95.8 | `av1_encode_frame` |
| 95.8 | `av1_encode_sb_row` |
| 95.8 | `av1_encode_tile` |
| 93.9 | `av1_rd_pick_partition` |
| 82.2 | `pick_sb_modes` |
| 82.1 | `av1_rd_pick_intra_mode_sb` |
| 45.3 | `av1_rd_pick_intra_sby_mode` |
| 36.7 | `av1_rd_pick_intra_sbuv_mode` |
| 33.6 | `av1_txfm_rd_in_plane` |
| 33.2 | `block_rd_txfm` |
| 30.9 | `search_tx_type` |
| 24.8 | `av1_pick_uniform_tx_size_type_yrd` |
| 24.6 | `av1_rd_pick_palette_intra_sby` |
| 24.5 | `av1_rd_pick_palette_intra_sbuv` |
| 16.8 | `palette_rd_y` |
| 15.7 | `cost_and_tokenize_map` |
| 15.7 | `av1_cost_color_map` |
| 14.3 | `rd_pick_rect_partition` |
| 12.3 | `intra_block_yrd` |
| 11.9 | `av1_optimize_b` |
| 11.8 | `av1_optimize_txb` |
| 11.7 | `av1_k_means_dim2` |
| 10.5 | `av1_quant` |
| 9.4 | `intra_mode_info_cost_y` |
| 8.6 | `av1_encode_intra_block_plane` |
| 8.6 | `av1_foreach_transformed_block_in_plane` |
| 8.5 | `encode_block_intra_and_set_context` |
| 7.5 | `av1_txfm_uvrd` |

## Stage shares (leaf-to-root first match)
| stage | % CPU | top self functions in stage |
| --- | ---: | --- |
| aom-encode | 98.8 | `cost_and_tokenize_map` 15.7; `av1_optimize_txb` 9.8; `av1_k_means_dim2` 8.3; `highbd_quantize_fp_helper_c` 5.1 |
| rgb2yuv | 0.7 | `avifImageRGBToYUV` 0.6; `avifRoundf` 0.1 |
| other | 0.4 | `__read_nocancel` 0.3; `main` 0.1; `__close_nocancel` 0.0; `__bzero` 0.0 |
