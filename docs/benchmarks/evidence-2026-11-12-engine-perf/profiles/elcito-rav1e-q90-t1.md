# prof/elcito-rav1e-q90-t1.json.gz: total CPU weight 3983111 (thread CPU delta units, us)

## Self (top 40)
| % | function |
| ---: | --- |
| 7.9 | `jpeg::jdhuff::decode_mcu` |
| 7.9 | `rav1e::lrf::rust::sgrproj_box_ab_r1::<10>` |
| 7.8 | `<rav1e::quantize::QuantizationContext>::quantize::<i32>` |
| 7.3 | `<rav1e::context::cdf_context::ContextWriter>::write_coeffs_lv_map::<i32, rav1e::ec::WriterBase<rav1e::ec::WriterCounter>>` |
| 6.0 | `ferrisma_kernels::dispatch::neon::horizontal_rgba` |
| 5.7 | `<rav1e::ec::WriterBase<rav1e::ec::WriterCounter> as rav1e::ec::Writer>::symbol_with_update::<4>` |
| 5.3 | `jpeg::jdhuff::HUFF_DECODE_FAST` |
| 4.0 | `<rav1e::context::cdf_context::ContextWriter>::get_nz_map_contexts` |
| 3.2 | `rav1e::rdo::rdo_loop_decision::<u16, rav1e::ec::WriterBase<rav1e::ec::WriterEncoder>>` |
| 2.6 | `rav1e::lrf::sgrproj_stripe_filter::<u16, u16>` |
| 2.1 | `rav1e::encoder::encode_tx_block::<u16, rav1e::ec::WriterBase<rav1e::ec::WriterCounter>>` |
| 1.8 | `rav1e::asm::aarch64::transform::forward::forward_transform_neon::<i32>` |
| 1.8 | `ferrisma::fast_execute::source_row_to_linear::<3>` |
| 1.7 | `rav1e::lrf::rust::sgrproj_box_ab_r2::<10>` |
| 1.4 | `jpeg::jidctint_neon::jsimd_idct_islow_neon` |
| 1.2 | `rav1e::rdo::compute_distortion::<u16>` |
| 1.1 | `rav1e::encoder::encode_block_post_cdef::<u16, rav1e::ec::WriterBase<rav1e::ec::WriterCounter>>` |
| 1.1 | `<rav1e::context::cdf_context::ContextWriter>::write_coeffs_lv_map::<i32, rav1e::ec::WriterBase<rav1e::ec::WriterRecorder>>` |
| 1.1 | `jpeg::jidctint_neon::jsimd_idct_islow_pass2_regular` |
| 1.0 | `rav1e::partition::get_intra_edges::<u16>` |
| 1.0 | `_platform_memset` |
| 0.9 | `ferrisma_kernels::dispatch::neon::axpy` |
| 0.9 | `<rav1e::ec::WriterBase<rav1e::ec::WriterRecorder> as rav1e::ec::Writer>::symbol_with_update::<4>` |
| 0.8 | `rav1e::quantize::rust::dequantize::<i32>` |
| 0.8 | `<rav1e::predict::PredictionMode>::predict_intra::<u16>` |
| 0.7 | `rav1e::asm::aarch64::transform::forward::daala_fdct_ii_16::<rav1e::asm::aarch64::transform::forward::I32X8>` |
| 0.7 | `rav1e::deblock::sse_size6::<u16>` |
| 0.7 | `rav1e::rdo::rdo_cfl_alpha::<u16>::{closure#0}::{closure#0}` |
| 0.7 | `ferrisma::fast_execute::quantize_row::<3>` |
| 0.6 | `rav1e::asm::aarch64::transform::forward::daala_fdct_ii_8::<rav1e::asm::aarch64::transform::forward::I32X8>` |
| 0.6 | `jpeg::jdcolext_neon::jsimd_ycc_rgb_convert_neon::<0, 1, 2, -1, 3>` |
| 0.6 | `rav1e_ipred_cfl_128_16bpc_neon` |
| 0.6 | `rav1e::rdo::rdo_mode_decision::<u16>` |
| 0.6 | `rav1e::encoder::write_tx_blocks::<u16, rav1e::ec::WriterBase<rav1e::ec::WriterCounter>>` |
| 0.5 | `<rav1e::ec::WriterBase<rav1e::ec::WriterEncoder> as rav1e::ec::StorageBackend>::store` |
| 0.5 | `rav1e_satd8x8_hbd_neon` |
| 0.5 | `<rav1e::context::cdf_context::ContextWriter>::rollback` |
| 0.5 | `_platform_memmove` |
| 0.4 | `rav1e::rdo::rdo_loop_plane_error::<u16>` |
| 0.4 | `rav1e::deblock::deblock_size6_inner` |

