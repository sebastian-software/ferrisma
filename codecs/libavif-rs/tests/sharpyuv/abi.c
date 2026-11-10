#include <stddef.h>
#include <stdio.h>
#include <pthread.h>
#include "sharpyuv/sharpyuv.h"
#include "sharpyuv/sharpyuv_csp.h"

int main(void) {
  unsigned char rgb[3] = {127, 64, 201}, y[1], u[1], v[1];
  SharpYuvOptions options;
  const SharpYuvConversionMatrix *matrix = SharpYuvGetConversionMatrix(kSharpYuvMatrixWebp);
  if (!SharpYuvOptionsInit(matrix, &options)) return 1;
  int code = SharpYuvConvertWithOptions(rgb, rgb + 1, rgb + 2, 3, 3, 8,
                                       y, 1, u, 1, v, 1, 8, 1, 1, &options);
  printf("{\"matrix\":[%zu,%zu,%zu,%zu,%zu],\"options\":[%zu,%zu,%zu,%zu],"
         "\"space\":[%zu,%zu,%zu,%zu,%zu,%zu],\"mutex\":[%zu,%zu],"
         "\"callback\":[%zu,%zu],\"pixel\":[%d,%u,%u,%u]}",
         sizeof(SharpYuvConversionMatrix), _Alignof(SharpYuvConversionMatrix),
         offsetof(SharpYuvConversionMatrix, rgb_to_y), offsetof(SharpYuvConversionMatrix, rgb_to_u),
         offsetof(SharpYuvConversionMatrix, rgb_to_v),
         sizeof(SharpYuvOptions), _Alignof(SharpYuvOptions), offsetof(SharpYuvOptions, yuv_matrix), offsetof(SharpYuvOptions, transfer_type),
         sizeof(SharpYuvColorSpace), _Alignof(SharpYuvColorSpace), offsetof(SharpYuvColorSpace, kr),
         offsetof(SharpYuvColorSpace, kb), offsetof(SharpYuvColorSpace, bit_depth), offsetof(SharpYuvColorSpace, range),
         sizeof(pthread_mutex_t), _Alignof(pthread_mutex_t), sizeof(void (*)(void)), sizeof(void *),
         code, y[0], u[0], v[0]);
  return code == 1 ? 0 : 1;
}
