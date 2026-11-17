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
 * printed checksum of the destination lets the runner compare libraries.
 *
 *   dav1d-kernel-bench LIBRARY MASK msac NAME NS ITERATIONS call|baseline
 *
 * calls the msac entry point NAME (`dav1d_msac_decode_symbol_adapt4_neon`,
 * …, reached through the test-only accessor `port_table_NAME` of either
 * library) on a 64 KiB stream with a fixed CDF of NS symbols, CDF
 * adaptation on; the context is re-initialized every 2048 iterations in
 * both modes. The checksum covers the decoded values.
 *
 *   dav1d-kernel-bench LIBRARY MASK lpf BPC SLOT ITERATIONS call|baseline
 *
 * calls loop_filter_sb[SLOT / 2][SLOT % 2] of dav1d_loop_filter_dsp_init_BPCbpc
 * (0: luma column edges, 1: luma row edges, 2, 3: chroma) on a 160x160
 * picture (a gradient with noise, so that every filter width and the flat
 * paths are taken) across 32 4-pixel units, with fixed masks, levels
 * (some zero) and sharpness 0; each iteration restores the picture. At 16
 * bpc the pixels are 10-bit. The checksum covers the filtered picture.
 *
 *   dav1d-kernel-bench LIBRARY MASK itx16|itx12 TX TYPE EOB ITERATIONS call|baseline
 *
 * as the first form for dav1d_itx_dsp_init_16bpc(c, 10): 32-bit
 * coefficients, 10-bit pixels (uint16_t) and bitdepth_max 1023; `itx12`
 * for dav1d_itx_dsp_init_16bpc(c, 12) with 12-bit pixels, coefficients four
 * times as large and bitdepth_max 4095 (the x86 assembly has kernels of its
 * own for 12 bits). In the itx forms the coefficients and the destination
 * are aligned to 64 bytes, as the decoder's are: the x86 kernels load and
 * store both with aligned moves.
 *
 *   dav1d-kernel-bench LIBRARY MASK cdef BPC SLOT EDGES ITERATIONS call|baseline
 *
 * calls the slot SLOT of dav1d_cdef_dsp_init_BPCbpc (0: dir, 1..3: fb[0..2],
 * the 8x8, 4x8 and 4x4 filters) on an 8x8 block of a 32x32 picture (a
 * gradient with noise) with the edge flags EDGES; the filters cycle through
 * the strengths (pri, sec) = (6, 2), (6, 0), (0, 2) (shifted by 2 at 16
 * bpc) and the eight directions, damping 5 (7 at 16 bpc); each iteration
 * restores the picture. At 16 bpc the pixels are 10-bit. The checksum
 * covers the picture and, for dir, the directions and variances.
 *
 *   dav1d-kernel-bench LIBRARY MASK ipred BPC SLOT W H PARAM ANGLE ITERATIONS call|baseline
 *
 * calls one slot of dav1d_intra_pred_dsp_init_BPCbpc for a W x H block:
 * SLOT 0..13 is intra_pred[SLOT] (enum IntraPredMode: 6, 7, 8 are Z1, Z2,
 * Z3, 13 is FILTER_PRED), 14..16 cfl_ac[SLOT - 14] (4:2:0, 4:2:2, 4:4:4),
 * 17..22 cfl_pred[SLOT - 17], 23 pal_pred. The edge, luma, ac, palette and
 * index inputs are fixed pseudo-random data (10-bit at 16 bpc). The call
 * cycles through a parameter set: for Z1..Z3 the 27 angles of the
 * original's checkasm test (or the single angle ANGLE, 0..90 relative to
 * the mode, if ANGLE >= 0), with PARAM as the angle's flag bits (0x400:
 * intra edge filter, 0x200: smooth); for FILTER_PRED the five filters; for
 * cfl_pred four values of alpha; for cfl_ac PARAM selects no padding (0)
 * or the largest w_pad and h_pad (1). ITERATIONS is rounded up to a
 * multiple of the set; one more pass over the set adds every output to the
 * checksum, in both modes, so the kernel runs ITERATIONS + set size times
 * (printed as `calls=`).
 *
 *   dav1d-kernel-bench LIBRARY MASK lr BPC SLOT W H EDGES ITERATIONS call|baseline
 *
 * calls one slot of dav1d_loop_restoration_dsp_init_BPCbpc(c, 8 or 10) on a
 * W x H unit (W <= 384, H <= 64) with the edge flags EDGES (enum
 * LrEdgeFlags): SLOT 0 and 1 are wiener[0] (7 taps) and wiener[1] (5 taps)
 * with fixed coefficients, 2..4 are sgr[0..2] (5x5, 3x3, mix) with the
 * strengths of dav1d_sgr_params[14], [10] and [0] and fixed weights. The
 * picture is the pattern of the original's checkasm test (an 8x8
 * checkerboard of 0 and the largest value, with noise in the low bits),
 * 10-bit at 16 bpc, with the two rows above and below in `lpf` and the
 * `left` columns; each iteration restores the unit's rows (the filters
 * work in place) in both modes. The checksum covers the whole picture. */
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

