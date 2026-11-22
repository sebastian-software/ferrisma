/* Neutral <stdio.h>. FILE stays opaque (libc::FILE in Rust). The standard
 * streams are reached through ferrisma_stderr()/ferrisma_stdout(), which
 * crate::platform maps to each C library's own symbol (__stderrp on macOS,
 * stderr on glibc). */
#ifndef FERRISMA_NEUTRAL_STDIO_H_
#define FERRISMA_NEUTRAL_STDIO_H_
#include <stddef.h>
#include <stdarg.h>
typedef struct ferrisma_FILE FILE;
#define EOF (-1)
#define SEEK_SET 0
#define SEEK_CUR 1
#define SEEK_END 2
FILE *ferrisma_stderr(void);
FILE *ferrisma_stdout(void);
#define stderr (ferrisma_stderr())
#define stdout (ferrisma_stdout())
FILE *fopen(const char *path, const char *mode);
int fclose(FILE *file);
int feof(FILE *file);
int fgetc(FILE *file);
int ungetc(int character, FILE *file);
int fseek(FILE *file, long offset, int origin);
long ftell(FILE *file);
size_t fread(void *buffer, size_t size, size_t count, FILE *file);
size_t fwrite(const void *buffer, size_t size, size_t count, FILE *file);
int fflush(FILE *file);
int fputs(const char *string, FILE *file);
int fputc(int character, FILE *file);
int printf(const char *format, ...);
int fprintf(FILE *file, const char *format, ...);
int fscanf(FILE *file, const char *format, ...);
int sscanf(const char *string, const char *format, ...);
int snprintf(char *buffer, size_t size, const char *format, ...);
int vfprintf(FILE *file, const char *format, va_list arguments);
int vsnprintf(char *buffer, size_t size, const char *format, va_list arguments);
#endif
