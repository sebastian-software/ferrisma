# Run original checkasm bodies across C and Rust

Keep all seventeen original checkasm test instantiations and their assertions,
parameters, and random generation unchanged. Prime their original headers, then
use test-only symbol-selection macros to obtain C pointers on checkasm's first
pass and Rust pointers on a labeled second pass. The driver replaces CPU-version
selection with implementation selection; no optimized CPU path is claimed.
Verify every nonnull tested function pointer belongs to the selected shared
library, and require equal nonzero function inventories and equal unsupported
null selections. The fixed-seed runs make 3,155 nonnull function selections and 1,923 unsupported
combinations in each implementation.

Build the C test reference from its original 51 objects. Small test-only
accessors expose five deliberately hidden constant-table addresses. They do not
implement decoder algorithms. Original inline helpers and the film-grain test's
`UNIT_TEST` scaling helper remain native test setup; candidate DSP calls still
resolve to Rust. The corpus and public API comparisons supply independent
decoder and film-grain integration evidence. These bridges are test tooling and
are not linked into the delivered Rust decoder.
