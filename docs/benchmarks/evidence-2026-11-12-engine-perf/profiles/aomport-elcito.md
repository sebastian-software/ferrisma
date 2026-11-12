# prof/aomport-elcito.json.gz: total CPU weight 2982086 (thread CPU delta units, us)

## Self (top 40)
| % | function |
| ---: | --- |
| 24.1 | `av1_k_means_dim2` |
| 11.2 | `av1_k_means_dim1` |
| 8.9 | `aom::av1__encoder__tokenize::cost_and_tokenize_map` |
| 7.0 | `av1_optimize_txb` |
| 4.8 | `_platform_memset` |
| 3.5 | `av1_rd_pick_palette_intra_sbuv` |
| 3.4 | `aom::av1__encoder__palette::palette_rd_y` |
| 3.3 | `av1_highbd_quantize_fp_facade` |
| 2.6 | `av1_cnn_convolve_no_maxpool_padding_valid_c` |
| 2.1 | `aom_highbd_quantize_b_helper_c` |
| 1.5 | `av1_cost_coeffs_txb` |
| 1.4 | `<u8 as c2rust_bitfields::FieldType>::get_field` |
| 1.3 | `aom::av1__encoder__tx_search::search_tx_type` |
| 1.0 | `aom::av1__encoder__encodeframe::highbd_compute_gradient_info_sb` |
| 1.0 | `_platform_memmove` |
| 0.9 | `aom::simd::av1__common__arm__highbd_inv_txfm_neon::inv_txfm2d_add_universe_neon` |
| 0.9 | `av1_predict_intra_block` |
| 0.9 | `av1_get_nz_map_contexts_c` |
| 0.7 | `av1_rd_pick_palette_intra_sby` |
| 0.7 | `aom::simd::av1__common__arm__highbd_inv_txfm_neon::idct32x32_neon` |
| 0.7 | `av1_count_colors_highbd` |
| 0.7 | `aom::simd::av1__common__arm__highbd_inv_txfm_neon::idct16x16_neon` |
| 0.6 | `aom::simd::av1__encoder__arm__highbd_fwd_txfm_neon::highbd_fdct32_x4_neon` |
| 0.5 | `cfl_predict_hbd_8x8_c` |
| 0.5 | `av1_update_and_record_txb_context` |
| 0.5 | `libsystem_malloc.dylib+?` |
| 0.5 | `av1_write_coeffs_txb` |
| 0.4 | `aom::simd::av1__encoder__arm__highbd_fwd_txfm_neon::highbd_fdct16_x4_neon` |
| 0.4 | `avifImageRGBToYUV` |
| 0.4 | `aom::av1__encoder__tx_search::dist_block_tx_domain` |
| 0.4 | `__bzero` |
| 0.4 | `av1_rd_pick_intra_sby_mode` |
| 0.4 | `av1_fwd_txfm2d_8x8_neon` |
| 0.4 | `av1_highbd_quantize_fp_neon` |
| 0.3 | `aom::simd::av1__common__arm__highbd_inv_txfm_neon::idct8x8_neon` |
| 0.3 | `av1_quant` |
| 0.3 | `libsystem_c.dylib+?` |
| 0.3 | `av1_nn_predict_c` |
| 0.3 | `encode_still_port+?` |
| 0.3 | `aom_highbd_hadamard_8x8_c` |

## Inclusive (top 40)
| % | function |
| ---: | --- |
| 100.0 | `encode_still::main` |
| 100.0 | `std::sys::backtrace::__rust_begin_short_backtrace::<fn(), ()>` |
| 100.0 | `std::rt::lang_start::<()>::{closure#0}` |
| 100.0 | `std::rt::lang_start_internal` |
| 100.0 | `main` |
| 100.0 | `start` |
| 99.2 | `avif::codec_aom::aomCodecEncodeImage` |
| 99.2 | `avif::write::avifEncoderAddImageInternal` |
| 99.2 | `avifEncoderWrite` |
| 99.2 | `aom::av1__av1_cx_iface::encoder_encode` |
| 99.2 | `aom_codec_encode` |
| 99.2 | `aom::jump::value_scope::<u32, aom::av1__av1_cx_iface::encoder_encode::{closure#0}>` |
| 99.1 | `av1_encode_strategy` |
| 99.1 | `aom::jump::value_scope::<i32, aom::av1__encoder__encoder::av1_get_compressed_data::{closure#0}>` |
| 98.9 | `av1_encode` |
| 98.9 | `aom::av1__encoder__encoder::encode_with_recode_loop_and_filter` |
| 96.8 | `aom::av1__encoder__encodeframe::encode_frame_internal` |
| 96.8 | `av1_encode_frame` |
| 96.8 | `av1_encode_sb_row` |
| 96.8 | `av1_encode_tile` |
| 95.5 | `av1_rd_pick_partition` |
| 85.6 | `aom::av1__encoder__partition_search::pick_sb_modes` |
| 85.4 | `av1_rd_pick_intra_mode_sb` |
| 43.8 | `av1_rd_pick_intra_sbuv_mode` |
| 41.6 | `av1_rd_pick_intra_sby_mode` |
| 35.1 | `av1_rd_pick_palette_intra_sbuv` |
| 27.3 | `av1_rd_pick_palette_intra_sby` |
| 25.4 | `av1_txfm_rd_in_plane` |
| 25.2 | `aom::av1__encoder__tx_search::block_rd_txfm` |
| 24.4 | `av1_k_means_dim2` |
| 23.9 | `aom::av1__encoder__tx_search::search_tx_type` |
| 18.8 | `aom::av1__encoder__palette::perform_k_means_palette_search` |
| 18.4 | `av1_pick_uniform_tx_size_type_yrd` |
| 14.4 | `aom::av1__encoder__palette::palette_rd_y` |
| 14.1 | `aom::av1__encoder__partition_search::rd_pick_rect_partition` |
| 11.4 | `av1_k_means_dim1` |
| 8.9 | `aom::av1__encoder__tokenize::cost_and_tokenize_map` |
| 8.5 | `aom::av1__encoder__intra_mode_search::intra_block_yrd` |
| 8.1 | `av1_optimize_b` |
| 8.0 | `av1_optimize_txb` |

## Stage shares (leaf-to-root first match)
| stage | % CPU | top self functions in stage |
| --- | ---: | --- |
| aom-encode | 99.2 | `av1_k_means_dim2` 24.1; `av1_k_means_dim1` 11.2; `aom::av1__encoder__tokenize::cost_and_tokenize_map` 8.9; `av1_optimize_txb` 7.0 |
| rgb2yuv | 0.5 | `avifImageRGBToYUV` 0.4; `encode_still_port+?` 0.1; `libsystem_m.dylib+?` 0.0 |
| other | 0.3 | `encode_still::main` 0.1; `read` 0.1; `__bzero` 0.0; `__open` 0.0 |
