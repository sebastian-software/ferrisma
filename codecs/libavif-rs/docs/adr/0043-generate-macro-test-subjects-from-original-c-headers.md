# Generate macro test subjects from the original C headers

Translate small C wrappers around reviewed original macro subjects with the
same C2Rust rules and preprocessing profiles as the codec. This keeps the
algorithm in the pinned production header instead of writing a second Rust
implementation for the tests. Keep the generated subjects optional and outside
the 168 production translation units; bind the unchanged original C++ test
bodies to those Rust exports while retaining their native expected-value
calculations and complete registration inventory.
