#ifndef FERRISMA_NEUTRAL_STDLIB_H_
#define FERRISMA_NEUTRAL_STDLIB_H_
#include <stddef.h>
#define EXIT_SUCCESS 0
#define EXIT_FAILURE 1
#define RAND_MAX 2147483647
_Noreturn void abort(void);
_Noreturn void exit(int status);
int abs(int x);
long labs(long x);
long long llabs(long long x);
void *malloc(size_t size);
void *calloc(size_t count, size_t size);
void *realloc(void *pointer, size_t size);
void free(void *pointer);
void qsort(void *base, size_t count, size_t size,
           int (*compare)(const void *, const void *));
int rand(void);
long strtol(const char *string, char **end, int base);
unsigned long strtoul(const char *string, char **end, int base);
double strtod(const char *string, char **end);
int atoi(const char *string);
char *getenv(const char *name);
#endif
