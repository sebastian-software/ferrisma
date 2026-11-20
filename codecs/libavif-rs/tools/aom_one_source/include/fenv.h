/* Neutral <fenv.h>. libaom only enables floating-point exceptions in its
 * GNU Linux Debug build (HAVE_FEXCEPT && CONFIG_DEBUG, aom_encoder.c);
 * crate::platform keeps exactly that: the real calls on GNU Linux with debug
 * assertions, -1 (no previous mask, nothing to restore) everywhere else. */
#ifndef FERRISMA_NEUTRAL_FENV_H_
#define FERRISMA_NEUTRAL_FENV_H_
#define FE_DIVBYZERO 4
#define FE_OVERFLOW 8
#define FE_UNDERFLOW 16
#define FE_ALL_EXCEPT 61
int ferrisma_feenableexcept(int excepts);
int ferrisma_fedisableexcept(int excepts);
#define feenableexcept ferrisma_feenableexcept
#define fedisableexcept ferrisma_fedisableexcept
#endif
