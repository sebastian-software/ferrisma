/* Test-only observation of the original alloc_fail.c wrapper results. */
#include <pthread.h>
#include <stdatomic.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

static _Atomic unsigned calls[5], failures[5];
static _Atomic unsigned folded_calloc_calls;
void *port_original_wrap_malloc(size_t);
int port_original_wrap_posix_memalign(void **, size_t, size_t);
int port_original_wrap_pthread_create(pthread_t *, const pthread_attr_t *, void *(*)(void *), void *);
int port_original_wrap_pthread_cond_init(pthread_cond_t *, const pthread_condattr_t *);
int port_original_wrap_pthread_mutex_init(pthread_mutex_t *, const pthread_mutexattr_t *);

void *__wrap_malloc(size_t size) {
    atomic_fetch_add_explicit(&calls[0], 1, memory_order_relaxed);
    void *result = port_original_wrap_malloc(size);
    if (!result) atomic_fetch_add_explicit(&failures[0], 1, memory_order_relaxed);
    return result;
}

/* The selected decoder source has no calloc calls. LLVM can fold its malloc
 * followed by zeroing into calloc. Recover that source operation for the
 * original malloc fault hook in test objects only. */
void *port_wrap_folded_calloc(size_t count, size_t size) {
    atomic_fetch_add_explicit(&folded_calloc_calls, 1, memory_order_relaxed);
    if (count && size > SIZE_MAX / count) return NULL;
    size_t total = count * size;
    void *result = __wrap_malloc(total);
    if (result) memset(result, 0, total);
    return result;
}

int __wrap_posix_memalign(void **pointer, size_t alignment, size_t size) {
    atomic_fetch_add_explicit(&calls[1], 1, memory_order_relaxed);
    int result = port_original_wrap_posix_memalign(pointer, alignment, size);
    if (result) atomic_fetch_add_explicit(&failures[1], 1, memory_order_relaxed);
    return result;
}

int __wrap_pthread_create(pthread_t *thread, const pthread_attr_t *attr, void *(*start)(void *), void *arg) {
    atomic_fetch_add_explicit(&calls[2], 1, memory_order_relaxed);
    int result = port_original_wrap_pthread_create(thread, attr, start, arg);
    if (result) atomic_fetch_add_explicit(&failures[2], 1, memory_order_relaxed);
    return result;
}

int __wrap_pthread_cond_init(pthread_cond_t *cond, const pthread_condattr_t *attr) {
    atomic_fetch_add_explicit(&calls[3], 1, memory_order_relaxed);
    int result = port_original_wrap_pthread_cond_init(cond, attr);
    if (result) atomic_fetch_add_explicit(&failures[3], 1, memory_order_relaxed);
    return result;
}

int __wrap_pthread_mutex_init(pthread_mutex_t *mutex, const pthread_mutexattr_t *attr) {
    atomic_fetch_add_explicit(&calls[4], 1, memory_order_relaxed);
    int result = port_original_wrap_pthread_mutex_init(mutex, attr);
    if (result) atomic_fetch_add_explicit(&failures[4], 1, memory_order_relaxed);
    return result;
}

static void report(void) {
    fprintf(stderr, "[port-faults] {\"calls\":[%u,%u,%u,%u,%u],\"failures\":[%u,%u,%u,%u,%u],\"folded_calloc_calls\":%u}\n",
            atomic_load(&calls[0]), atomic_load(&calls[1]), atomic_load(&calls[2]),
            atomic_load(&calls[3]), atomic_load(&calls[4]),
            atomic_load(&failures[0]), atomic_load(&failures[1]), atomic_load(&failures[2]),
            atomic_load(&failures[3]), atomic_load(&failures[4]), atomic_load(&folded_calloc_calls));
}

__attribute__((constructor)) static void initialize(void) { atexit(report); }
