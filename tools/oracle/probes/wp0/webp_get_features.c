// WP0 example probe: prints the WebPGetFeatures result for one input.
//
// Usage: webp_get_features [FILE]
// Reads FILE, or standard input when FILE is omitted or "-", and prints the
// returned VP8StatusCode and every WebPBitstreamFeatures field, including the
// fields left by a failing call, since partial state on error is part of the
// compared behavior. tests/wp0_features.rs runs it over the test vectors.

#include <stdio.h>
#include <stdlib.h>

#include "probe_support.h"
#include "src/webp/decode.h"

int main(int argc, char* argv[]) {
  size_t size;
  uint8_t* data;
  WebPBitstreamFeatures features;
  VP8StatusCode status;
  ProbeCheckArgs(argc, 0, 1, "webp_get_features [FILE]");
  data = ProbeReadInput(argc > 1 ? argv[1] : NULL, &size);
  status = WebPGetFeatures(data, size, &features);
  printf("status=%d\n", (int)status);
  printf("width=%d\n", features.width);
  printf("height=%d\n", features.height);
  printf("has_alpha=%d\n", features.has_alpha);
  printf("has_animation=%d\n", features.has_animation);
  printf("format=%d\n", features.format);
  free(data);
  return 0;
}
