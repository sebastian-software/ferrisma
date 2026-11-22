#ifndef FERRISMA_NEUTRAL_MATH_H_
#define FERRISMA_NEUTRAL_MATH_H_
/* FLT_EVAL_METHOD is 0 on both targets. */
typedef float float_t;
typedef double double_t;
#define INFINITY __builtin_inff()
#define NAN __builtin_nanf("")
#define HUGE_VAL __builtin_huge_val()
#define HUGE_VALF __builtin_huge_valf()
#define M_E 2.71828182845904523536028747135266250
#define M_LOG2E 1.44269504088896340735992468100189214
#define M_LOG10E 0.434294481903251827651128918916605082
#define M_LN2 0.693147180559945309417232121458176568
#define M_LN10 2.30258509299404568401799145468436421
#define M_PI 3.14159265358979323846264338327950288
#define M_PI_2 1.57079632679489661923132169163975144
#define M_SQRT2 1.41421356237309504880168872420969808
#define isnan(x) __builtin_isnan(x)
#define isinf(x) __builtin_isinf(x)
#define isfinite(x) __builtin_isfinite(x)
#define signbit(x) __builtin_signbit(x)
double fabs(double x);
float fabsf(float x);
double floor(double x);
float floorf(float x);
double ceil(double x);
float ceilf(float x);
double round(double x);
float roundf(float x);
double rint(double x);
long lrint(double x);
long lround(double x);
double sqrt(double x);
float sqrtf(float x);
double pow(double x, double y);
float powf(float x, float y);
double exp(double x);
float expf(float x);
double exp2(double x);
double log(double x);
float logf(float x);
double log2(double x);
float log2f(float x);
double log10(double x);
double log1p(double x);
float log1pf(float x);
double cos(double x);
double sin(double x);
double atan(double x);
double atan2(double y, double x);
double tanh(double x);
float tanhf(float x);
double fmax(double x, double y);
double fmin(double x, double y);
float fmaxf(float x, float y);
float fminf(float x, float y);
double fmod(double x, double y);
double trunc(double x);
double cbrt(double x);
double erf(double x);
#endif