typedef struct {
    const uint8_t *buf_pos, *buf_end;
    size_t dif;
    unsigned rng;
    int cnt, allow_update_cdf;
#if defined(__x86_64__)
    /* The original with assembly has this member on x86-64 and its
     * dav1d_msac_init writes it (src/msac.h); the port's context ends before
     * it. */
    void *symbol_adapt16;
#endif
} MsacContext;

static int msac(void *library, char **argv) {
    void *(*accessor)(void);
    char name[160];
    snprintf(name, sizeof(name), "port_table_%s", argv[4]);
    accessor = (void *(*)(void)) dlsym(library, name);
    void (*init)(MsacContext *, const uint8_t *, size_t, int) =
        (void (*)(MsacContext *, const uint8_t *, size_t, int)) dlsym(library, "dav1d_msac_init");
    if (!accessor || !init) { fprintf(stderr, "missing symbols\n"); return 1; }
    void *fn = accessor();
    const int ns = atoi(argv[5]);
    const long iterations = atol(argv[6]);
    const int call = !strcmp(argv[7], "call");
    static uint8_t buf[65536];
    uint32_t seed = 12345;
    for (size_t i = 0; i < sizeof(buf); i++) {
        seed = seed * 1664525u + 1013904223u;
        buf[i] = (uint8_t) (seed >> 24);
    }
    /* randomize_cdf of the original checkasm test, with a fixed seed. The
     * array is aligned as the decoder's CDF arrays are: the x86 kernels
     * load it with aligned moves. */
    __attribute__((aligned(32))) uint16_t cdf[16] = { 0 };
    for (int i = ns; i > 0; i--) {
        seed = seed * 1664525u + 1013904223u;
        cdf[i - 1] = (uint16_t) (cdf[i] + (seed >> 8) % (32768u - cdf[i] - (unsigned) i) + 1);
    }
    MsacContext s;
    uint64_t sum = 0;
    unsigned f = 16384;
    const int kind = strstr(argv[4], "adapt") && strstr(argv[4], "symbol") ? 0 :
                     strstr(argv[4], "hi_tok") || strstr(argv[4], "bool_adapt") ? 1 :
                     strstr(argv[4], "bool_equi") ? 2 : 3;
    for (long k = 0; k < iterations; k++) {
        if (!(k & 2047)) init(&s, buf, sizeof(buf), 0);
        unsigned r = 0;
        if (call) {
            switch (kind) {
            case 0: r = ((unsigned (*)(MsacContext *, uint16_t *, size_t)) fn)(&s, cdf, (size_t) ns); break;
            case 1: r = ((unsigned (*)(MsacContext *, uint16_t *)) fn)(&s, cdf); break;
            case 2: r = ((unsigned (*)(MsacContext *)) fn)(&s); break;
            default: r = ((unsigned (*)(MsacContext *, unsigned)) fn)(&s, f); break;
            }
        }
        f = (f * 7 + 1) & 0x7fff;
        sum = sum * 31 + r;
    }
    printf("checksum=%016llx\n", (unsigned long long) sum);
    return 0;
}

