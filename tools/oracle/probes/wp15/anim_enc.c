// WP15 probe: the animation encoder of src/mux/anim_encode.c (ADR 0002). It
// uses only the public headers, so it also builds against the Rust library as
// anim_enc_rs (probes/wp15/oracle.cmake), which runs the same commands through
// the C ABI of src/abi/mux.rs.
//
// Commands on standard input, one per line, on one encoder (initially none):
//
//   options null                    the next `new` passes NULL options
//   options MIN KMIN KMAX MIXED VERBOSE BGCOLOR LOOP
//                                   WebPAnimEncoderOptionsInit(), then these
//   optionsinit V                   WebPAnimEncoderOptionsInitInternal(V) on
//                                   marked options, printed
//   new W H                         WebPAnimEncoderNew() (deletes the current)
//   newversion V W H                WebPAnimEncoderNewInternal()
//   delete                          WebPAnimEncoderDelete()
//   config null | KEY=VALUE...      the configuration of the next frames:
//                                   NULL, or WebPConfigInit() and the values
//   hook N                          the next frames' progress hook: none (-1),
//                                   counting (0), or failing at call N
//   add TS W H ARGB PATH            imports the W x H RGBA file at PATH into a
//                                   picture (ARGB 1: use_argb, 0: YUV) and
//                                   calls WebPAnimEncoderAdd() with it
//   addnull TS                      WebPAnimEncoderAdd(enc, NULL, TS, config)
//   assemble OUT                    WebPAnimEncoderAssemble(); writes the
//                                   bytes to OUT unless it is `-`
//   setchunk FOURCC COPY HEX        WebPAnimEncoderSetChunk() (HEX may be
//                                   `hex:` for no bytes)
//   getchunk FOURCC                 WebPAnimEncoderGetChunk()
//   delchunk FOURCC                 WebPAnimEncoderDeleteChunk()
//   nulls                           every function with NULL and invalid
//                                   arguments
//
// After each call the probe prints its result, the picture's error code, the
// encoder's error string, the number of progress hook calls, and for
// `assemble` the size and 64-bit FNV-1a digest of the output. The hook is
// counted per call: C's encoder keeps the last frame's hook in its canvas and
// calls it again in WebPAnimEncoderAssemble() (docs/status/wp15.md).
// tests/wp15_anim_encode.rs reproduces the output with the Rust translation,
// except for `nulls`; tests/wp15_abi.rs compares this probe with anim_enc_rs.

#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "src/webp/encode.h"
#include "src/webp/mux.h"
#include "probe_support.h"

static unsigned long long Digest(const uint8_t* data, size_t size) {
  unsigned long long hash = 0xcbf29ce484222325ULL;
  size_t i;
  for (i = 0; i < size; ++i) {
    hash ^= (unsigned long long)data[i];
    hash *= 0x100000001b3ULL;
  }
  return hash;
}

static void Die(const char* what, const char* token) {
  fprintf(stderr, "anim_enc probe: %s: %s\n", what, token);
  exit(2);
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
  return -1;
}

static WebPData ParseHex(const char* token) {
  WebPData data;
  const char* digits;
  size_t i, size;
  uint8_t* bytes;
  if (strncmp(token, "hex:", 4)) Die("bad data", token);
  digits = token + 4;
  size = strlen(digits) / 2;
  bytes = Keep((uint8_t*)malloc(size + 1));
  if (bytes == NULL) abort();
  for (i = 0; i < size; ++i) {
    const int hi = HexValue(digits[2 * i]), lo = HexValue(digits[2 * i + 1]);
    if (hi < 0 || lo < 0) Die("bad hex", token);
    bytes[i] = (uint8_t)(hi * 16 + lo);
  }
  data.bytes = bytes;
  data.size = size;
  return data;
}

static const char* ParseFourCC(const char* token, char fourcc[5]) {
  int i;
  if (strlen(token) != 4) Die("bad fourcc", token);
  for (i = 0; i < 4; ++i) fourcc[i] = (token[i] == '_') ? ' ' : token[i];
  fourcc[4] = '\0';
  return fourcc;
}

static WebPAnimEncoder* g_enc = NULL;
static WebPAnimEncoderOptions g_options;
static int g_null_options = 1;
static WebPConfig g_config;
static int g_null_config = 1;

