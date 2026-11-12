# prof/aomport-dog.json.gz: total CPU weight 1954581 (thread CPU delta units, us)

## Self (top 40)
| % | function |
| ---: | --- |
| 11.4 | `av1_optimize_txb` |
| 9.2 | `_platform_memset` |
| 8.4 | `av1_cnn_convolve_no_maxpool_padding_valid_c` |
| 7.7 | `av1_highbd_quantize_fp_facade` |
| 3.2 | `<u8 as c2rust_bitfields::FieldType>::get_field` |
| 3.1 | `aom::av1__encoder__tx_search::search_tx_type` |
| 3.0 | `aom::av1__encoder__encodeframe::highbd_compute_gradient_info_sb` |
| 2.4 | `av1_predict_intra_block` |
| 2.2 | `_platform_memmove` |
| 1.8 | `aom::av1__encoder__tx_search::dist_block_tx_domain` |
| 1.5 | `aom::simd::av1__common__arm__highbd_inv_txfm_neon::inv_txfm2d_add_universe_neon` |
| 1.5 | `aom_highbd_quantize_b_helper_c` |
| 1.4 | `aom::simd::av1__encoder__arm__highbd_fwd_txfm_neon::highbd_fdct16_x4_neon` |
| 1.3 | `aom_highbd_hadamard_8x8_c` |
| 1.2 | `aom::simd::av1__common__arm__highbd_inv_txfm_neon::idct16x16_neon` |
| 1.2 | `av1_rd_pick_intra_sby_mode` |
| 1.2 | `av1_cost_coeffs_txb` |
| 1.1 | `av1_fwd_txfm2d_8x8_neon` |
| 1.0 | `encode_still_port+?` |
| 1.0 | `avifImageRGBToYUV` |
| 1.0 | `libsystem_malloc.dylib+?` |
| 0.9 | `av1_nn_predict_c` |
| 0.8 | `cfl_predict_hbd_8x8_c` |
| 0.8 | `aom::av1__encoder__pickcdef::compute_cdef_dist_highbd` |
| 0.8 | `av1_get_nz_map_contexts_c` |
| 0.7 | `av1_update_and_record_txb_context` |
| 0.7 | `cdef_filter_16_0_neon` |
| 0.7 | `av1_highbd_quantize_fp_neon` |
| 0.7 | `cfl_predict_hbd_8x16_c` |
| 0.7 | `aom::av1__encoder__intra_mode_search::prune_intra_mode_with_hog` |
| 0.7 | `aom::simd::av1__common__arm__highbd_inv_txfm_neon::idct8x8_neon` |
| 0.6 | `aom_highbd_subtract_block_neon` |
| 0.6 | `aom::av1__encoder__intra_mode_search::intra_model_rd` |
| 0.6 | `av1_predict_intra_block_facade` |
| 0.5 | `av1_write_coeffs_txb` |
| 0.5 | `aom::av1__encoder__txb_rdopt::get_coeff_cost_general` |
| 0.5 | `__bzero` |
| 0.5 | `cdef_filter_16_2_neon` |
| 0.5 | `av1_rd_pick_partition` |
| 0.5 | `aom::av1__encoder__tx_search::get_tx_mask` |

## Inclusive (top 40)
| % | function |
| ---: | --- |
| 100.0 | `start` |
| 99.8 | `std::rt::lang_start_internal` |
| 99.8 | `main` |
| 99.8 | `encode_still::main` |
| 99.8 | `std::sys::backtrace::__rust_begin_short_backtrace::<fn(), ()>` |
| 99.8 | `std::rt::lang_start::<()>::{closure#0}` |
| 97.8 | `avif::write::avifEncoderAddImageInternal` |
| 97.8 | `avifEncoderWrite` |
| 97.8 | `avif::codec_aom::aomCodecEncodeImage` |
| 97.7 | `aom::av1__av1_cx_iface::encoder_encode` |
| 97.7 | `aom_codec_encode` |
| 97.5 | `aom::jump::value_scope::<u32, aom::av1__av1_cx_iface::encoder_encode::{closure#0}>` |
| 97.4 | `av1_encode_strategy` |
| 97.4 | `aom::jump::value_scope::<i32, aom::av1__encoder__encoder::av1_get_compressed_data::{closure#0}>` |
| 97.0 | `av1_encode` |
| 97.0 | `aom::av1__encoder__encoder::encode_with_recode_loop_and_filter` |
| 91.7 | `av1_encode_frame` |
| 91.6 | `aom::av1__encoder__encodeframe::encode_frame_internal` |
| 91.6 | `av1_encode_sb_row` |
| 91.6 | `av1_encode_tile` |
| 87.7 | `av1_rd_pick_partition` |
| 63.0 | `aom::av1__encoder__partition_search::pick_sb_modes` |
| 62.7 | `av1_rd_pick_intra_mode_sb` |
| 46.5 | `av1_txfm_rd_in_plane` |
| 45.8 | `aom::av1__encoder__tx_search::block_rd_txfm` |
| 42.7 | `av1_rd_pick_intra_sby_mode` |
| 42.7 | `aom::av1__encoder__tx_search::search_tx_type` |
| 36.4 | `av1_pick_uniform_tx_size_type_yrd` |
| 26.7 | `aom::av1__encoder__partition_search::rd_pick_rect_partition` |
| 23.1 | `aom::av1__encoder__intra_mode_search::intra_block_yrd` |
| 19.9 | `av1_rd_pick_intra_sbuv_mode` |
| 14.4 | `av1_foreach_transformed_block_in_plane` |
| 14.1 | `av1_optimize_b` |
| 13.8 | `av1_optimize_txb` |
| 13.4 | `av1_encode_intra_block_plane` |
| 12.9 | `aom::av1__encoder__encodemb::encode_block_intra_and_set_context` |
| 11.7 | `aom::av1__encoder__partition_search::encode_superblock` |
| 11.6 | `av1_quant` |
| 11.3 | `aom::av1__encoder__partition_search::encode_sb` |
| 11.2 | `aom::av1__encoder__partition_search::encode_b` |

## Stage shares (leaf-to-root first match)
| stage | % CPU | top self functions in stage |
| --- | ---: | --- |
| aom-encode | 97.7 | `av1_optimize_txb` 11.4; `_platform_memset` 9.2; `av1_cnn_convolve_no_maxpool_padding_valid_c` 8.4; `av1_highbd_quantize_fp_facade` 7.7 |
| rgb2yuv | 1.4 | `avifImageRGBToYUV` 1.0; `libsystem_m.dylib+?` 0.2; `encode_still_port+?` 0.2 |
| other | 0.8 | `encode_still::main` 0.4; `aom::av1__encoder__partition_strategy::run_static_initializers` 0.2; `read` 0.1; `avif::write::avifEncoderAddImageInternal` 0.0 |
| av1-decode | 0.0 | `dav1d::common::intra_edge::run_static_initializers` 0.0; `dav1d::b16::ipred_prepare_tmpl::run_static_initializers` 0.0; `dav1d::b8::ipred_prepare_tmpl::run_static_initializers` 0.0 |