typedef struct { uint8_t e[64], i[64]; uint64_t sharp[2]; } Av1FilterLUT;
typedef void (*lpf_fn)(void *dst, ptrdiff_t stride, const uint32_t *mask, const uint8_t (*l)[4],
                       ptrdiff_t b4_stride, const Av1FilterLUT *lut, int w, int bitdepth_max);

static int lpf(void *library, char **argv) {
    const int bpc = atoi(argv[4]), slot = atoi(argv[5]);
    const long iterations = atol(argv[6]);
    const int call = !strcmp(argv[7], "call");
    char name[64];
    snprintf(name, sizeof(name), "dav1d_loop_filter_dsp_init_%dbpc", bpc);
    void (*init)(lpf_fn (*)[2]) = (void (*)(lpf_fn (*)[2])) dlsym(library, name);
    if (!init) { fprintf(stderr, "missing symbols\n"); return 1; }
    static lpf_fn c[2][2];
    init(c);
    lpf_fn fn = c[slot / 2][slot % 2];
    enum { W = 160 };
    static uint16_t source[W * W];
    static __attribute__((aligned(64))) uint16_t picture[W * W];
    uint32_t seed = 12345;
    for (int y = 0; y < W; y++)
        for (int x = 0; x < W; x++) {
            seed = seed * 1664525u + 1013904223u;
            const int v = ((x / 8 + y / 8) * 7 + (int) (seed >> 29)) & 255;
            source[y * W + x] = (uint16_t) (bpc == 8 ? v : v * 4);
        }
    static uint8_t levels[34 * 34][4];
    for (int i = 0; i < 34 * 34; i++) {
        seed = seed * 1664525u + 1013904223u;
        levels[i][0] = (seed >> 24) % 5 == 0 ? 0 : (uint8_t) ((seed >> 16) % 64);
    }
    static Av1FilterLUT lut;
    lut.sharp[0] = 0;
    lut.sharp[1] = 63;
    const uint32_t vmask[3] = { 0xffffffffu, 0xf0f0f0f0u, slot / 2 ? 0 : 0xff00ff00u };
    const ptrdiff_t bytes = bpc == 8 ? 1 : 2, stride = W * bytes;
    uint8_t *const base = (uint8_t *) picture;
    /* 8 rows and 8 columns in: room for the 7 (row edges) or 8 (column
     * edges) pixels before the first edge. On x86-64 32 columns in: the
     * contract gives the kernels a destination aligned to 32 bytes
     * (src/loopfilter.h), and the AVX2 row-edge kernels store with aligned
     * moves; the 128 pixels of the edge then end at the picture's last
     * column. */
#if defined(__x86_64__)
    enum { X0 = 32 };
#else
    enum { X0 = 8 };
#endif
    void *const dst = base + 8 * stride + X0 * bytes;
    for (long k = 0; k < iterations; k++) {
        if (bpc == 8)
            for (int i = 0; i < W * W; i++) ((uint8_t *) picture)[i] = (uint8_t) source[i];
        else
            memcpy(picture, source, sizeof(picture));
        /* The length of the edge in 4-pixel units: the x86 kernels loop
         * over it; the C and NEON kernels follow the masks alone. */
        if (call) fn(dst, stride, vmask, (const uint8_t (*)[4]) &levels[34 + 1], 34, &lut, 32, 1023);
    }
    uint64_t sum = 0;
    for (size_t i = 0; i < sizeof(picture); i++) sum = sum * 31 + ((uint8_t *) picture)[i];
    printf("checksum=%016llx\n", (unsigned long long) sum);
    return 0;
}

typedef void (*itxfm16_fn)(uint16_t *dst, ptrdiff_t stride, int32_t *coeff, int eob, int bitdepth_max);
typedef struct { itxfm16_fn itxfm_add[19][17]; } InvTxfm16Context;

