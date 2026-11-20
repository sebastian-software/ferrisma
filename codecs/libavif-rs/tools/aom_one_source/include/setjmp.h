/* Neutral <setjmp.h>. The port maps setjmp/longjmp to unwinding
 * (crates/aom/src/jump.rs); jmp_buf only has to keep each C library's size and
 * alignment inside libaom's records, which crate::platform::jmp_buf does. The
 * placeholder below gives Clang the same layout for its record queries. */
#ifndef FERRISMA_NEUTRAL_SETJMP_H_
#define FERRISMA_NEUTRAL_SETJMP_H_
#if defined(__APPLE__)
struct ferrisma_jmp_buf_element { int value; };
typedef struct ferrisma_jmp_buf_element jmp_buf[48];
#else
struct ferrisma_jmp_buf_element { long registers[8]; int mask_was_saved; unsigned long mask[16]; };
typedef struct ferrisma_jmp_buf_element jmp_buf[1];
#endif
int setjmp(jmp_buf environment);
_Noreturn void longjmp(jmp_buf environment, int value);
#endif
