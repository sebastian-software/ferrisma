/* Test-only C/Rust loader for the unchanged original checkasm test bodies. */
#include "dav1d-checkasm-runtime.h"
#include <dlfcn.h>
#include <stdio.h>
#include <stdlib.h>

const CheckasmCpuInfo port_implementations[] = {
    { "Rust portable", "rust", 1 }, { 0 }
};
static unsigned phase;
static void *handles[2];
static void *bases[2];
static unsigned checked[2];
static unsigned levels[2];
static unsigned null_selections[2];

/* The CPU level of each implementation. Without DAV1D_PORT_SIMD_MASK (the
 * scalar comparison of ADR 0029) neither library runs dav1d_init_cpu, so
 * both report no CPU flags and select their C kernels; the candidate's mask
 * is also set to 0. With DAV1D_PORT_SIMD_MASK (the SIMD comparison of ADR
 * 0062) both libraries detect the CPU with their own dav1d_init_cpu and are
 * restricted to the same level with dav1d_set_cpu_flags_mask, the original
 * then selecting its assembly kernels and the port its intrinsics kernels. */
static void select_cpu_level(void *handle) {
    void (*init_cpu)(void) = (void (*)(void)) dlsym(handle, "dav1d_init_cpu");
    void (*set_mask)(unsigned) = (void (*)(unsigned)) dlsym(handle, "dav1d_set_cpu_flags_mask");
    /* The original keeps dav1d_cpu_flags hidden; its test-only accessor
     * port_cpu_flags returns dav1d_get_cpu_flags(). The port exports both
     * globals. */
    unsigned (*accessor)(void) = (unsigned (*)(void)) dlsym(handle, "port_cpu_flags");
    unsigned *flags = (unsigned *) dlsym(handle, "dav1d_cpu_flags");
    unsigned *mask = (unsigned *) dlsym(handle, "dav1d_cpu_flags_mask");
    const char *level = getenv("DAV1D_PORT_SIMD_MASK");
    if (!init_cpu || !set_mask || (!accessor && (!flags || !mask))) {
        fprintf(stderr, "Missing CPU selection symbols: phase=%u\n", phase);
        exit(1);
    }
    if (level) {
        init_cpu();
        set_mask((unsigned) strtoul(level, NULL, 0));
    } else if (phase) {
        set_mask(0);
    }
    levels[phase] = accessor ? accessor() : *flags & *mask;
    fprintf(stderr, "[port] phase=%u cpu_flags=0x%x\n", phase, levels[phase]);
}

/* dav1d_get_cpu_flags() of the current phase's library, for test bodies
 * that select kernels by symbol rather than through a DSP init (msac). */
unsigned port_cpu_level(void) {
    return levels[phase];
}

void port_set_phase(CheckasmCpu value) {
    phase = value != 0;
    if (!handles[phase]) {
        const char *path = getenv(phase ? "DAV1D_CANDIDATE" : "DAV1D_REFERENCE");
        if (!path || !(handles[phase] = dlopen(path, RTLD_NOW | RTLD_LOCAL))) {
            fprintf(stderr, "Cannot load implementation: %s\n", dlerror());
            exit(1);
        }
        Dl_info info;
        if (!dladdr(dlsym(handles[phase], "dav1d_version"), &info)) abort();
        bases[phase] = info.dli_fbase;
        fprintf(stderr, "[port] phase=%u library=%s\n", phase, info.dli_fname);
        select_cpu_level(handles[phase]);
    }
}

void *port_lookup(const char *name) {
    void *pointer = dlsym(handles[phase], name);
    /* Hidden symbols (the original's constant tables and assembly kernels,
     * the port's rebuilt msac kernels) through test-only accessors. */
    if (!pointer) {
        char accessor[160];
        snprintf(accessor, sizeof(accessor), "port_table_%s", name);
        void *(*get_table)(void) = (void *(*)(void)) dlsym(handles[phase], accessor);
        if (get_table) pointer = get_table();
    }
    Dl_info info;
    if (!pointer || !dladdr(pointer, &info) || info.dli_fbase != bases[phase]) {
        fprintf(stderr, "Wrong or missing implementation symbol: phase=%u %s\n", phase, name);
        exit(1);
    }
    return pointer;
}

CheckasmKey port_verify(CheckasmKey key, const char *expression, const char *file, int line) {
    if (!key) {
        null_selections[phase]++;
        return key;
    }
    Dl_info info = { 0 };
    if (!dladdr((void *) key, &info) || info.dli_fbase != bases[phase]) {
        fprintf(stderr, "DSP function pointer escaped selected implementation: phase=%u %s:%d %s (%s %s)\n",
                phase, file, line, expression, info.dli_fname, info.dli_sname);
        exit(1);
    }
    checked[phase]++;
    return key;
}

static void report_dispatch(void) {
    fprintf(stderr, "[port] checked C=%u Rust=%u function selections\n", checked[0], checked[1]);
    fprintf(stderr, "[port] null C=%u Rust=%u unsupported selections\n", null_selections[0], null_selections[1]);
    if (!checked[0] || checked[0] != checked[1] || null_selections[0] != null_selections[1]) abort();
}

__attribute__((constructor)) static void initialize_dispatch(void) {
    atexit(report_dispatch);
}
