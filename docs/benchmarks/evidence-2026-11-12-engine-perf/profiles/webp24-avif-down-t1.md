# prof/webp24-avif-down-t1.json.gz: total CPU weight 2333514 (thread CPU delta units, us)

## Self (top 40)
| % | function |
| ---: | --- |
| 15.7 | `av1_optimize_txb` |
| 7.1 | `webp::dec::vp8_dec::get_coeffs_fast` |
| 6.2 | `av1_highbd_quantize_fp_facade` |
| 5.6 | `av1_cnn_convolve_no_maxpool_padding_valid_c` |
| 4.5 | `_platform_memset` |
| 3.5 | `ferrisma_kernels::dispatch::neon::horizontal_rgba` |
| 2.6 | `<u8 as c2rust_bitfields::FieldType>::get_field` |
| 1.9 | `aom::av1__encoder__tx_search::search_tx_type` |
| 1.6 | `aom::av1__encoder__encodeframe::highbd_compute_gradient_info_sb` |
| 1.3 | `av1_fwd_txfm2d_8x8_neon` |
| 1.3 | `_platform_memmove` |
| 1.3 | `cfl_predict_hbd_8x8_c` |
| 1.2 | `webp::dec::tree_dec::vp8_parse_intra_mode_row` |
| 1.2 | `aom::av1__encoder__tx_search::dist_block_tx_domain` |
| 1.1 | `aom_highbd_quantize_b_helper_c` |
| 1.1 | `av1_predict_intra_block` |
| 1.0 | `av1_cost_coeffs_txb` |
| 1.0 | `av1_write_coeffs_txb` |
| 0.9 | `webp::dsp::upsampling_neon::upsample_rgb_line_pair` |
| 0.9 | `__bzero` |
| 0.9 | `webp::dsp::dec_neon::load4x8_neon` |
| 0.9 | `ferrisma::fast_execute::source_row_to_linear::<3>` |
| 0.9 | `aom::av1__encoder__txb_rdopt::get_coeff_cost_general` |
| 0.8 | `aom::simd::av1__encoder__arm__highbd_fwd_txfm_neon::highbd_fdct32_x4_neon` |
| 0.8 | `aom::simd::av1__common__arm__highbd_inv_txfm_neon::idct8x8_neon` |
| 0.8 | `av1_get_nz_map_contexts_c` |
| 0.7 | `av1_nn_predict_c` |
| 0.7 | `av1_update_and_record_txb_context` |
| 0.7 | `aom::simd::av1__common__arm__highbd_inv_txfm_neon::idct32x32_neon` |
| 0.7 | `cdef_filter_16_0_neon` |
| 0.7 | `av1_highbd_quantize_fp_neon` |
| 0.7 | `ferrisma_kernels::dispatch::neon::axpy` |
| 0.6 | `aom::av1__encoder__pickcdef::compute_cdef_dist_highbd` |
| 0.6 | `cdef_filter_16_2_neon` |
| 0.6 | `av1_rd_pick_intra_sby_mode` |
| 0.6 | `av1_fwd_txfm2d_16x16_neon` |
| 0.6 | `aom::simd::av1__encoder__arm__highbd_fwd_txfm_neon::highbd_fdct16_x4_neon` |
| 0.6 | `webp::dsp::dec_neon::store4x8_neon` |
| 0.5 | `webp::dec::frame_dec::reconstruct_row` |
| 0.5 | `aom::av1__encoder__txb_rdopt::update_coeff_general` |

