# Target Rust consumers without C++ binary compatibility

The follow-up library ports target Rust consumers. Preserve complete library
functionality and the exact behavior required by
[ADR 0012](0012-require-bit-exact-results-against-declared-references.md), but do
not require existing compiled C++ applications to load the Rust libraries as
binary replacements. Existing C++ class layouts, standard-library types, and
mangled symbols are not a product compatibility contract for these crates.

Foreign interfaces and compatibility glue may serve original test harnesses
and differential references. Their validation role does not create a promise
to support existing C++ applications. Preserve the source-corresponding
mechanical implementation policy rather than mixing in an idiomatic API
redesign. The current libavif test-facing C ABI remains usable; this decision
does not remove it.

The original-test coverage requirement still applies. How internally coupled
C++ tests exercise their Rust counterparts, including possible mechanical
test translation, remains a separate validation-policy decision. Executing
original C++ algorithms cannot be counted as evidence for the Rust port.
