# Port the complete dav1d portable configuration

Translate dav1d 1.5.4 at `54706fc6bc0cdecab7e9593974a4039cc038fca7`
into its own workspace crate from the original C sources. Select both upstream
bit-depth instantiations (8 and 16, covering 8/10/12-bit decoding),
`enable_asm=false`, `trim_dsp=false`, logging and threading enabled, and the
noncontracting C reference in ADR 0024. Retain Debug assertions and map system
storage and synchronization to target libc without replacing decoder algorithms.

Pin the original checkasm 1.2.0 framework and the separate dav1d test-data
repository as test inputs. Keep their source, cases, and expected hashes intact.
The normal upstream build gates checkasm on assembly: a separate validation
bridge must compare original scalar C and Rust function pointers through the
original test bodies. A successful assembly-disabled Meson run alone cannot
establish internal DSP parity. Keep original tools and fuzz drivers as consumers,
and do not replace the working libavif dependency before the standalone decoder
and its original suites establish parity.