static int itx16(void *library, char **argv) {
    void (*init)(InvTxfm16Context *, int) =
        (void (*)(InvTxfm16Context *, int)) dlsym(library, "dav1d_itx_dsp_init_16bpc");
    if (!init) { fprintf(stderr, "missing symbols\n"); return 1; }
    static InvTxfm16Context c;
    const int bpc = strcmp(argv[3], "itx12") ? 10 : 12, max = (1 << bpc) - 1;
    init(&c, bpc);
    const int tx = atoi(argv[4]), type = atoi(argv[5]), eob = atoi(argv[6]);
    const long iterations = atol(argv[7]);
    const int call = !strcmp(argv[8], "call");
    const int w = dims[tx][0], h = dims[tx][1], n = (w > 32 ? 32 : w) * (h > 32 ? 32 : h);
    itxfm16_fn fn = c.itxfm_add[tx][type];
    static int32_t source[32 * 32];
    static __attribute__((aligned(64))) int32_t coeff[32 * 32];
    static __attribute__((aligned(64))) uint16_t dst[64 * 64];
    static volatile int32_t sink;
    uint32_t seed = 12345;
    for (int i = 0; i < n; i++) {
        seed = seed * 1664525u + 1013904223u;
        /* Small coefficients, denser at low frequencies (the bit depth's range). */
        source[i] = (((int) (seed >> 16) % (2 << bpc)) - (1 << bpc)) / (1 + i / 16);
    }
    for (int i = 0; i < 64 * 64; i++) dst[i] = (uint16_t) ((i * 37 + 11) & max);
    for (long k = 0; k < iterations; k++) {
        memcpy(coeff, source, sizeof(int32_t) * (size_t) n);
        if (call) fn(dst, 64 * sizeof(uint16_t), coeff, eob, max);
        sink = coeff[k & 31];
    }
    (void) sink;
    uint64_t sum = 0;
    for (int i = 0; i < 64 * 64; i++) sum = sum * 31 + dst[i];
    printf("checksum=%016llx\n", (unsigned long long) sum);
    return 0;
}

typedef int (*cdef_dir_fn)(const void *dst, ptrdiff_t stride, unsigned *var, int bitdepth_max);
typedef void (*cdef_fn)(void *dst, ptrdiff_t stride, const void *left, const void *top, const void *bottom,
                        int pri_strength, int sec_strength, int dir, int damping, unsigned edges,
                        int bitdepth_max);
typedef struct { cdef_dir_fn dir; cdef_fn fb[3]; } CdefDSPContext;

static int cdef(void *library, char **argv) {
    const int bpc = atoi(argv[4]), slot = atoi(argv[5]);
    const unsigned edges = (unsigned) strtoul(argv[6], NULL, 0);
    const long iterations = atol(argv[7]);
    const int call = !strcmp(argv[8], "call");
    char name[64];
    snprintf(name, sizeof(name), "dav1d_cdef_dsp_init_%dbpc", bpc);
    void (*init)(CdefDSPContext *) = (void (*)(CdefDSPContext *)) dlsym(library, name);
    if (!init) { fprintf(stderr, "missing symbols\n"); return 1; }
    static CdefDSPContext c;
    init(&c);
    enum { W = 32 };
    static uint16_t source[W * W], picture[W * W], left[8][2];
    uint32_t seed = 12345;
    for (int y = 0; y < W; y++)
        for (int x = 0; x < W; x++) {
            seed = seed * 1664525u + 1013904223u;
            const int v = ((x * 3 + y * 5) + (int) (seed >> 28)) & 255;
            source[y * W + x] = (uint16_t) (bpc == 8 ? v : v * 4);
        }
    for (int i = 0; i < 8; i++) {
        seed = seed * 1664525u + 1013904223u;
        left[i][0] = (uint16_t) ((seed >> 24) * (bpc == 8 ? 1 : 4));
        left[i][1] = (uint16_t) ((seed >> 16 & 255) * (bpc == 8 ? 1 : 4));
    }
    static uint8_t left8[8][2];
    for (int i = 0; i < 8; i++) left8[i][0] = (uint8_t) left[i][0], left8[i][1] = (uint8_t) left[i][1];
    const ptrdiff_t bytes = bpc == 8 ? 1 : 2, stride = W * bytes;
    uint8_t *const base = (uint8_t *) picture;
    /* The block 8 rows and 8 columns in; top and bottom are the two rows
     * above and below it. */
    uint8_t *const dst = base + 8 * stride + 8 * bytes;
    const int shift = bpc == 8 ? 0 : 2;
    /* (pri, sec) of the three filter paths; the direction cycles 0..7. */
    static const int strengths[3][2] = { { 6, 2 }, { 6, 0 }, { 0, 2 } };
    uint64_t sum = 0;
    unsigned var = 0;
    for (long k = 0; k < iterations; k++) {
        if (bpc == 8)
            for (int i = 0; i < W * W; i++) ((uint8_t *) picture)[i] = (uint8_t) source[i];
        else
            memcpy(picture, source, sizeof(picture));
        if (!call) continue;
        if (slot == 0) {
            sum = sum * 31 + (unsigned) c.dir(dst, stride, &var, 1023);
            sum = sum * 31 + var;
        } else {
            const int *s = strengths[k % 3];
            const int h = slot == 3 ? 4 : 8;
            c.fb[slot - 1](dst, stride, bpc == 8 ? (const void *) left8 : (const void *) left,
                           dst - 2 * stride, dst + h * stride, s[0] << shift, s[1] << shift, (int) (k & 7),
                           5 + shift, edges, 1023);
        }
    }
    for (size_t i = 0; i < sizeof(picture); i++) sum = sum * 31 + base[i];
    printf("checksum=%016llx\n", (unsigned long long) sum);
    return 0;
}

