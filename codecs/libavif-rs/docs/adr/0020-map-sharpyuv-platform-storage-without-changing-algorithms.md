# Map SharpYUV platform storage without changing algorithms

Generate the portable configuration from the original Debug preprocessing and
retain its assertions through a helper that follows Rust's debug-assertion
configuration and calls the target C runtime. Map pthread mutex storage and
its initializer to the target libc declarations. Use raw addresses for static
storage instead of creating Rust references before locking.

SharpYUV initializes its last-used CPU callback with an object address cast to
a function pointer and compares it with another object-address sentinel.
Rust rejects such a value as `Option<fn>`. In the translation unit that only
stores and compares these values, represent the original pointer bits as an
opaque pointer. Preserve the original guards before assigning actual callbacks
or NULL to the DSP unit's callable storage. No sentinel is invoked. Verify the
pointer representation and initialization behavior on each supported target.

The public matrix declaration comes from one shared C header. Reuse that exact
Rust structure across its translated units so Rust callers can pass the result
of the matrix API to the conversion API without incompatible duplicate types.
All corrections are reproducible in `tools/transpile_sharpyuv.py`; algorithms,
ownership, and numeric expressions retain their original forms.
