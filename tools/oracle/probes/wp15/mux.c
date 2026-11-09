// WP15 probe: the mux API of src/mux/muxedit.c, muxinternal.c and muxread.c
// (ADR 0002). It uses only the public headers, so it also builds against the
// Rust library as mux_rs (probes/wp15/oracle.cmake), which runs the same
// commands through the C ABI of src/abi/mux.rs.
//
// Commands on standard input, one per line, on one mux object (initially
// none). DATA is `file:PATH` (the file's bytes), `hex:DIGITS` (inline bytes,
// possibly none), `null` (a NULL pointer with size 0) or `huge` (one byte with
// the size MAX_CHUNK_PAYLOAD + 1, which every function refuses before it reads
// the bytes). The bytes stay valid until the end of the input, so that a mux
// may keep them without copying. FOURCC is four characters with `_` for a
// space, or `null`.
//
//   version                     WebPGetMuxVersion()
//   new                         WebPMuxNew() (deletes the current mux)
//   newversion V                WebPNewInternal(V)
//   create COPY DATA            WebPMuxCreate()
//   createversion V COPY DATA   WebPMuxCreateInternal()
//   delete                      WebPMuxDelete()
//   setchunk FOURCC COPY DATA   WebPMuxSetChunk()
//   getchunk FOURCC             WebPMuxGetChunk() into a marked WebPData
//   delchunk FOURCC             WebPMuxDeleteChunk()
//   setimage COPY DATA          WebPMuxSetImage()
//   pushframe COPY X Y DURATION ID DISPOSE BLEND DATA
//                               WebPMuxPushFrame()
//   getframe NTH HEX            WebPMuxGetFrame() into a marked frame; HEX 1
//                               prints the bitstream
//   delframe NTH                WebPMuxDeleteFrame()
//   setanim BGCOLOR LOOP        WebPMuxSetAnimationParams()
//   getanim                     WebPMuxGetAnimationParams()
//   setcanvas W H               WebPMuxSetCanvasSize()
//   getcanvas                   WebPMuxGetCanvasSize()
//   features                    WebPMuxGetFeatures()
//   numchunks ID                WebPMuxNumChunks()
//   assemble HEX                WebPMuxAssemble(); HEX 1 prints the bytes
//   state                       the queries above for every chunk id, the
//                               metadata and two unknown chunks, and every
//                               frame from 0 to the number of images (of
//                               WEBP_CHUNK_IMAGE) + 1
//   nulls                       every function with NULL and invalid
//                               arguments (pointers are not printed)
//
// Every command prints one or more `key=value` lines; byte strings are printed
// as their size, whether the pointer is NULL and their 64-bit FNV-1a digest.
// tests/wp15_mux.rs reproduces the output with the Rust translation, except
// for `nulls`; tests/wp15_abi.rs compares this probe with mux_rs.

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "src/webp/format_constants.h"
#include "src/webp/mux.h"
#include "probe_support.h"

// The 64-bit FNV-1a digest, as tests/wp8_support computes it.
static unsigned long long Digest(const uint8_t* data, size_t size) {
  unsigned long long hash = 0xcbf29ce484222325ULL;
  size_t i;
  for (i = 0; i < size; ++i) {
    hash ^= (unsigned long long)data[i];
    hash *= 0x100000001b3ULL;
  }
  return hash;
}

// Buffers that live until the end of the input.
static uint8_t** g_buffers = NULL;
static size_t g_num_buffers = 0;

static uint8_t* Keep(uint8_t* buffer) {
  g_buffers = (uint8_t**)realloc(g_buffers,
                                 (g_num_buffers + 1) * sizeof(*g_buffers));
  if (g_buffers == NULL) abort();
  g_buffers[g_num_buffers++] = buffer;
  return buffer;
}

static int HexValue(char c) {
  if (c >= '0' && c <= '9') return c - '0';
  if (c >= 'a' && c <= 'f') return c - 'a' + 10;
  if (c >= 'A' && c <= 'F') return c - 'A' + 10;
  return -1;
}

static void Die(const char* what, const char* token) {
  fprintf(stderr, "mux probe: %s: %s\n", what, token);
  exit(2);
}

