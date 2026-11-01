# Validate new crates on macOS ARM64 and Linux x86-64

Use macOS ARM64 as the initial pilot platform and require Linux x86-64 evidence
before releasing a new crate. Each supported target compares the Rust port
with its declared native reference; matching each local reference does not by
itself establish identical output between different platforms.

Record the source, compiler, build flags, executed test inventory, and exact
comparison results for each platform. Later platforms need their own evidence
rather than inheriting a portability claim from a successful host build.

The current libavif-rs root package only supports macOS ARM64. Linux jobs for
independent new packages must select those packages explicitly until the root
package receives its own Linux translation and validation. Passing a new crate
on Linux is not a claim that the complete workspace or integrated AVIF pipeline
already works there.
