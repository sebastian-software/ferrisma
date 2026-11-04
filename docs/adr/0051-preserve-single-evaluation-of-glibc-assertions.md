# Preserve single evaluation of glibc assertions

The native Linux Debug root profile exposes a C2Rust 0.21.0 duplication of
glibc's assertion expansion. The original macro's `sizeof` operand is
unevaluated; its following statement expression evaluates the predicate once.
The generated output includes both an extra assertion and that runtime
statement expression. Repeated side effects corrupt sample-transform recipes
even though Release preprocessing and the Darwin assertion expansion pass.

Correct this representation during native root import. Require adjacent copies
at the same scope with identical predicate tokens and original failure-call
arguments; only optional trailing function-call commas may differ. Remove the
extra copy and retain the complete runtime assertion, including its original
condition, diagnostics, and failure call. Reject unknown shapes, differing
copies, or an incomplete assertion inventory. The default Linux Debug profile
contains 231 such sites across 13 units. Existing codec crate bodies, original
C sources, test bodies, inputs, parameters, and disabled flags remain unchanged.

Verify true and false runtime paths, single evaluation, and guard rejections,
then run all three affected unchanged original test executables before repeating
complete native root Debug acceptance. A passing focused regression is not
complete target acceptance. Keep the original failed evidence and independent
passing Release and macOS evidence.
