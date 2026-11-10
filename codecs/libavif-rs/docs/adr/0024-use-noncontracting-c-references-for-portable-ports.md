# Use noncontracting C references for portable ports

Set `-ffp-contract=off` explicitly in the primary original C reference and
original C test consumers. Keep normal Rust floating-point code generation,
unoptimized development builds, and the original translated expressions.
Apply this numeric contract consistently to SharpYUV and the existing libavif
reference builds; future portable dependency ports inherit it.

This supersedes ADR 0021's Rust backend-contraction flag and development
optimization override. The selected contract avoids requiring a special Rust
compiler setting in consuming projects and preserves separate rounding in the
mechanically translated arithmetic. Source versions, algorithms, assertions,
and test tolerances remain unchanged.

Keep compiler-default Clang builds as explicitly labeled diagnostic references
and record their output differences separately. They do not replace the new
primary reference or weaken its exact comparison gate. The earlier contracting
comparison remains historical evidence. Re-run the native Debug/Release corpus,
original WebP consumers, and affected libavif integration profiles before
recording acceptance under this contract.
