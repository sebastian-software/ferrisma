/* Test-only implementation dispatch; no decoder algorithm is defined here. */
#include <checkasm/checkasm.h>
#include <checkasm/test.h>
extern const CheckasmCpuInfo port_implementations[];
void port_set_phase(CheckasmCpu phase);
void *port_lookup(const char *name);
CheckasmKey port_verify(CheckasmKey key, const char *expression, const char *file, int line);

#undef checkasm_check_func
#define checkasm_check_func(func, ...) \
    (checkasm_key_ref = checkasm_check_key( \
        (checkasm_key_new = port_verify((CheckasmKey) (func), #func, __FILE__, __LINE__)), __VA_ARGS__))