## Inclusive (top 40)
| % | function |
| ---: | --- |
| 72.9 | `<rayon_registry::WorkerThread>::wait_until_cold` |
| 72.9 | `<rayon_registry::ThreadBuilder>::run` |
| 72.9 | `std::sys::backtrace::__rust_begin_short_backtrace::<<rayon_registry::DefaultSpawn as rayon_registry::ThreadSpawn>::spawn::{closure#0}, ()>` |
| 72.9 | `<std::thread::lifecycle::spawn_unchecked<..>::{closure#1} as ops::function::FnOnce<()>>::call_once::{shim:vtable#0}` |
| 72.9 | `<std::sys::thread::unix::Thread>::new::thread_start` |
| 72.9 | `_pthread_start` |
| 72.7 | `<rav1e::api::internal::ContextInner<u16>>::receive_packet` |
| 72.7 | `<rayon_job::StackJob<rayon_latch::LatchRef<rayon_latch::LockLatch>, <rayon_registry::Registry>::in_worker_cold<<rayon_thread_pool::ThreadPool>::install<<..>::receive_packet::{closure#0}, result::Resul` |
| 72.7 | `rav1e::encoder::encode_frame::<u16>` |
| 70.6 | `<&rav1e::encoder::encode_tile_group<u16>::{closure#0} as ops::function::FnMut<((rav1e::tiling::tiler::TileContextMut<u16>, &mut rav1e::context::cdf_context::CDFContext),)>>::call_mut` |
| 70.6 | `<rayon::iter::unzip::UnzipFolder<rayon::iter::unzip::Unzip, rayon::iter::collect::consumer::CollectResult<vec::Vec<u8>>, rayon::iter::collect::consumer::CollectResult<rav1e::stats::EncoderStats>> as r` |
| 70.6 | `rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<..>, rayon::iter::map::MapConsumer<rayon::iter::unzip::UnzipConsumer<rayon::iter::unzip::Unzip, rayon::iter::collect` |
| 70.6 | `<rayon::vec::IntoIter<..> as rayon::iter::IndexedParallelIterator>::with_producer::<rayon::iter::plumbing::bridge::Callback<rayon::iter::map::MapConsumer<rayon::iter::unzip::UnzipConsumer<rayon::iter:` |
| 70.6 | `rayon::iter::collect::collect_with_consumer::<rav1e::stats::EncoderStats, rayon::iter::collect::special_extend<rayon::iter::unzip::UnzipB<rayon::iter::map::Map<rayon::vec::IntoIter<..>, rav1e::encoder` |
| 70.6 | `<rayon::iter::unzip::UnzipA<rayon::iter::map::Map<rayon::vec::IntoIter<..>, rav1e::encoder::encode_tile_group<u16>::{closure#0}>, rayon::iter::unzip::Unzip, vec::Vec<rav1e::stats::EncoderStats>> as ra` |
| 70.6 | `rayon::iter::collect::collect_with_consumer::<vec::Vec<u8>, rayon::iter::collect::special_extend<rayon::iter::unzip::UnzipA<rayon::iter::map::Map<rayon::vec::IntoIter<..>, rav1e::encoder::encode_tile_` |
| 70.6 | `rayon::iter::unzip::execute::<rayon::iter::map::Map<rayon::vec::IntoIter<..>, rav1e::encoder::encode_tile_group<u16>::{closure#0}>, rayon::iter::unzip::Unzip, vec::Vec<vec::Vec<u8>>, vec::Vec<rav1e::s` |
| 46.6 | `rav1e::encoder::encode_partition_topdown::<u16, rav1e::ec::WriterBase<rav1e::ec::WriterRecorder>>` |
| 46.1 | `rav1e::rdo::rdo_mode_decision::<u16>` |
| 44.4 | `rav1e::rdo::rdo_partition_decision::<u16, rav1e::ec::WriterBase<rav1e::ec::WriterRecorder>>` |
| 36.9 | `rav1e::encoder::write_tx_blocks::<u16, rav1e::ec::WriterBase<rav1e::ec::WriterCounter>>` |
| 36.6 | `rav1e::rdo::luma_chroma_mode_rdo::<u16>::{closure#0}` |
| 36.0 | `rav1e::encoder::encode_tx_block::<u16, rav1e::ec::WriterBase<rav1e::ec::WriterCounter>>` |
| 27.1 | `start` |
| 27.0 | `ferrisma::cli::run` |
| 27.0 | `ferrisma::main` |
| 27.0 | `std::sys::backtrace::__rust_begin_short_backtrace::<fn() -> result::Result<std::process::ExitCode, boxed::Box<dyn error::Error>>, result::Result<std::process::ExitCode, boxed::Box<dyn error::Error>>>` |
| 27.0 | `std::rt::lang_start::<result::Result<..>>::{closure#0}` |
| 27.0 | `std::rt::lang_start_internal` |
| 27.0 | `main` |
| 26.9 | `ferrisma::pipeline::run` |
| 26.9 | `ferrisma::fast_execute::execute_fast_srgb8::<ferrisma::pipeline::run::{closure#1}>` |
| 26.7 | `<ferrisma::codecs::jpeg::JpegSource as ferrisma::decoder::DecodeSource>::read_region` |
| 18.2 | `<rav1e::context::cdf_context::ContextWriter>::write_coeffs_lv_map::<i32, rav1e::ec::WriterBase<rav1e::ec::WriterCounter>>` |
| 17.5 | `rav1e::encoder::check_lf_queue::<u16>` |
| 17.1 | `jpeg::jdapistd::jpeg_read_scanlines` |
| 17.1 | `<jpeg::decode::Scanlines>::read_rows` |
| 17.1 | `jpeg::jdmainct::process_data_simple_main` |
| 17.0 | `rav1e::rdo::rdo_loop_decision::<u16, rav1e::ec::WriterBase<rav1e::ec::WriterEncoder>>` |
| 16.2 | `jpeg::jdcoefct::decompress_onepass` |

