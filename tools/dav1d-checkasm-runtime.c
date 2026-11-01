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
static unsigned null_selections[2];

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
    }
}

void *port_lookup(const char *name) {
    void *pointer = dlsym(handles[phase], name);
    if (!pointer && !phase) {
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
