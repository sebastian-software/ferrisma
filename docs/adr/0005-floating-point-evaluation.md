# Keep the translated arithmetic and expose compiler rounding differences

Do not rewrite translated floating-point expressions into Rust-specific fused
operations merely to match one C optimizer. The original Clang Release build
contracts some multiply/add expressions, while Rust retains separate rounding;
these compiler choices can change a small number of decoded RGB values despite
passing the unchanged upstream numerical tolerances.

Keep the original C Release build as the primary baseline. A separate diagnostic
decoder recompiles only `reformat.c` with `-ffp-contract=off` to establish the
cause of differences. This diagnostic does not replace the baseline, modify
upstream assertions, or establish full byte parity. Strict byte parity remains
an explicit open item until a source-faithful cross-compiler contract is chosen.