## Stage shares (leaf-to-root first match)
| stage | % CPU | top self functions in stage |
| --- | ---: | --- |
| rav1e-encode | 72.8 | `rav1e::lrf::rust::sgrproj_box_ab_r1::<10>` 7.9; `<rav1e::quantize::QuantizationContext>::quantize::<i32>` 7.8; `<rav1e::context::cdf_context::ContextWriter>::write_coeffs_lv_map::<i3` 7.3; `<rav1e::ec::WriterBase<rav1e::ec::WriterCounter> as rav1e::ec::Writer>` 5.7 |
| jpeg-decode | 17.1 | `jpeg::jdhuff::decode_mcu` 7.9; `jpeg::jdhuff::HUFF_DECODE_FAST` 5.3; `jpeg::jidctint_neon::jsimd_idct_islow_neon` 1.4; `jpeg::jidctint_neon::jsimd_idct_islow_pass2_regular` 1.1 |
| resize | 6.9 | `ferrisma_kernels::dispatch::neon::horizontal_rgba` 6.0; `ferrisma_kernels::dispatch::neon::axpy` 0.9; `<ferrisma_kernels::Kernels>::axpy` 0.0; `<ferrisma_kernels::Kernels>::horizontal_rgba` 0.0 |
| linearize | 1.8 | `ferrisma::fast_execute::source_row_to_linear::<3>` 1.8 |
| quantize | 0.7 | `ferrisma::fast_execute::quantize_row::<3>` 0.7 |
| engine-pipeline | 0.6 | `<ferrisma::avif_sink::AvifSink>::write_row` 0.1; `read` 0.1; `ferrisma::quality::taps` 0.1; `libsystem_m.dylib+?` 0.1 |
| other | 0.1 | `avif::avif::run_static_initializers` 0.1; `swtch_pri` 0.0 |
| sync/wait | 0.0 | `__psynch_cvwait` 0.0 |