typedef void (*angular_ipred_fn)(void *dst, ptrdiff_t stride, const void *topleft, int width, int height,
                                 int angle, int max_width, int max_height, int bitdepth_max);
typedef void (*cfl_ac_fn)(int16_t *ac, const void *y, ptrdiff_t stride, int w_pad, int h_pad, int cw, int ch);
typedef void (*cfl_pred_fn)(void *dst, ptrdiff_t stride, const void *topleft, int width, int height,
                            const int16_t *ac, int alpha, int bitdepth_max);
typedef void (*pal_pred_fn)(void *dst, ptrdiff_t stride, const void *pal, const uint8_t *idx, int w, int h);
typedef struct {
    angular_ipred_fn intra_pred[14];
    cfl_ac_fn cfl_ac[3];
    cfl_pred_fn cfl_pred[6];
    pal_pred_fn pal_pred;
} IntraPredDSPContext;

static int ipred(void *library, char **argv) {
    const int bpc = atoi(argv[4]), slot = atoi(argv[5]), w = atoi(argv[6]), h = atoi(argv[7]);
    const int param = (int) strtol(argv[8], NULL, 0), fixed_angle = atoi(argv[9]);
    long iterations = atol(argv[10]);
    const int call = !strcmp(argv[11], "call");
    char name[64];
    snprintf(name, sizeof(name), "dav1d_intra_pred_dsp_init_%dbpc", bpc);
    void (*init)(IntraPredDSPContext *) = (void (*)(IntraPredDSPContext *)) dlsym(library, name);
    if (!init) { fprintf(stderr, "missing symbols\n"); return 1; }
    static IntraPredDSPContext c;
    init(&c);
    /* The angles of the original's checkasm test (tests/checkasm/ipred.c). */
    static const uint8_t z_angles[27] = { 3, 6, 9, 14, 17, 20, 23, 26, 29, 32, 36, 39, 42, 45, 48, 51, 54,
                                          58, 61, 64, 67, 70, 73, 76, 81, 84, 87 };
    static const int alphas[4] = { 1, -3, 9, -16 };
    const int z = slot >= 6 && slot <= 8;
    const int set = z ? (fixed_angle >= 0 ? 1 : 27) : slot == 13 ? 5 : slot >= 17 && slot <= 22 ? 4 : 1;
    iterations = (iterations + set - 1) / set * set;
    const int bitdepth_max = bpc == 8 ? 255 : 1023;
    const ptrdiff_t bytes = bpc == 8 ? 1 : 2, stride = 64 * bytes;
    /* 16 pixels behind the 257 of the edge array: Z1 at 64x64 with the intra edge filter loads up to 14 pixels
     * past them (assembly and port alike; in the decoder they are inside the task context's scratch union). */
    _Alignas(64) static uint16_t picture[64 * 64], edge[257 + 16], luma[64 * 64], pal[8];
    _Alignas(64) static int16_t ac[32 * 32];
    _Alignas(64) static uint8_t idx[32 * 64];
    uint32_t seed = 12345;
#define NEXT (seed = seed * 1664525u + 1013904223u, (int) (seed >> 16))
#define PUT(buffer, i, value) \
    (bpc == 8 ? (void) (((uint8_t *) (buffer))[i] = (uint8_t) (value)) : (void) ((buffer)[i] = (uint16_t) (value)))
    for (int i = 0; i < 257; i++) PUT(edge, i, (i * 5 + (NEXT & 63)) & bitdepth_max);
    for (int i = 0; i < 64 * 64; i++) PUT(luma, i, ((i % 64) * 3 + (i / 64) * 2 + (NEXT & 31)) & bitdepth_max);
    for (int i = 0; i < 8; i++) PUT(pal, i, NEXT & bitdepth_max);
    for (int i = 0; i < 32 * 64; i++) idx[i] = (uint8_t) (NEXT & 0x77);
    long sum_ac = (long) (w * h >> 1);
    for (int i = 0; i < w * h && i < 32 * 32; i++) sum_ac += ac[i] = (int16_t) (NEXT & (bitdepth_max << 3));
    if (slot >= 17 && slot <= 22)
        for (int i = 0; i < w * h; i++) ac[i] = (int16_t) (ac[i] - sum_ac / (w * h));
    const void *const topleft = (const uint8_t *) edge + 128 * bytes;
    const int ss_hor = slot != 16, ss_ver = slot == 14;
    const int w_pad = param ? (w >> 2) - (2 >> ss_hor) : 0, h_pad = param ? (h >> 2) - (2 >> ss_ver) : 0;
    uint64_t sum = 0;
    for (long k = 0; k < iterations + set; k++) {
        const int p = (int) (k % set);
        if (call) {
            if (slot < 14) {
                const int a = z ? (90 * (slot - 6) + (fixed_angle >= 0 ? fixed_angle : z_angles[p])) | param
                                : slot == 13 ? p : 0;
                c.intra_pred[slot](picture, stride, topleft, w, h, a, 128, 128, bitdepth_max);
            } else if (slot < 17) {
                c.cfl_ac[slot - 14](ac, luma, stride, w_pad > 0 ? w_pad : 0, h_pad > 0 ? h_pad : 0, w, h);
            } else if (slot < 23) {
                c.cfl_pred[slot - 17](picture, stride, topleft, w, h, ac, alphas[p], bitdepth_max);
            } else {
                c.pal_pred(picture, stride, pal, idx, w, h);
            }
        }
        if (k >= iterations) {
            for (size_t i = 0; i < sizeof(picture); i++) sum = sum * 31 + ((const uint8_t *) picture)[i];
            if (slot >= 14 && slot < 17)
                for (int i = 0; i < w * h; i++) sum = sum * 31 + (uint16_t) ac[i];
        }
    }
#undef NEXT
#undef PUT
    printf("checksum=%016llx calls=%ld\n", (unsigned long long) sum, iterations + set);
    return 0;
}

