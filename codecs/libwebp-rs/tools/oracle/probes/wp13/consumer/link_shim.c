// WP13 consumer build: a link stand-in for the one symbol that the
// Rust-linked cwebp references outside the libwebp headers that the Rust
// library exports. examples/cwebp.c prints SharpYuvGetVersion() for -version;
// SharpYUV's header is not part of the C ABI (ADR 0004), so the Rust library
// does not export it. A static link resolves every symbol of the object file,
// so this definition only satisfies the linker: it aborts if it is ever
// called, so a test can never pass through it. This file is part of the test
// harness, not of the library.

#include <stdio.h>
#include <stdlib.h>

#include "sharpyuv/sharpyuv.h"

int SharpYuvGetVersion(void) {
  fprintf(stderr, "SharpYuvGetVersion is not exported by libwebp-rs\n");
  abort();
  return 0;
}
