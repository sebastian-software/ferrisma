# Separate optimization from the mechanical port

First port the upstream portable implementations mechanically and compare them
with the corresponding portable original configuration. Keep this work
independent from SIMD, assembly, algorithm changes, and other optimization.
An initial portable milestone may be slower than the optimized native library;
its result establishes correctness for its declared configuration.

Treat optimization as separate follow-up work. Existing crates may be evaluated
for that work, while the source-corresponding port remains the comparison
baseline. This does not select an optimization dependency or make reduced
performance an unmeasured release promise.

Retain optimized reference configurations and their test inventory. A portable
build that does not register assembly tests cannot establish that those tests
passed or that optimized paths are covered. Full-library completion and the
supported configuration matrix still require explicit evidence.