typedef void (*lr_filter_fn)(void *dst, ptrdiff_t stride, const void *left, const void *lpf, int w, int h,
                             const void *params, unsigned edges, int bitdepth_max);
typedef struct { lr_filter_fn wiener[2], sgr[3]; } LoopRestorationDSPContext;
typedef union {
    _Alignas(16) int16_t filter[2][8];
    struct { uint32_t s0, s1; int16_t w0, w1; } sgr;
} LooprestorationParams;

static int lr(void *library, char **argv) {
    const int bpc = atoi(argv[4]), slot = atoi(argv[5]), w = atoi(argv[6]), h = atoi(argv[7]);
    const unsigned edges = (unsigned) strtoul(argv[8], NULL, 0);
    const long iterations = atol(argv[9]);
    const int call = !strcmp(argv[10], "call");
    char name[64];
    snprintf(name, sizeof(name), "dav1d_loop_restoration_dsp_init_%dbpc", bpc);
    void (*init)(LoopRestorationDSPContext *, int) =
        (void (*)(LoopRestorationDSPContext *, int)) dlsym(library, name);
    if (!init || slot < 0 || slot > 4 || w < 1 || w > 384 || h < 1 || h > 64) {
        fprintf(stderr, "missing symbols or bad arguments\n");
        return 1;
    }
    static LoopRestorationDSPContext c;
    init(&c, bpc == 8 ? 8 : 10);
    const lr_filter_fn fn = slot < 2 ? c.wiener[slot] : c.sgr[slot - 2];
    const int bitdepth_max = bpc == 8 ? 255 : 1023;
    LooprestorationParams params;
    memset(&params, 0, sizeof(params));
    if (slot < 2) {
        /* Symmetric taps as the original's test draws them; 5 taps: the outer pair is zero. */
        for (int i = 0; i < 2; i++) {
            int16_t *const f = params.filter[i];
            f[0] = f[6] = slot ? 0 : 3;
            f[1] = f[5] = -7;
            f[2] = f[4] = 15;
            f[3] = (int16_t) (-(f[0] + f[1] + f[2]) * 2 + (i == 1 || bpc != 8 ? 128 : 0));
        }
    } else {
        static const uint32_t strengths[3][2] = { { 56, 0 }, { 0, 2589 }, { 140, 3236 } };
        params.sgr.s0 = strengths[slot - 2][0];
        params.sgr.s1 = strengths[slot - 2][1];
        params.sgr.w0 = params.sgr.s0 ? -32 : 0;
        params.sgr.w1 = (int16_t) ((params.sgr.s1 ? 96 : 33) - params.sgr.w0);
    }
    /* 448 pixels per row: 32 before the unit and at least 32 behind the widest one; 8 rows above the unit
     * and 8 below it. */
    enum { STRIDE = 448, ROWS = 80, X0 = 32, Y0 = 8 };
    _Alignas(64) static uint16_t source[ROWS * STRIDE], picture[ROWS * STRIDE], edge[8 * STRIDE], left[64 * 4];
    uint32_t seed = 12345;
#define NEXT (seed = seed * 1664525u + 1013904223u, (int) (seed >> 16))
#define PUT(buffer, i, value) \
    (bpc == 8 ? (void) (((uint8_t *) (buffer))[i] = (uint8_t) (value)) : (void) ((buffer)[i] = (uint16_t) (value)))
#define PATTERN(x, y) ((((x) ^ (y)) & 8 ? bitdepth_max : 0) ^ (NEXT & (bitdepth_max >> 4)))
    for (int y = 0; y < ROWS; y++)
        for (int x = 0; x < STRIDE; x++) PUT(source, y * STRIDE + x, PATTERN(x + 3, y + 5));
    for (int y = 0; y < 8; y++)
        for (int x = 0; x < STRIDE; x++) PUT(edge, y * STRIDE + x, PATTERN(x + 1, y + 2));
    for (int i = 0; i < 64 * 4; i++) PUT(left, i, PATTERN(i & 3, (i >> 2) + 6));
    const ptrdiff_t bytes = bpc == 8 ? 1 : 2, stride = STRIDE * bytes;
    memcpy(picture, source, sizeof(picture));
    uint8_t *const dst = (uint8_t *) picture + Y0 * stride + X0 * bytes;
    const uint8_t *const src = (const uint8_t *) source + Y0 * stride;
    const uint8_t *const lpf = (const uint8_t *) edge + X0 * bytes;
    for (long k = 0; k < iterations; k++) {
        memcpy(dst - X0 * bytes, src, (size_t) (h * stride));
        if (call) fn(dst, stride, left, lpf, w, h, &params, edges, bitdepth_max);
    }
#undef NEXT
#undef PUT
#undef PATTERN
    uint64_t sum = 0;
    for (size_t i = 0; i < sizeof(picture); i++) sum = sum * 31 + ((const uint8_t *) picture)[i];
    printf("checksum=%016llx calls=%ld\n", (unsigned long long) sum, iterations);
    return 0;
}

