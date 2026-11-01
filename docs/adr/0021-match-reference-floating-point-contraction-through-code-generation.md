# Match reference floating-point contraction through code generation

Keep the original AppleClang C reference and the translated expressions.
For aarch64-apple-darwin, enable LLVM floating-point contraction through the
pinned Rust compiler's `-C llvm-args=-fp-contract=fast` option. Record this
compiler setting as part of the reference configuration, rather than replacing
individual expressions with Rust-specific fused operations. Linux x86-64 uses
the default compiler settings and requires its own exact comparison.

The SharpYUV development profile uses `opt-level = 1` with debug assertions
enabled: at level zero, Rust does not perform the required contraction even
with the backend option. This is a code-generation requirement for numeric
equivalence, not a rewrite or a performance-tuning pass. Both profiles retain
the original Debug-preprocessed assertion predicates.

The first SharpYUV corpus found 47 differences in 1,271 cases with default
AppleClang and default Rust. Disabling C contraction eliminated those differences
in a diagnostic build. Enabling Rust contraction also eliminated them while
retaining the original C reference. The latter preserves the existing primary
baseline and the source expressions. The corpus establishes equality for its
recorded cases, not a general proof for every floating-point input.

The workspace setting also applies to the existing libavif translation. Re-run
its original tests and record any remaining RGB rounding differences separately.
Sanitizer helpers that override Cargo flags must include the contraction setting.
Compiler upgrades and additional targets require new differential evidence.
This implements the source-faithful compiler contract left open in ADR 0005;
it does not authorize algorithmic or SIMD optimization.