// The progress hook: counts its calls and fails at call g_fail_at (0: never).
static int g_hook_mode = -1;
static int g_hook_calls = 0;
static int g_fail_at = 0;

static int Hook(int percent, const WebPPicture* picture) {
  (void)percent;
  (void)picture;
  ++g_hook_calls;
  return !(g_fail_at > 0 && g_hook_calls == g_fail_at);
}

static void PrintError(const char* key) {
  const char* const error = WebPAnimEncoderGetError(g_enc);
  printf("%s.error=%s\n", key, error == NULL ? "(null)" : error);
}

static void SetConfig(char** tokens, int count) {
  int i;
  if (count == 2 && !strcmp(tokens[1], "null")) {
    g_null_config = 1;
    return;
  }
  g_null_config = 0;
  if (!WebPConfigInit(&g_config)) abort();
  for (i = 1; i < count; ++i) {
    char* const eq = strchr(tokens[i], '=');
    const char* key = tokens[i];
    double value;
    if (eq == NULL) Die("bad config", tokens[i]);
    *eq = '\0';
    value = atof(eq + 1);
    if (!strcmp(key, "quality")) {
      g_config.quality = (float)value;
    } else {
      static const struct {
        const char* name;
        size_t offset;
      } kInts[] = {
#define INT_FIELD(name) { #name, offsetof(WebPConfig, name) }
        INT_FIELD(lossless), INT_FIELD(method), INT_FIELD(exact),
        INT_FIELD(thread_level), INT_FIELD(alpha_quality),
        INT_FIELD(alpha_compression), INT_FIELD(alpha_filtering),
        INT_FIELD(near_lossless), INT_FIELD(use_sharp_yuv),
        INT_FIELD(sns_strength), INT_FIELD(filter_strength),
        INT_FIELD(filter_sharpness), INT_FIELD(autofilter), INT_FIELD(segments),
        INT_FIELD(preprocessing), INT_FIELD(partitions), INT_FIELD(pass),
        INT_FIELD(target_size), INT_FIELD(low_memory)
#undef INT_FIELD
      };
      size_t k;
      for (k = 0; k < sizeof(kInts) / sizeof(kInts[0]); ++k) {
        if (!strcmp(key, kInts[k].name)) break;
      }
      if (k == sizeof(kInts) / sizeof(kInts[0])) Die("unknown config key", key);
      *(int*)((char*)&g_config + kInts[k].offset) = (int)value;
    }
  }
}

static void Add(int timestamp, int width, int height, int argb,
                const char* path) {
  WebPPicture pic;
  size_t size;
  uint8_t* const rgba = ProbeReadInput(path, &size);
  int ok;
  if (!WebPPictureInit(&pic)) abort();
  pic.width = width;
  pic.height = height;
  pic.use_argb = argb;
  if (size != (size_t)width * height * 4) Die("bad frame size", path);
  if (!WebPPictureImportRGBA(&pic, rgba, width * 4)) Die("import", path);
  free(rgba);
  if (g_hook_mode >= 0) {
    pic.progress_hook = Hook;
    pic.user_data = &g_hook_calls;
  }
  g_hook_calls = 0;
  g_fail_at = g_hook_mode;
  ok = WebPAnimEncoderAdd(g_enc, &pic, timestamp,
                          g_null_config ? NULL : &g_config);
  g_fail_at = 0;
  printf("add=%d\n", ok);
  printf("add.error_code=%d\n", pic.error_code);
  printf("add.use_argb=%d\n", pic.use_argb);
  PrintError("add");
  printf("add.hook=%d\n", g_hook_calls);
  WebPPictureFree(&pic);
}

static void Assemble(const char* out) {
  WebPData data;
  int ok;
  WebPDataInit(&data);
  g_hook_calls = 0;
  ok = WebPAnimEncoderAssemble(g_enc, &data);
  printf("assemble=%d\n", ok);
  PrintError("assemble");
  printf("assemble.data=%lu,%016llx\n", (unsigned long)data.size,
         Digest(data.bytes, data.size));
  printf("assemble.hook=%d\n", g_hook_calls);
  if (strcmp(out, "-") && data.size > 0) {
    FILE* const file = fopen(out, "wb");
    if (file == NULL || fwrite(data.bytes, data.size, 1, file) != 1) {
      Die("cannot write", out);
    }
    fclose(file);
  }
  WebPDataClear(&data);
}

