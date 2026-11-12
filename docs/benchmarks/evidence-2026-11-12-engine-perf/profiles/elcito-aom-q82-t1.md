# prof/elcito-aom-q82-t1.json.gz: total CPU weight 6825367 (thread CPU delta units, us)

## Self (top 40)
| % | function |
| ---: | --- |
| 19.6 | `av1_k_means_dim2` |
| 8.1 | `av1_k_means_dim1` |
| 6.0 | `aom::av1__encoder__tokenize::cost_and_tokenize_map` |
| 5.9 | `av1_optimize_txb` |
| 4.9 | `jpeg::jdhuff::decode_mcu` |
| 4.1 | `ferrisma_kernels::dispatch::neon::horizontal_rgba` |
| 4.0 | `_platform_memset` |
| 3.2 | `jpeg::jdhuff::HUFF_DECODE_FAST` |
| 2.9 | `av1_highbd_quantize_fp_facade` |
| 2.4 | `av1_cnn_convolve_no_maxpool_padding_valid_c` |
| 2.3 | `av1_rd_pick_palette_intra_sbuv` |
| 2.3 | `aom::av1__encoder__palette::palette_rd_y` |
| 1.5 | `aom::av1__encoder__tx_search::search_tx_type` |
| 1.3 | `aom_highbd_quantize_b_helper_c` |
| 1.2 | `<u8 as c2rust_bitfields::FieldType>::get_field` |
| 1.2 | `ferrisma::fast_execute::source_row_to_linear::<3>` |
| 1.1 | `av1_cost_coeffs_txb` |
| 1.1 | `av1_predict_intra_block` |
| 0.8 | `aom::simd::av1__encoder__arm__highbd_fwd_txfm_neon::highbd_fdct32_x4_neon` |
| 0.8 | `jpeg::jidctint_neon::jsimd_idct_islow_neon` |
| 0.8 | `_platform_memmove` |
| 0.8 | `jpeg::jidctint_neon::jsimd_idct_islow_pass2_regular` |
| 0.8 | `aom::simd::av1__common__arm__highbd_inv_txfm_neon::idct32x32_neon` |
| 0.7 | `av1_get_nz_map_contexts_c` |
| 0.7 | `aom::av1__encoder__encodeframe::highbd_compute_gradient_info_sb` |
| 0.6 | `aom::simd::av1__common__arm__highbd_inv_txfm_neon::inv_txfm2d_add_universe_neon` |
| 0.6 | `avifImageRGBToYUV` |
| 0.6 | `ferrisma_kernels::dispatch::neon::axpy` |
| 0.6 | `av1_highbd_quantize_fp_neon` |
| 0.6 | `aom::simd::av1__encoder__arm__highbd_fwd_txfm_neon::highbd_fdct16_x4_neon` |
| 0.5 | `libsystem_malloc.dylib+?` |
| 0.5 | `av1_rd_pick_palette_intra_sby` |
| 0.5 | `av1_count_colors_highbd` |
| 0.5 | `cfl_predict_hbd_8x8_c` |
| 0.5 | `aom::simd::av1__common__arm__highbd_inv_txfm_neon::idct16x16_neon` |
| 0.4 | `av1_nn_predict_c` |
| 0.4 | `jpeg::jdcolext_neon::jsimd_ycc_rgb_convert_neon::<0, 1, 2, -1, 3>` |
| 0.4 | `__bzero` |
| 0.4 | `aom::simd::av1__common__arm__highbd_inv_txfm_neon::idct8x8_neon` |
| 0.3 | `av1_fwd_txfm2d_8x8_neon` |