// A DATA token as described above.
static WebPData ParseData(const char* token) {
  WebPData data;
  if (!strncmp(token, "file:", 5)) {
    data.bytes = Keep(ProbeReadInput(token + 5, &data.size));
  } else if (!strncmp(token, "hex:", 4)) {
    const char* digits = token + 4;
    const size_t size = strlen(digits) / 2;
    uint8_t* const bytes = Keep((uint8_t*)malloc(size + 1));
    size_t i;
    if (bytes == NULL) abort();
    for (i = 0; i < size; ++i) {
      const int hi = HexValue(digits[2 * i]), lo = HexValue(digits[2 * i + 1]);
      if (hi < 0 || lo < 0) Die("bad hex", token);
      bytes[i] = (uint8_t)(hi * 16 + lo);
    }
    data.bytes = bytes;
    data.size = size;
  } else if (!strcmp(token, "null")) {
    data.bytes = NULL;
    data.size = 0;
  } else if (!strcmp(token, "huge")) {
    data.bytes = Keep((uint8_t*)calloc(1, 1));
    data.size = (size_t)MAX_CHUNK_PAYLOAD + 1;
  } else {
    Die("bad data", token);
  }
  return data;
}

// A FOURCC token: four characters with '_' for a space, or `null`.
static const char* ParseFourCC(const char* token, char fourcc[5]) {
  int i;
  if (!strcmp(token, "null")) return NULL;
  if (strlen(token) != 4) Die("bad fourcc", token);
  for (i = 0; i < 4; ++i) fourcc[i] = (token[i] == '_') ? ' ' : token[i];
  fourcc[4] = '\0';
  return fourcc;
}

static void PrintData(const char* key, const WebPData* data, int hex) {
  printf("%s.data=%lu,%d,%016llx\n", key, (unsigned long)data->size,
         data->bytes == NULL, Digest(data->bytes, data->size));
  if (hex) {
    char name[64];
    snprintf(name, sizeof(name), "%s.hex", key);
    ProbePrintHex(name, data->bytes, data->size);
  }
}

static WebPMux* g_mux = NULL;

static void SetMux(WebPMux* mux) {
  WebPMuxDelete(g_mux);
  g_mux = mux;
}

// A WebPData that GetChunk may or may not write: a marked pointer and size.
static const uint8_t kMark[1] = { 0x5a };

static void GetChunk(const char* fourcc_token) {
  char buffer[5];
  const char* const fourcc = ParseFourCC(fourcc_token, buffer);
  WebPData data;
  WebPMuxError err;
  data.bytes = kMark;
  data.size = 12345;
  err = WebPMuxGetChunk(g_mux, fourcc, &data);
  printf("getchunk.%s=%d\n", fourcc_token, err);
  if (data.bytes == kMark && data.size == 12345) {
    printf("getchunk.%s.untouched=1\n", fourcc_token);
  } else if (err == WEBP_MUX_OK) {
    PrintData("getchunk", &data, data.size <= 64);
  } else {
    printf("getchunk.%s.reset=%d,%lu\n", fourcc_token, data.bytes == NULL,
           (unsigned long)data.size);
  }
}

static void GetFrame(uint32_t nth, int hex) {
  WebPMuxFrameInfo frame;
  WebPMuxError err;
  memset(&frame, 0x5a, sizeof(frame));
  frame.bitstream.bytes = kMark;
  frame.bitstream.size = 12345;
  err = WebPMuxGetFrame(g_mux, nth, &frame);
  printf("getframe.%u=%d\n", nth, err);
  printf("getframe.%u.info=%d,%d,%d,%d,%d,%d,%u\n", nth, frame.x_offset,
         frame.y_offset, frame.duration, (int)frame.id,
         (int)frame.dispose_method, (int)frame.blend_method, frame.pad[0]);
  if (frame.bitstream.bytes == kMark && frame.bitstream.size == 12345) {
    printf("getframe.%u.untouched=1\n", nth);
  } else {
    PrintData("getframe", &frame.bitstream, hex);
    WebPDataClear(&frame.bitstream);
  }
}

static void NumChunks(int id) {
  int num = -77;
  const WebPMuxError err = WebPMuxNumChunks(g_mux, (WebPChunkId)id, &num);
  printf("numchunks.%d=%d,%d\n", id, err, num);
}

static void GetAnim(void) {
  WebPMuxAnimParams params;
  WebPMuxError err;
  params.bgcolor = 0x5a5a5a5a;
  params.loop_count = -77;
  err = WebPMuxGetAnimationParams(g_mux, &params);
  printf("getanim=%d,%u,%d\n", err, params.bgcolor, params.loop_count);
}

