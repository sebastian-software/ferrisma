/* Neutral <assert.h> of the one-source libaom translation: assert(e) is the
 * Debug check of every Rust profile with debug assertions, and nothing
 * otherwise. Like C's assert under NDEBUG, the expression is not evaluated
 * when the check is off. */
#undef assert
#define assert(e)                                                        \
  ((ferrisma_debug_assertions() && !(e))                                 \
       ? ferrisma_assert_fail(#e, __FILE__, __LINE__, __func__)          \
       : (void)0)
#ifndef __cplusplus
#undef static_assert
#define static_assert _Static_assert
#endif
