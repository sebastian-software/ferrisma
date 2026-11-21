# libavif-dav1d-kernels

dav1d 1.5.4's CDEF and inverse transforms, written from the C algorithm on
the portable vector types of `fearless_simd` (NEON on AArch64, AVX2 on
x86-64) for `ferrisma-dav1d`, which installs them in its DSP tables and holds
their tests and gates. libavif-rs ADR 0062, "Kernels written from the
algorithm". BSD-2-Clause, as dav1d; the original's license and patent grant
are in `LICENSES/` (`NOTICE.md`).