## Inclusive (top 40)
| % | function |
| ---: | --- |
| 100.0 | `start` |
| 99.8 | `ferrisma::cli::run` |
| 99.8 | `ferrisma::main` |
| 99.8 | `std::sys::backtrace::__rust_begin_short_backtrace::<fn() -> result::Result<std::process::ExitCode, boxed::Box<dyn error::Error>>, result::Result<std::process::ExitCode, boxed::Box<dyn error::Error>>>` |
| 99.8 | `std::rt::lang_start::<result::Result<..>>::{closure#0}` |
| 99.8 | `std::rt::lang_start_internal` |
| 99.8 | `main` |
| 99.6 | `ferrisma::pipeline::run` |
| 77.2 | `<ferrisma_avif_write::Frame>::finish` |
| 77.2 | `<ferrisma::own_avif::Frame as ferrisma::encoder_backend::RowEncoder>::finish` |
| 77.2 | `<ferrisma::pipeline::Sink>::finish` |
| 76.6 | `avif::codec_aom::aomCodecEncodeImage` |
| 76.6 | `avif::write::avifEncoderAddImageInternal` |
| 76.6 | `avifEncoderWrite` |
| 76.6 | `aom::av1__av1_cx_iface::encoder_encode` |
| 76.6 | `aom_codec_encode` |
| 76.5 | `aom::jump::value_scope::<u32, aom::av1__av1_cx_iface::encoder_encode::{closure#0}>` |
| 76.4 | `av1_encode_strategy` |
| 76.4 | `aom::jump::value_scope::<i32, aom::av1__encoder__encoder::av1_get_compressed_data::{closure#0}>` |
| 76.1 | `av1_encode` |
| 76.1 | `aom::av1__encoder__encoder::encode_with_recode_loop_and_filter` |
| 70.3 | `aom::av1__encoder__encodeframe::encode_frame_internal` |
| 70.3 | `av1_encode_frame` |
| 70.2 | `av1_encode_sb_row` |
| 70.2 | `av1_encode_tile` |
| 67.9 | `av1_rd_pick_partition` |
| 47.4 | `aom::av1__encoder__partition_search::pick_sb_modes` |
| 47.1 | `av1_rd_pick_intra_mode_sb` |
| 37.6 | `av1_txfm_rd_in_plane` |
| 37.3 | `aom::av1__encoder__tx_search::block_rd_txfm` |
| 35.7 | `aom::av1__encoder__tx_search::search_tx_type` |
| 29.9 | `av1_rd_pick_intra_sby_mode` |
| 27.5 | `av1_pick_uniform_tx_size_type_yrd` |
| 22.4 | `ferrisma::fast_execute::execute_fast_srgb8::<ferrisma::pipeline::run::{closure#1}>` |
| 22.1 | `<ferrisma::codecs::webp::WebpSource as ferrisma::decoder::DecodeSource>::read_region` |
| 21.6 | `aom::av1__encoder__intra_mode_search::intra_block_yrd` |
| 18.7 | `av1_optimize_b` |
| 18.4 | `av1_optimize_txb` |
| 17.1 | `av1_rd_pick_intra_sbuv_mode` |
| 16.2 | `webp::dec::vp8_dec::vp8_decode` |

## Stage shares (leaf-to-root first match)
| stage | % CPU | top self functions in stage |
| --- | ---: | --- |
| aom-encode | 76.6 | `av1_optimize_txb` 15.7; `av1_highbd_quantize_fp_facade` 6.2; `av1_cnn_convolve_no_maxpool_padding_valid_c` 5.6; `_platform_memset` 4.4 |
| webp-decode | 16.2 | `webp::dec::vp8_dec::get_coeffs_fast` 7.1; `webp::dec::tree_dec::vp8_parse_intra_mode_row` 1.2; `webp::dsp::upsampling_neon::upsample_rgb_line_pair` 0.9; `webp::dsp::dec_neon::load4x8_neon` 0.9 |
| resize | 4.1 | `ferrisma_kernels::dispatch::neon::horizontal_rgba` 3.5; `ferrisma_kernels::dispatch::neon::axpy` 0.7 |
| linearize | 0.9 | `ferrisma::fast_execute::source_row_to_linear::<3>` 0.9 |
| engine-pipeline | 0.8 | `__bzero` 0.3; `ferrisma::quality::taps` 0.2; `fmt::format::format_inner` 0.1; `libsystem_m.dylib+?` 0.1 |
| rgb2yuv | 0.6 | `avifImageRGBToYUV` 0.4; `ferrisma-base+?` 0.1; `libsystem_m.dylib+?` 0.0 |
| quantize | 0.5 | `ferrisma::fast_execute::quantize_row::<3>` 0.5 |
| other | 0.2 | `std::__1::__copy_impl::operator()[abi:fqn220106]<std::byte*, std::byte` 0.2 |