static void GetCanvas(void) {
  int width = -77, height = -77;
  const WebPMuxError err = WebPMuxGetCanvasSize(g_mux, &width, &height);
  printf("getcanvas=%d,%d,%d\n", err, width, height);
}

static void Features(void) {
  uint32_t flags = 0x5a5a5a5a;
  const WebPMuxError err = WebPMuxGetFeatures(g_mux, &flags);
  printf("features=%d,%u\n", err, flags);
}

static void Assemble(int hex) {
  WebPData data;
  WebPMuxError err;
  data.bytes = kMark;
  data.size = 12345;
  err = WebPMuxAssemble(g_mux, &data);
  printf("assemble=%d\n", err);
  PrintData("assemble", &data, hex && err == WEBP_MUX_OK);
  WebPDataClear(&data);
}

static void State(void) {
  static const char* const kTags[] = { "VP8X", "ICCP", "ANIM", "EXIF", "XMP_",
                                       "ANMF", "ALPH", "VP8_", "VP8L", "abcd",
                                       "zzzz" };
  int id, images = 0;
  size_t i;
  uint32_t nth;
  Features();
  GetCanvas();
  for (id = 0; id <= 11; ++id) NumChunks(id);
  GetAnim();
  for (i = 0; i < sizeof(kTags) / sizeof(kTags[0]); ++i) GetChunk(kTags[i]);
  if (WebPMuxNumChunks(g_mux, WEBP_CHUNK_IMAGE, &images) != WEBP_MUX_OK) {
    images = 0;
  }
  for (nth = 0; nth <= (uint32_t)images + 1; ++nth) GetFrame(nth, 0);
}

