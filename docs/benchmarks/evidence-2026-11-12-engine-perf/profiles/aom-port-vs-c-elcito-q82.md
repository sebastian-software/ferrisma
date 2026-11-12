| function (normalized) | port ms | port variant | C ms | C variant | port − C ms |
| --- | ---: | --- | ---: | --- | ---: |
| `av1_k_means_dim2` | 719 | - | 148 | - | +571 |
| `av1_k_means_dim1` | 334 | - | 68 | - | +266 |
| `platform_memset` | 143 | - | 5 | - | +138 |
| `palette_rd_y` | 101 | - | 4 | - | +98 |
| `av1_highbd_quantize_fp` | 110 | neon | 14 | neon | +96 |
| `av1_rd_pick_palette_intra_sbuv` | 104 | - | 9 | - | +95 |
| `av1_cnn_convolve_no_maxpool_padding_valid` | 78 | c | 7 | c/neon | +70 |
| `get_field` | 42 | - | 0 | - | +42 |
| `av1_optimize_txb` | 209 | - | 175 | - | +34 |
| `highbd_compute_gradient_info_sb` | 30 | - | 0 | - | +30 |
| `av1_get_nz_map_contexts` | 27 | c | 4 | neon | +23 |
| `cfl_predict_hbd_8x8` | 15 | c | 2 | neon | +13 |
| `platform_memmove` | 30 | - | 20 | - | +10 |
| `av1_write_coeffs_txb` | 15 | - | 5 | - | +10 |
| `encode_still_port+?` | 9 | - | 0 | - | +9 |
| `search_tx_type` | 39 | - | 30 | - | +8 |
| `av1_update_and_record_txb_context` | 15 | - | 7 | - | +8 |
| `inv_txfm2d_add_universe` | 27 | neon | 20 | neon | +7 |
| `aom_highbd_hadamard_8x8` | 9 | c | 2 | neon | +7 |
| `bzero` | 12 | - | 5 | - | +7 |
| `compute_cdef_dist_highbd` | 6 | - | 0 | - | +6 |
| `get_tx_mask` | 6 | - | 0 | - | +6 |
| `av1_get_tx_type` | 6 | - | 0 | - | +6 |
| `av1_quant` | 9 | - | 4 | - | +5 |
| `intra_model_rd` | 6 | - | 2 | - | +4 |
| `dist_block_px_domain` | 6 | - | 2 | - | +4 |
| `av1_nn_predict` | 9 | c | 5 | neon | +4 |
| `idct16x16` | 21 | neon | 18 | neon | +3 |
| `av1_fill_mode_rates` | 3 | - | 0 | - | +3 |
| `cfl_predict_hbd_4x4` | 3 | c | 0 | - | +3 |

(total port 2982 ms, C 1781 ms; gap +1201 ms)
