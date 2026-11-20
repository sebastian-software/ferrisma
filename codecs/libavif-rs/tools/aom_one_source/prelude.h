/* Forced include of the one-source libaom translation (route B).
 *
 * This header only changes how the unchanged libaom C sources are presented
 * to C2Rust; the C reference builds never see it. Every unit is preprocessed
 * once, without NDEBUG and with CONFIG_DEBUG set, so Debug-only code is part
 * of the single translation. Whether it runs is decided by the Rust build
 * profile: ferrisma_debug_assertions() becomes `cfg!(debug_assertions)` and
 * ferrisma_assert_fail() becomes crate::platform::assert_fail
 * (tools/aom_one_source.py). */
#ifndef FERRISMA_AOM_ONE_SOURCE_PRELUDE_H_
#define FERRISMA_AOM_ONE_SOURCE_PRELUDE_H_
_Bool ferrisma_debug_assertions(void);
_Noreturn void ferrisma_assert_fail(const char *expression, const char *file,
                                    unsigned int line, const char *function);
#endif