// Every function with NULL and invalid arguments around a fresh mux.
static void Nulls(void) {
  static const uint8_t kByte[1] = { 0 };
  const WebPData empty = { kByte, 0 };
  const WebPData null_bytes = { NULL, 0 };
  const WebPData null_sized = { NULL, 4 };
  WebPMux* const mux = WebPMuxNew();
  WebPMuxFrameInfo frame;
  WebPMuxAnimParams params = { 0, 0 };
  WebPData data;
  int width, height, num;
  uint32_t flags;

  printf("nulls.create.null=%d\n", WebPMuxCreate(NULL, 0) != NULL);
  printf("nulls.create.nullbytes=%d\n", WebPMuxCreate(&null_bytes, 1) != NULL);
  printf("nulls.create.nullsized=%d\n", WebPMuxCreate(&null_sized, 0) != NULL);
  printf("nulls.create.empty=%d\n", WebPMuxCreate(&empty, 0) != NULL);
  printf("nulls.newversion=%d\n", WebPNewInternal(0x0200) != NULL);
  WebPMuxDelete(NULL);

  printf("nulls.setchunk.mux=%d\n", WebPMuxSetChunk(NULL, "ICCP", &empty, 1));
  printf("nulls.setchunk.fourcc=%d\n", WebPMuxSetChunk(mux, NULL, &empty, 1));
  printf("nulls.setchunk.data=%d\n", WebPMuxSetChunk(mux, "ICCP", NULL, 1));
  printf("nulls.setchunk.bytes=%d\n",
         WebPMuxSetChunk(mux, "ICCP", &null_bytes, 0));
  printf("nulls.setchunk.sized=%d\n",
         WebPMuxSetChunk(mux, "ICCP", &null_sized, 0));
  printf("nulls.getchunk.mux=%d\n", WebPMuxGetChunk(NULL, "ICCP", &data));
  printf("nulls.getchunk.fourcc=%d\n", WebPMuxGetChunk(mux, NULL, &data));
  printf("nulls.getchunk.data=%d\n", WebPMuxGetChunk(mux, "ICCP", NULL));
  printf("nulls.delchunk.mux=%d\n", WebPMuxDeleteChunk(NULL, "ICCP"));
  printf("nulls.delchunk.fourcc=%d\n", WebPMuxDeleteChunk(mux, NULL));

  printf("nulls.setimage.mux=%d\n", WebPMuxSetImage(NULL, &empty, 1));
  printf("nulls.setimage.data=%d\n", WebPMuxSetImage(mux, NULL, 1));
  printf("nulls.setimage.bytes=%d\n", WebPMuxSetImage(mux, &null_bytes, 1));
  printf("nulls.setimage.sized=%d\n", WebPMuxSetImage(mux, &null_sized, 0));
  memset(&frame, 0, sizeof(frame));
  frame.id = WEBP_CHUNK_ANMF;
  printf("nulls.pushframe.mux=%d\n", WebPMuxPushFrame(NULL, &frame, 1));
  printf("nulls.pushframe.frame=%d\n", WebPMuxPushFrame(mux, NULL, 1));
  printf("nulls.pushframe.bytes=%d\n", WebPMuxPushFrame(mux, &frame, 1));
  frame.bitstream = null_sized;
  printf("nulls.pushframe.sized=%d\n", WebPMuxPushFrame(mux, &frame, 0));
  frame.bitstream = empty;
  frame.id = WEBP_CHUNK_IMAGE;
  printf("nulls.pushframe.id=%d\n", WebPMuxPushFrame(mux, &frame, 0));
  printf("nulls.getframe.mux=%d\n", WebPMuxGetFrame(NULL, 1, &frame));
  printf("nulls.getframe.frame=%d\n", WebPMuxGetFrame(mux, 1, NULL));
  printf("nulls.delframe.mux=%d\n", WebPMuxDeleteFrame(NULL, 1));

  printf("nulls.setanim.mux=%d\n", WebPMuxSetAnimationParams(NULL, &params));
  printf("nulls.setanim.params=%d\n", WebPMuxSetAnimationParams(mux, NULL));
  printf("nulls.getanim.mux=%d\n", WebPMuxGetAnimationParams(NULL, &params));
  printf("nulls.getanim.params=%d\n", WebPMuxGetAnimationParams(mux, NULL));
  printf("nulls.setcanvas.mux=%d\n", WebPMuxSetCanvasSize(NULL, 1, 1));
  printf("nulls.getcanvas.mux=%d\n", WebPMuxGetCanvasSize(NULL, &width, &height));
  printf("nulls.getcanvas.width=%d\n", WebPMuxGetCanvasSize(mux, NULL, &height));
  printf("nulls.getcanvas.height=%d\n", WebPMuxGetCanvasSize(mux, &width, NULL));
  printf("nulls.features.mux=%d\n", WebPMuxGetFeatures(NULL, &flags));
  printf("nulls.features.flags=%d\n", WebPMuxGetFeatures(mux, NULL));
  printf("nulls.numchunks.mux=%d\n", WebPMuxNumChunks(NULL, WEBP_CHUNK_ICCP, &num));
  printf("nulls.numchunks.num=%d\n", WebPMuxNumChunks(mux, WEBP_CHUNK_ICCP, NULL));
  printf("nulls.assemble.data=%d\n", WebPMuxAssemble(mux, NULL));
  data.bytes = kMark;
  data.size = 12345;
  printf("nulls.assemble.mux=%d\n", WebPMuxAssemble(NULL, &data));
  printf("nulls.assemble.mux.data=%d,%lu\n", data.bytes == NULL,
         (unsigned long)data.size);
  data.bytes = kMark;
  data.size = 12345;
  printf("nulls.assemble.empty=%d\n", WebPMuxAssemble(mux, &data));
  printf("nulls.assemble.empty.data=%d,%lu\n", data.bytes == NULL,
         (unsigned long)data.size);
  WebPMuxDelete(mux);
}

#define MAX_TOKENS 16