## Inclusive (top 40)
| % | function |
| ---: | --- |
| 100.0 | `ferrisma::cli::run` |
| 100.0 | `ferrisma::main` |
| 100.0 | `std::sys::backtrace::__rust_begin_short_backtrace::<fn() -> result::Result<std::process::ExitCode, boxed::Box<dyn error::Error>>, result::Result<std::process::ExitCode, boxed::Box<dyn error::Error>>>` |
| 100.0 | `std::rt::lang_start::<result::Result<..>>::{closure#0}` |
| 100.0 | `std::rt::lang_start_internal` |
| 100.0 | `main` |
| 100.0 | `start` |
| 99.8 | `ferrisma::pipeline::run` |
| 82.4 | `<ferrisma_avif_write::Frame>::finish` |
| 82.4 | `<ferrisma::own_avif::Frame as ferrisma::encoder_backend::RowEncoder>::finish` |
| 82.4 | `<ferrisma::pipeline::Sink>::finish` |
| 81.7 | `aom::av1__av1_cx_iface::encoder_encode` |
| 81.7 | `aom_codec_encode` |
| 81.7 | `avif::codec_aom::aomCodecEncodeImage` |
| 81.7 | `avif::write::avifEncoderAddImageInternal` |
| 81.7 | `avifEncoderWrite` |
| 81.6 | `aom::jump::value_scope::<u32, aom::av1__av1_cx_iface::encoder_encode::{closure#0}>` |
| 81.6 | `av1_encode_strategy` |
| 81.6 | `aom::jump::value_scope::<i32, aom::av1__encoder__encoder::av1_get_compressed_data::{closure#0}>` |
| 81.4 | `av1_encode` |
| 81.4 | `aom::av1__encoder__encoder::encode_with_recode_loop_and_filter` |
| 79.4 | `aom::av1__encoder__encodeframe::encode_frame_internal` |
| 79.4 | `av1_encode_frame` |
| 79.3 | `av1_encode_sb_row` |
| 79.3 | `av1_encode_tile` |
| 78.3 | `av1_rd_pick_partition` |
| 69.0 | `aom::av1__encoder__partition_search::pick_sb_modes` |
| 68.8 | `av1_rd_pick_intra_mode_sb` |
| 37.0 | `av1_rd_pick_intra_sbuv_mode` |
| 31.7 | `av1_rd_pick_intra_sby_mode` |
| 28.1 | `av1_rd_pick_palette_intra_sbuv` |
| 22.6 | `av1_txfm_rd_in_plane` |
| 22.4 | `aom::av1__encoder__tx_search::block_rd_txfm` |
| 21.1 | `aom::av1__encoder__tx_search::search_tx_type` |
| 19.8 | `av1_k_means_dim2` |
| 19.2 | `av1_rd_pick_palette_intra_sby` |
| 17.4 | `ferrisma::fast_execute::execute_fast_srgb8::<ferrisma::pipeline::run::{closure#1}>` |
| 17.3 | `<ferrisma::codecs::jpeg::JpegSource as ferrisma::decoder::DecodeSource>::read_region` |
| 15.2 | `av1_pick_uniform_tx_size_type_yrd` |
| 13.5 | `aom::av1__encoder__palette::perform_k_means_palette_search` |

## Stage shares (leaf-to-root first match)
| stage | % CPU | top self functions in stage |
| --- | ---: | --- |
| aom-encode | 81.7 | `av1_k_means_dim2` 19.6; `av1_k_means_dim1` 8.1; `aom::av1__encoder__tokenize::cost_and_tokenize_map` 6.0; `av1_optimize_txb` 5.9 |
| jpeg-decode | 10.9 | `jpeg::jdhuff::decode_mcu` 4.9; `jpeg::jdhuff::HUFF_DECODE_FAST` 3.2; `jpeg::jidctint_neon::jsimd_idct_islow_neon` 0.8; `jpeg::jidctint_neon::jsimd_idct_islow_pass2_regular` 0.8 |
| resize | 4.8 | `ferrisma_kernels::dispatch::neon::horizontal_rgba` 4.1; `ferrisma_kernels::dispatch::neon::axpy` 0.6 |
| linearize | 1.2 | `ferrisma::fast_execute::source_row_to_linear::<3>` 1.2 |
| rgb2yuv | 0.7 | `avifImageRGBToYUV` 0.6; `libsystem_m.dylib+?` 0.1; `ferrisma-base+?` 0.0 |
| engine-pipeline | 0.5 | `read` 0.2; `ferrisma::quality::taps` 0.1; `_platform_memmove` 0.1; `libsystem_m.dylib+?` 0.0 |
| quantize | 0.3 | `ferrisma::fast_execute::quantize_row::<3>` 0.3 |