int main(int argc, char **argv) {
    if (argc == 11 && !strcmp(argv[3], "lr")) {
        void *library = dlopen(argv[1], RTLD_NOW | RTLD_LOCAL);
        if (!library) { fprintf(stderr, "%s\n", dlerror()); return 1; }
        void (*init_cpu)(void) = (void (*)(void)) dlsym(library, "dav1d_init_cpu");
        void (*set_mask)(unsigned) = (void (*)(unsigned)) dlsym(library, "dav1d_set_cpu_flags_mask");
        if (!init_cpu || !set_mask) { fprintf(stderr, "missing symbols\n"); return 1; }
        init_cpu();
        set_mask((unsigned) strtoul(argv[2], NULL, 0));
        return lr(library, argv);
    }
    if (argc == 12 && !strcmp(argv[3], "ipred")) {
        void *library = dlopen(argv[1], RTLD_NOW | RTLD_LOCAL);
        if (!library) { fprintf(stderr, "%s\n", dlerror()); return 1; }
        void (*init_cpu)(void) = (void (*)(void)) dlsym(library, "dav1d_init_cpu");
        void (*set_mask)(unsigned) = (void (*)(unsigned)) dlsym(library, "dav1d_set_cpu_flags_mask");
        if (!init_cpu || !set_mask) { fprintf(stderr, "missing symbols\n"); return 1; }
        init_cpu();
        set_mask((unsigned) strtoul(argv[2], NULL, 0));
        return ipred(library, argv);
    }
    if (argc == 9 && !strcmp(argv[3], "cdef")) {
        void *library = dlopen(argv[1], RTLD_NOW | RTLD_LOCAL);
        if (!library) { fprintf(stderr, "%s\n", dlerror()); return 1; }
        void (*init_cpu)(void) = (void (*)(void)) dlsym(library, "dav1d_init_cpu");
        void (*set_mask)(unsigned) = (void (*)(unsigned)) dlsym(library, "dav1d_set_cpu_flags_mask");
        if (!init_cpu || !set_mask) { fprintf(stderr, "missing symbols\n"); return 1; }
        init_cpu();
        set_mask((unsigned) strtoul(argv[2], NULL, 0));
        return cdef(library, argv);
    }
    if (argc == 9 && (!strcmp(argv[3], "itx16") || !strcmp(argv[3], "itx12"))) {
        void *library = dlopen(argv[1], RTLD_NOW | RTLD_LOCAL);
        if (!library) { fprintf(stderr, "%s\n", dlerror()); return 1; }
        void (*init_cpu)(void) = (void (*)(void)) dlsym(library, "dav1d_init_cpu");
        void (*set_mask)(unsigned) = (void (*)(unsigned)) dlsym(library, "dav1d_set_cpu_flags_mask");
        if (!init_cpu || !set_mask) { fprintf(stderr, "missing symbols\n"); return 1; }
        init_cpu();
        set_mask((unsigned) strtoul(argv[2], NULL, 0));
        return itx16(library, argv);
    }
    if (argc == 8 && !strcmp(argv[3], "lpf")) {
        void *library = dlopen(argv[1], RTLD_NOW | RTLD_LOCAL);
        if (!library) { fprintf(stderr, "%s\n", dlerror()); return 1; }
        void (*init_cpu)(void) = (void (*)(void)) dlsym(library, "dav1d_init_cpu");
        void (*set_mask)(unsigned) = (void (*)(unsigned)) dlsym(library, "dav1d_set_cpu_flags_mask");
        if (!init_cpu || !set_mask) { fprintf(stderr, "missing symbols\n"); return 1; }
        init_cpu();
        set_mask((unsigned) strtoul(argv[2], NULL, 0));
        return lpf(library, argv);
    }
    if (argc == 8 && !strcmp(argv[3], "msac")) {
        void *library = dlopen(argv[1], RTLD_NOW | RTLD_LOCAL);
        if (!library) { fprintf(stderr, "%s\n", dlerror()); return 1; }
        void (*init_cpu)(void) = (void (*)(void)) dlsym(library, "dav1d_init_cpu");
        void (*set_mask)(unsigned) = (void (*)(unsigned)) dlsym(library, "dav1d_set_cpu_flags_mask");
        if (!init_cpu || !set_mask) { fprintf(stderr, "missing symbols\n"); return 1; }
        init_cpu();
        set_mask((unsigned) strtoul(argv[2], NULL, 0));
        return msac(library, argv);
    }
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
    static int16_t source[32 * 32];
    static __attribute__((aligned(64))) int16_t coeff[32 * 32];
    static __attribute__((aligned(64))) uint8_t dst[64 * 64];
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