int main(int argc, char* argv[]) {
  char line[8192];
  (void)argv;
  ProbeCheckArgs(argc, 0, 0, "mux < commands");
  while (fgets(line, sizeof(line), stdin) != NULL) {
    char* tokens[MAX_TOKENS];
    int count = 0;
    char* token = strtok(line, " \r\n");
    while (token != NULL && count < MAX_TOKENS) {
      tokens[count++] = token;
      token = strtok(NULL, " \r\n");
    }
    if (count == 0) continue;
#define ARGS(n) if (count != (n) + 1) Die("bad arguments", tokens[0])
    if (!strcmp(tokens[0], "version")) {
      ARGS(0);
      printf("version=%d\n", WebPGetMuxVersion());
    } else if (!strcmp(tokens[0], "new")) {
      ARGS(0);
      SetMux(WebPMuxNew());
      printf("new=%d\n", g_mux != NULL);
    } else if (!strcmp(tokens[0], "newversion")) {
      ARGS(1);
      SetMux(WebPNewInternal(atoi(tokens[1])));
      printf("new=%d\n", g_mux != NULL);
    } else if (!strcmp(tokens[0], "create")) {
      WebPData data;
      ARGS(2);
      data = ParseData(tokens[2]);
      SetMux(WebPMuxCreate(&data, atoi(tokens[1])));
      printf("create=%d\n", g_mux != NULL);
    } else if (!strcmp(tokens[0], "createversion")) {
      WebPData data;
      ARGS(3);
      data = ParseData(tokens[3]);
      SetMux(WebPMuxCreateInternal(&data, atoi(tokens[2]), atoi(tokens[1])));
      printf("create=%d\n", g_mux != NULL);
    } else if (!strcmp(tokens[0], "delete")) {
      ARGS(0);
      SetMux(NULL);
      printf("delete=1\n");
    } else if (!strcmp(tokens[0], "setchunk")) {
      char buffer[5];
      WebPData data;
      ARGS(3);
      data = ParseData(tokens[3]);
      printf("setchunk=%d\n",
             WebPMuxSetChunk(g_mux, ParseFourCC(tokens[1], buffer), &data,
                             atoi(tokens[2])));
    } else if (!strcmp(tokens[0], "getchunk")) {
      ARGS(1);
      GetChunk(tokens[1]);
    } else if (!strcmp(tokens[0], "delchunk")) {
      char buffer[5];
      ARGS(1);
      printf("delchunk=%d\n",
             WebPMuxDeleteChunk(g_mux, ParseFourCC(tokens[1], buffer)));
    } else if (!strcmp(tokens[0], "setimage")) {
      WebPData data;
      ARGS(2);
      data = ParseData(tokens[2]);
      printf("setimage=%d\n", WebPMuxSetImage(g_mux, &data, atoi(tokens[1])));
    } else if (!strcmp(tokens[0], "pushframe")) {
      WebPMuxFrameInfo frame;
      ARGS(8);
      memset(&frame, 0, sizeof(frame));
      frame.x_offset = atoi(tokens[2]);
      frame.y_offset = atoi(tokens[3]);
      frame.duration = atoi(tokens[4]);
      frame.id = (WebPChunkId)atoi(tokens[5]);
      frame.dispose_method = (WebPMuxAnimDispose)atoi(tokens[6]);
      frame.blend_method = (WebPMuxAnimBlend)atoi(tokens[7]);
      frame.bitstream = ParseData(tokens[8]);
      printf("pushframe=%d\n", WebPMuxPushFrame(g_mux, &frame, atoi(tokens[1])));
    } else if (!strcmp(tokens[0], "getframe")) {
      ARGS(2);
      GetFrame((uint32_t)strtoul(tokens[1], NULL, 10), atoi(tokens[2]));
    } else if (!strcmp(tokens[0], "delframe")) {
      ARGS(1);
      printf("delframe=%d\n",
             WebPMuxDeleteFrame(g_mux, (uint32_t)strtoul(tokens[1], NULL, 10)));
    } else if (!strcmp(tokens[0], "setanim")) {
      WebPMuxAnimParams params;
      ARGS(2);
      params.bgcolor = (uint32_t)strtoul(tokens[1], NULL, 10);
      params.loop_count = atoi(tokens[2]);
      printf("setanim=%d\n", WebPMuxSetAnimationParams(g_mux, &params));
    } else if (!strcmp(tokens[0], "getanim")) {
      ARGS(0);
      GetAnim();
    } else if (!strcmp(tokens[0], "setcanvas")) {
      ARGS(2);
      printf("setcanvas=%d\n",
             WebPMuxSetCanvasSize(g_mux, atoi(tokens[1]), atoi(tokens[2])));
    } else if (!strcmp(tokens[0], "getcanvas")) {
      ARGS(0);
      GetCanvas();
    } else if (!strcmp(tokens[0], "features")) {
      ARGS(0);
      Features();
    } else if (!strcmp(tokens[0], "numchunks")) {
      ARGS(1);
      NumChunks(atoi(tokens[1]));
    } else if (!strcmp(tokens[0], "assemble")) {
      ARGS(1);
      Assemble(atoi(tokens[1]));
    } else if (!strcmp(tokens[0], "state")) {
      ARGS(0);
      State();
    } else if (!strcmp(tokens[0], "nulls")) {
      ARGS(0);
      Nulls();
    } else {
      Die("unknown command", tokens[0]);
    }
#undef ARGS
    fflush(stdout);
  }
  SetMux(NULL);
  while (g_num_buffers > 0) free(g_buffers[--g_num_buffers]);
  free(g_buffers);
  return 0;
}