static const uint8_t kMark[1] = { 0x5a };

static void GetChunk(const char* token) {
  char buffer[5];
  WebPData data;
  WebPMuxError err;
  data.bytes = kMark;
  data.size = 12345;
  err = WebPAnimEncoderGetChunk(g_enc, ParseFourCC(token, buffer), &data);
  printf("getchunk.%s=%d\n", token, err);
  if (data.bytes == kMark && data.size == 12345) {
    printf("getchunk.%s.untouched=1\n", token);
  } else {
    printf("getchunk.%s.data=%lu,%d,%016llx\n", token, (unsigned long)data.size,
           data.bytes == NULL, Digest(data.bytes, data.size));
  }
}

static void Nulls(void) {
  static const uint8_t kByte[1] = { 0 };
  const WebPData empty = { kByte, 0 };
  const WebPData null_bytes = { NULL, 0 };
  WebPAnimEncoderOptions options;
  WebPAnimEncoder* const enc = WebPAnimEncoderNew(4, 4, NULL);
  WebPPicture pic;
  WebPData data;
  printf("nulls.optionsinit.null=%d\n",
         WebPAnimEncoderOptionsInitInternal(NULL, WEBP_MUX_ABI_VERSION));
  printf("nulls.optionsinit.version=%d\n",
         WebPAnimEncoderOptionsInitInternal(&options, 0x0200));
  printf("nulls.new.version=%d\n",
         WebPAnimEncoderNewInternal(4, 4, NULL, 0x0200) != NULL);
  WebPAnimEncoderDelete(NULL);
  if (!WebPPictureInit(&pic)) abort();
  printf("nulls.add.enc=%d\n", WebPAnimEncoderAdd(NULL, &pic, 0, NULL));
  printf("nulls.add.enc.error_code=%d\n", pic.error_code);
  printf("nulls.assemble.enc=%d\n", WebPAnimEncoderAssemble(NULL, &data));
  printf("nulls.assemble.data=%d\n", WebPAnimEncoderAssemble(enc, NULL));
  printf("nulls.assemble.data.error=%s\n", WebPAnimEncoderGetError(enc));
  printf("nulls.geterror.enc=%d\n", WebPAnimEncoderGetError(NULL) == NULL);
  printf("nulls.setchunk.enc=%d\n",
         WebPAnimEncoderSetChunk(NULL, "ICCP", &empty, 1));
  printf("nulls.setchunk.fourcc=%d\n",
         WebPAnimEncoderSetChunk(enc, NULL, &empty, 1));
  printf("nulls.setchunk.data=%d\n",
         WebPAnimEncoderSetChunk(enc, "ICCP", NULL, 1));
  printf("nulls.setchunk.bytes=%d\n",
         WebPAnimEncoderSetChunk(enc, "ICCP", &null_bytes, 1));
  printf("nulls.getchunk.enc=%d\n", WebPAnimEncoderGetChunk(NULL, "ICCP", &data));
  printf("nulls.getchunk.fourcc=%d\n", WebPAnimEncoderGetChunk(enc, NULL, &data));
  printf("nulls.getchunk.data=%d\n", WebPAnimEncoderGetChunk(enc, "ICCP", NULL));
  printf("nulls.delchunk.enc=%d\n", WebPAnimEncoderDeleteChunk(NULL, "ICCP"));
  printf("nulls.delchunk.fourcc=%d\n", WebPAnimEncoderDeleteChunk(enc, NULL));
  // Frames that the encoder refuses before it reads their planes.
  pic.width = 4;
  pic.height = 5;
  printf("nulls.add.size=%d,%d\n", WebPAnimEncoderAdd(enc, &pic, 0, NULL),
         pic.error_code);
  printf("nulls.add.size.error=%s\n", WebPAnimEncoderGetError(enc));
  WebPAnimEncoderDelete(enc);
}

#define MAX_TOKENS 32

