# Preserve preprocessor branches as complete generated module variants

Translate each supported experimental preprocessor combination independently
and select changed Rust modules with Cargo `cfg` attributes. Keep unchanged
modules shared byte-for-byte and retain complete generated variants where the
C source differs; this avoids manually reconstructing conditionals inside large
parser and writer functions while preserving independent Mini and Extended-PIXI
feature switches and their original test configurations.
