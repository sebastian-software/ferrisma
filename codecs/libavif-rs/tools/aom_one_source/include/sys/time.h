/* Neutral <sys/time.h>. struct timeval differs between the C libraries
 * (Darwin: int microseconds, glibc: long); Rust uses libc::timeval, the record
 * below only gives Clang the same layout. */
#ifndef FERRISMA_NEUTRAL_SYS_TIME_H_
#define FERRISMA_NEUTRAL_SYS_TIME_H_
#include <time.h>
#if defined(__APPLE__)
struct timeval { long tv_sec; int tv_usec; };
#else
struct timeval { long tv_sec; long tv_usec; };
#endif
int gettimeofday(struct timeval *time, void *zone);
#endif
