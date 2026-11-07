// WP12 consumer build: link stand-ins for the symbols that imageio/webpdec.c
// references but that the Rust library does not export yet. webpdec.c's
// ReadWebP() uses the demux API (WP8, exported by WP14) and WebPPictureAlloc()
// and WebPPictureFree() (WP9, exported by WP13); dwebp never calls ReadWebP(),
// only LoadWebP(), DecodeWebP() and DecodeWebPIncremental(), which use the
// decoding API that WP12 exports. A static link resolves every symbol of the
// object file, so these definitions only satisfy the linker: each aborts if it
// is ever called, so a test can never pass through one of them. This file is
// part of the test harness, not of the library; it goes away when WP13 and WP14
// export the real functions.

#include <stdio.h>
#include <stdlib.h>

#include "src/webp/demux.h"
#include "src/webp/encode.h"

static void NotExported(const char* name) {
  fprintf(stderr, "%s is not exported by libwebp-rs yet\n", name);
  abort();
}

WebPDemuxer* WebPDemuxInternal(const WebPData* data, int allow_partial,
                               WebPDemuxState* state, int version) {
  (void)data, (void)allow_partial, (void)state, (void)version;
  NotExported("WebPDemuxInternal");
  return NULL;
}

void WebPDemuxDelete(WebPDemuxer* dmux) {
  (void)dmux;
  NotExported("WebPDemuxDelete");
}

uint32_t WebPDemuxGetI(const WebPDemuxer* dmux, WebPFormatFeature feature) {
  (void)dmux, (void)feature;
  NotExported("WebPDemuxGetI");
  return 0;
}

int WebPDemuxGetChunk(const WebPDemuxer* dmux, const char fourcc[4],
                      int chunk_number, WebPChunkIterator* iter) {
  (void)dmux, (void)fourcc, (void)chunk_number, (void)iter;
  NotExported("WebPDemuxGetChunk");
  return 0;
}

void WebPDemuxReleaseChunkIterator(WebPChunkIterator* iter) {
  (void)iter;
  NotExported("WebPDemuxReleaseChunkIterator");
}

int WebPPictureAlloc(WebPPicture* picture) {
  (void)picture;
  NotExported("WebPPictureAlloc");
  return 0;
}

void WebPPictureFree(WebPPicture* picture) {
  (void)picture;
  NotExported("WebPPictureFree");
}
