/* Neutral <pthread.h>. Every type keeps its C library's layout through
 * libc:: in Rust (tools/transpile_dav1d.py corrections); the placeholder
 * records below only give Clang the same sizes and alignments. */
#ifndef FERRISMA_NEUTRAL_PTHREAD_H_
#define FERRISMA_NEUTRAL_PTHREAD_H_
#include <stddef.h>
#if defined(__linux__) && !defined(__GLIBC__)
/* glibc announces itself through <features.h>, which every glibc header
 * includes; libaom tests it in aom_thread.c. */
#define __GLIBC__ 2
#endif
#if defined(__APPLE__)
struct ferrisma_pthread_mutex { long sig; char opaque[56]; };
struct ferrisma_pthread_cond { long sig; char opaque[40]; };
struct ferrisma_pthread_attr { long sig; char opaque[56]; };
struct ferrisma_pthread_mutexattr { long sig; char opaque[8]; };
struct ferrisma_pthread_condattr { long sig; char opaque[8]; };
struct ferrisma_pthread_once { long sig; char opaque[8]; };
struct ferrisma_pthread;
typedef struct ferrisma_pthread *pthread_t;
#define PTHREAD_ONCE_INIT {0x30B1BCBA, {0}}
#else
struct ferrisma_pthread_mutex { long align; char size[32]; };
struct ferrisma_pthread_cond { long long align; char size[40]; };
struct ferrisma_pthread_attr { long align; char size[48]; };
struct ferrisma_pthread_mutexattr { int align; };
struct ferrisma_pthread_condattr { int align; };
struct ferrisma_pthread_once { int value; };
typedef unsigned long pthread_t;
#define PTHREAD_ONCE_INIT {0}
#endif
typedef struct ferrisma_pthread_mutex pthread_mutex_t;
typedef struct ferrisma_pthread_cond pthread_cond_t;
typedef struct ferrisma_pthread_attr pthread_attr_t;
typedef struct ferrisma_pthread_mutexattr pthread_mutexattr_t;
typedef struct ferrisma_pthread_condattr pthread_condattr_t;
typedef struct ferrisma_pthread_once pthread_once_t;
int pthread_create(pthread_t *thread, const pthread_attr_t *attributes,
                   void *(*start)(void *), void *argument);
int pthread_join(pthread_t thread, void **result);
pthread_t pthread_self(void);
int pthread_attr_init(pthread_attr_t *attributes);
int pthread_attr_destroy(pthread_attr_t *attributes);
int pthread_attr_getstacksize(const pthread_attr_t *attributes, size_t *size);
int pthread_attr_setstacksize(pthread_attr_t *attributes, size_t size);
int pthread_mutex_init(pthread_mutex_t *mutex, const pthread_mutexattr_t *attributes);
int pthread_mutex_destroy(pthread_mutex_t *mutex);
int pthread_mutex_lock(pthread_mutex_t *mutex);
int pthread_mutex_unlock(pthread_mutex_t *mutex);
int pthread_cond_init(pthread_cond_t *condition, const pthread_condattr_t *attributes);
int pthread_cond_destroy(pthread_cond_t *condition);
int pthread_cond_signal(pthread_cond_t *condition);
int pthread_cond_broadcast(pthread_cond_t *condition);
int pthread_cond_wait(pthread_cond_t *condition, pthread_mutex_t *mutex);
int pthread_once(pthread_once_t *once, void (*routine)(void));
/* Darwin names only the calling thread, glibc any thread; aom_thread.c
 * calls the form of its C library (a target detail that stays in Rust). */
#if defined(__APPLE__)
int pthread_setname_np(const char *name);
#else
int pthread_setname_np(pthread_t thread, const char *name);
#endif
#endif