int main(int argc, char* argv[]) {
  char line[8192];
  (void)argv;
  ProbeCheckArgs(argc, 0, 0, "anim_enc < commands");
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
    if (!strcmp(tokens[0], "options")) {
      if (count == 2 && !strcmp(tokens[1], "null")) {
        g_null_options = 1;
      } else {
        ARGS(7);
        g_null_options = 0;
        if (!WebPAnimEncoderOptionsInit(&g_options)) abort();
        g_options.minimize_size = atoi(tokens[1]);
        g_options.kmin = atoi(tokens[2]);
        g_options.kmax = atoi(tokens[3]);
        g_options.allow_mixed = atoi(tokens[4]);
        g_options.verbose = atoi(tokens[5]);
        g_options.anim_params.bgcolor = (uint32_t)strtoul(tokens[6], NULL, 10);
        g_options.anim_params.loop_count = atoi(tokens[7]);
      }
    } else if (!strcmp(tokens[0], "optionsinit")) {
      WebPAnimEncoderOptions options;
      ARGS(1);
      memset(&options, 0x5a, sizeof(options));
      printf("optionsinit=%d\n",
             WebPAnimEncoderOptionsInitInternal(&options, atoi(tokens[1])));
      printf("optionsinit.options=%u,%d,%d,%d,%d,%d,%d,%u\n",
             options.anim_params.bgcolor, options.anim_params.loop_count,
             options.minimize_size, options.kmin, options.kmax,
             options.allow_mixed, options.verbose, options.padding[0]);
    } else if (!strcmp(tokens[0], "new") || !strcmp(tokens[0], "newversion")) {
      const int version_given = !strcmp(tokens[0], "newversion");
      const int version = version_given ? atoi(tokens[1]) : WEBP_MUX_ABI_VERSION;
      ARGS(version_given ? 3 : 2);
      WebPAnimEncoderDelete(g_enc);
      g_enc = WebPAnimEncoderNewInternal(atoi(tokens[1 + version_given]),
                                         atoi(tokens[2 + version_given]),
                                         g_null_options ? NULL : &g_options,
                                         version);
      printf("new=%d\n", g_enc != NULL);
      if (g_enc != NULL) PrintError("new");
    } else if (!strcmp(tokens[0], "delete")) {
      ARGS(0);
      WebPAnimEncoderDelete(g_enc);
      g_enc = NULL;
      printf("delete=1\n");
    } else if (!strcmp(tokens[0], "config")) {
      SetConfig(tokens, count);
    } else if (!strcmp(tokens[0], "hook")) {
      ARGS(1);
      g_hook_mode = atoi(tokens[1]);
    } else if (!strcmp(tokens[0], "add")) {
      ARGS(5);
      Add(atoi(tokens[1]), atoi(tokens[2]), atoi(tokens[3]), atoi(tokens[4]),
          tokens[5]);
    } else if (!strcmp(tokens[0], "addnull")) {
      ARGS(1);
      g_hook_calls = 0;
      printf("add=%d\n", WebPAnimEncoderAdd(g_enc, NULL, atoi(tokens[1]),
                                            g_null_config ? NULL : &g_config));
      PrintError("add");
    } else if (!strcmp(tokens[0], "assemble")) {
      ARGS(1);
      Assemble(tokens[1]);
    } else if (!strcmp(tokens[0], "setchunk")) {
      char buffer[5];
      WebPData data;
      ARGS(3);
      data = ParseHex(tokens[3]);
      printf("setchunk=%d\n",
             WebPAnimEncoderSetChunk(g_enc, ParseFourCC(tokens[1], buffer),
                                     &data, atoi(tokens[2])));
    } else if (!strcmp(tokens[0], "getchunk")) {
      ARGS(1);
      GetChunk(tokens[1]);
    } else if (!strcmp(tokens[0], "delchunk")) {
      char buffer[5];
      ARGS(1);
      printf("delchunk=%d\n",
             WebPAnimEncoderDeleteChunk(g_enc, ParseFourCC(tokens[1], buffer)));
    } else if (!strcmp(tokens[0], "nulls")) {
      ARGS(0);
      Nulls();
    } else {
      Die("unknown command", tokens[0]);
    }
#undef ARGS
    fflush(stdout);
    fflush(stderr);
  }
  WebPAnimEncoderDelete(g_enc);
  while (g_num_buffers > 0) free(g_buffers[--g_num_buffers]);
  free(g_buffers);
  return 0;
}
