| function (normalized) | port ms | port variant | C ms | C variant | port − C ms |
| --- | ---: | --- | ---: | --- | ---: |
| `platform_memset` | 180 | - | 9 | - | +171 |
| `av1_cnn_convolve_no_maxpool_padding_valid` | 164 | c | 12 | c/neon | +152 |
| `av1_highbd_quantize_fp` | 164 | neon | 21 | neon | +143 |
| `get_field` | 63 | - | 0 | - | +63 |
| `highbd_compute_gradient_info_sb` | 59 | - | 0 | - | +59 |
| `av1_optimize_txb` | 223 | - | 179 | - | +44 |
| `aom_highbd_hadamard_8x8` | 25 | c | 3 | neon | +22 |
| `encode_still_port+?` | 20 | - | 0 | - | +20 |
| `search_tx_type` | 61 | - | 45 | - | +16 |
| `cfl_predict_hbd_8x8` | 16 | c | 2 | neon | +14 |
| `compute_cdef_dist_highbd` | 16 | - | 2 | - | +14 |
| `platform_memmove` | 43 | - | 30 | - | +13 |
| `cfl_predict_hbd_8x16` | 14 | c | 2 | neon | +12 |
| `av1_get_nz_map_contexts` | 16 | c | 5 | neon | +11 |
| `get_tx_mask` | 10 | - | 0 | - | +10 |
| `av1_inv_txfm2d_add_8x8` | 10 | neon | 3 | neon | +7 |
| `av1_fill_coeff_costs` | 8 | - | 2 | - | +6 |
| `av1_fill_mode_rates` | 8 | - | 2 | - | +6 |
| `av1_update_and_record_txb_context` | 14 | - | 8 | - | +6 |
| `av1_get_tx_type` | 6 | - | 0 | - | +6 |
| `intra_model_rd` | 12 | - | 6 | - | +6 |
| `av1_nn_predict` | 18 | c | 12 | neon | +6 |
| `dist_block_px_domain` | 8 | - | 3 | - | +5 |
| `av1_get_qmatrix` | 8 | - | 3 | - | +5 |
| `libsystem_malloc.dylib+?` | 20 | - | 15 | - | +5 |
| `av1_setup_xform` | 6 | - | 2 | - | +4 |
| `libsystem_m.dylib+?` | 6 | - | 2 | - | +4 |
| `highbd_filter4` | 4 | - | 0 | - | +4 |
| `av1_set_screen_content_options` | 4 | - | 0 | - | +4 |
| `av1_cnn_predict_img_multi_out_highbd` | 4 | - | 0 | - | +4 |

(total port 1955 ms, C 1502 ms; gap +453 ms)
