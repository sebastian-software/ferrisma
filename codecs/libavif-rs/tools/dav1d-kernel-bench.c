/* Calls one 8 bpc inverse-transform kernel of a dav1d library many times on
 * fixed inputs, for tools/measure_dav1d_kernels.py (ADR 0062). The library is
 * either the original with its assembly (the DSO of check_dav1d_dsp.py
 * --simd) or the port's cdylib; both are restricted to the same CPU level.
 *
 *   dav1d-kernel-bench LIBRARY MASK TX TYPE EOB ITERATIONS call|baseline
 *
 * Each iteration restores the coefficients (the kernels clear them) and, in
 * `call` mode, calls the kernel. `baseline` does the same work without the
 * call, so the difference of the two instruction counts is the kernel's. The
 * printed checksum of the destination lets the runner compare libraries. */
#include <dlfcn.h>
#include <stddef.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef void (*itxfm_fn)(uint8_t *dst, ptrdiff_t stride, int16_t *coeff, int eob);
typedef struct { itxfm_fn itxfm_add[19][17]; } InvTxfmContext;

static const int dims[19][2] = {
    {4, 4}, {8, 8}, {16, 16}, {32, 32}, {64, 64}, {4, 8}, {8, 4}, {8, 16}, {16, 8},
    {16, 32}, {32, 16}, {32, 64}, {64, 32}, {4, 16}, {16, 4}, {8, 32}, {32, 8}, {16, 64}, {64, 16},
};

int main(int argc, char **argv) {
    if (argc != 8) {
        fprintf(stderr, "usage: %s LIBRARY MASK TX TYPE EOB ITERATIONS call|baseline\n", argv[0]);
        return 2;
    }
    void *library = dlopen(argv[1], RTLD_NOW | RTLD_LOCAL);
    if (!library) { fprintf(stderr, "%s\n", dlerror()); return 1; }
    void (*init_cpu)(void) = (void (*)(void)) dlsym(library, "dav1d_init_cpu");
    void (*set_mask)(unsigned) = (void (*)(unsigned)) dlsym(library, "dav1d_set_cpu_flags_mask");
    void (*init)(InvTxfmContext *, int) = (void (*)(InvTxfmContext *, int)) dlsym(library, "dav1d_itx_dsp_init_8bpc");
    if (!init_cpu || !set_mask || !init) { fprintf(stderr, "missing symbols\n"); return 1; }
    init_cpu();
    set_mask((unsigned) strtoul(argv[2], NULL, 0));
    static InvTxfmContext c;
    init(&c, 8);
    const int tx = atoi(argv[3]), type = atoi(argv[4]), eob = atoi(argv[5]);
    const long iterations = atol(argv[6]);
    const int call = !strcmp(argv[7], "call");
    const int w = dims[tx][0], h = dims[tx][1], n = (w > 32 ? 32 : w) * (h > 32 ? 32 : h);
    itxfm_fn fn = c.itxfm_add[tx][type];
    static int16_t source[32 * 32], coeff[32 * 32];
    static uint8_t dst[64 * 64];
    static volatile int16_t sink;
    uint32_t seed = 12345;
    for (int i = 0; i < n; i++) {
        seed = seed * 1664525u + 1013904223u;
        /* Small coefficients, denser at low frequencies. */
        source[i] = (int16_t) (((int) (seed >> 16) % 512) - 256) / (1 + i / 16);
    }
    for (int i = 0; i < 64 * 64; i++) dst[i] = (uint8_t) (i * 37 + 11);
    for (long k = 0; k < iterations; k++) {
        memcpy(coeff, source, sizeof(int16_t) * (size_t) n);
        if (call) fn(dst, 64, coeff, eob);
        sink = coeff[k & 31];
    }
    (void) sink;
    uint64_t sum = 0;
    for (int i = 0; i < 64 * 64; i++) sum = sum * 31 + dst[i];
    printf("checksum=%016llx\n", (unsigned long long) sum);
    return 0;
}
