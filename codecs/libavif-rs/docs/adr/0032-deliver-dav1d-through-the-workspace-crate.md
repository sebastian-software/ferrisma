# Deliver dav1d through the workspace crate

Use the mechanical `libavif-dav1d` crate for the existing libavif dav1d adapter
and remove native dav1d from Cargo and original-consumer link requirements.
Keep the original headers, C reference, and C/C++ test tooling for independent
validation; archive, symbol, and dynamic dependency checks must establish that
they do not supply a decoder implementation to the Rust consumer. This replaces
one default engine boundary without changing codec selection: libaom and the
optional native engines retain their current behavior until their own ports pass
the corresponding acceptance gates.
