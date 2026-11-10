# Preserve experimental AV2 behavior through the original AVM adapter

Translate the AVM adapter and the AV2-specific OBU, reader, and writer branches
for each Mini/Extended-PIXI combination. Keep AVM v1.0.0 as the external codec,
with its source revision and build options recorded in the dependency tooling.

Its macOS shared-library export list names the removed `avm_read_obu_header`
function. Remove that stale export line only in the separately downloaded build
dependency, rather than introducing an implementation stub or changing the
vendored libavif snapshot. Both reference and Rust tests link that same codec.
