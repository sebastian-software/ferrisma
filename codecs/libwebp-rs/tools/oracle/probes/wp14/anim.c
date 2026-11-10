// WP14 probe: the animation decoder of src/demux/anim_decode.c and the demux
// API around it (ADR 0002). It uses only the public headers, so it also builds
// against the Rust library as anim_rs (probes/wp14/oracle.cmake), which runs the
// same commands through the C ABI of src/abi/demux.rs. Commands on standard
// input, one per line:
//
//   version
//       prints WebPGetDemuxVersion()
//   anim MODE THREADS RESET DUMP PARTIAL PATH
//       decodes the file at PATH (the rest of the line) with
//       WebPAnimDecoderNew(). MODE is the color mode, or -1 for NULL options;
//       THREADS is use_threads. The probe prints WebPAnimDecoderGetInfo(), the
//       features of the decoder's demuxer (WebPAnimDecoderGetDemuxer() with
//       WebPDemuxGetI()) and its ICCP, EXIF and XMP chunks, then
//       WebPAnimDecoderHasMoreFrames() before every WebPAnimDecoderGetNext()
//       and, for each frame, the result, the timestamp and the FNV-1a digest
//       of the whole canvas. After frame DUMP of the first pass (0: none) it
//       prints the canvas in hex. After frame RESET of the first pass (0: none)
//       it calls WebPAnimDecoderReset() and decodes every frame again. After
//       a failed WebPAnimDecoderGetNext() and with PARTIAL set, it prints the
//       digest of the canvas that the last successful call returned: what the
//       failed call left in it. At the end it prints HasMoreFrames() and one
//       more GetNext().
//   nulls PATH
//       calls every function of the animation decoder and the demuxer with
//       NULL and invalid arguments around the file at PATH, and prints the
//       results; pointers are printed as their relation to the input
//
// Pointer values are not compared (a check that does not apply): the probes
// print positions relative to the input and whether private_ is the
// demuxer, never addresses.
//
// Each command starts with a `command=N` line. tests/wp14_anim_decode.rs
// generates the commands and reproduces the anim output with the Rust
// translation; tests/wp14_abi.rs compares this probe with anim_rs.

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "src/webp/decode.h"
#include "src/webp/demux.h"
#include "probe_support.h"

// The 64-bit FNV-1a digest, as tests/wp8_demux.rs computes it.
static unsigned long long Digest(const uint8_t* data, size_t size) {
  unsigned long long hash = 0xcbf29ce484222325ULL;
  size_t i;
  for (i = 0; i < size; ++i) {
    hash ^= (unsigned long long)data[i];
    hash *= 0x100000001b3ULL;
  }
  return hash;
}

static uint8_t* ReadFile(const char* path, size_t* size) {
  return ProbeReadInput(path, size);
}

// The features that the probe asks the demuxer for, with two unknown values.
static const int kFeatures[] = { 0, 1, 2, 3, 4, 5, 6, 99 };
#define NUM_FEATURES ((int)(sizeof(kFeatures) / sizeof(kFeatures[0])))
static const char* const kMetadata[] = { "ICCP", "EXIF", "XMP " };

static void PrintDemuxer(const WebPDemuxer* dmux) {
  int i;
  printf("demux=%d\n", dmux != NULL);
  if (dmux == NULL) return;
  for (i = 0; i < NUM_FEATURES; ++i) {
    printf("demux.feature%d=%u\n", kFeatures[i],
           WebPDemuxGetI(dmux, (WebPFormatFeature)kFeatures[i]));
  }
  for (i = 0; i < 3; ++i) {
    WebPChunkIterator it;
    int count = 0;
    if (WebPDemuxGetChunk(dmux, kMetadata[i], 1, &it)) {
      do {
        printf("demux.meta%d.%d=%d,%d,%lu,%016llx\n", i, count, it.chunk_num,
               it.num_chunks, (unsigned long)it.chunk.size,
               Digest(it.chunk.bytes, it.chunk.size));
        ++count;
      } while (WebPDemuxNextChunk(&it));
      WebPDemuxReleaseChunkIterator(&it);
    }
    printf("demux.meta%d.count=%d\n", i, count);
  }
}

static void Anim(int mode, int threads, int reset, int dump, int partial,
                 const char* path) {
  size_t size;
  uint8_t* const bytes = ReadFile(path, &size);
  WebPData data;
  WebPAnimDecoderOptions options;
  WebPAnimDecoder* dec;
  WebPAnimInfo info;
  size_t canvas_size;
  const uint8_t* last = NULL;
  int pass = 0, frame = 0, failed = 0;

  data.bytes = bytes;
  data.size = size;
  if (mode >= 0) {
    if (!WebPAnimDecoderOptionsInit(&options)) exit(2);
    options.color_mode = (WEBP_CSP_MODE)mode;
    options.use_threads = threads;
  }
  dec = WebPAnimDecoderNew(&data, mode >= 0 ? &options : NULL);
  printf("new=%d\n", dec != NULL);
  if (dec == NULL) {
    free(bytes);
    return;
  }
  if (!WebPAnimDecoderGetInfo(dec, &info)) exit(2);
  printf("info=%u,%u,%u,%u,%u,%u,%u,%u,%u\n", info.canvas_width,
         info.canvas_height, info.loop_count, info.bgcolor, info.frame_count,
         info.pad[0], info.pad[1], info.pad[2], info.pad[3]);
  PrintDemuxer(WebPAnimDecoderGetDemuxer(dec));
  canvas_size = (size_t)info.canvas_width * 4 * info.canvas_height;

  for (;;) {
    uint8_t* buf;
    int timestamp;
    const int more = WebPAnimDecoderHasMoreFrames(dec);
    printf("pass.%d.%d.more=%d\n", pass, frame + 1, more);
    if (!more) break;
    ++frame;
    if (!WebPAnimDecoderGetNext(dec, &buf, &timestamp)) {
      printf("pass.%d.%d.ok=0\n", pass, frame);
      if (partial && last != NULL) {
        printf("pass.%d.%d.partial=%016llx\n", pass, frame,
               Digest(last, canvas_size));
      }
      failed = 1;
      break;
    }
    printf("pass.%d.%d.ok=1\n", pass, frame);
    printf("pass.%d.%d.timestamp=%d\n", pass, frame, timestamp);
    printf("pass.%d.%d.digest=%016llx\n", pass, frame,
           Digest(buf, canvas_size));
    if (pass == 0 && frame == dump) {
      char key[64];
      snprintf(key, sizeof(key), "pass.%d.%d.canvas", pass, frame);
      ProbePrintHex(key, buf, canvas_size);
    }
    last = buf;
    if (pass == 0 && frame == reset) {
      WebPAnimDecoderReset(dec);
      printf("reset=%d\n", WebPAnimDecoderHasMoreFrames(dec));
      pass = 1;
      frame = 0;
    }
  }
  printf("failed=%d\n", failed);
  {
    uint8_t* buf;
    int timestamp;
    const int more = WebPAnimDecoderHasMoreFrames(dec);
    const int ok = WebPAnimDecoderGetNext(dec, &buf, &timestamp);
    printf("end.more=%d\n", more);
    printf("end.next=%d\n", ok);
    if (ok) {
      printf("end.timestamp=%d\n", timestamp);
      printf("end.digest=%016llx\n", Digest(buf, canvas_size));
    }
  }
  WebPAnimDecoderDelete(dec);
  free(bytes);
}

static void PrintCIterator(const char* key, const WebPIterator* it,
                           const WebPDemuxer* dmux, const uint8_t* bytes) {
  printf("%s=%d,%d,%d,%d,%d,%d,%d,%d,%d,%d,%d,%lu,%lu,%lu,%ld,%lu,%d\n", key,
         it->frame_num, it->num_frames, it->x_offset, it->y_offset, it->width,
         it->height, it->duration, (int)it->dispose_method, it->complete,
         it->has_alpha, (int)it->blend_method, (unsigned long)it->pad[0],
         (unsigned long)it->pad[1], (unsigned long)(it->fragment.bytes == NULL),
         it->fragment.bytes == NULL ? -1L
                                    : (long)(it->fragment.bytes - bytes),
         (unsigned long)it->fragment.size, it->private_ == (void*)dmux);
}

static void PrintCChunk(const char* key, const WebPChunkIterator* it,
                        const WebPDemuxer* dmux, const uint8_t* bytes) {
  int i;
  printf("%s=%d,%d,%ld,%lu,%d,", key, it->chunk_num, it->num_chunks,
         it->chunk.bytes == NULL ? -1L : (long)(it->chunk.bytes - bytes),
         (unsigned long)it->chunk.size, it->private_ == (void*)dmux);
  for (i = 0; i < 6; ++i) printf("%lu%s", (unsigned long)it->pad[i], i < 5 ? "," : "\n");
}

// The bytes of the color mode, which may hold a pattern that is no enumerator.
static unsigned int RawMode(const WebPAnimDecoderOptions* options) {
  unsigned int raw;
  memcpy(&raw, &options->color_mode, sizeof(raw));
  return raw;
}

static void Nulls(const char* path) {
  size_t size;
  uint8_t* const bytes = ReadFile(path, &size);
  WebPData data, null_bytes, empty;
  WebPAnimDecoderOptions options;
  WebPAnimDecoder* dec;
  WebPDemuxer* dmux;
  WebPDemuxState state;
  WebPAnimInfo info;
  WebPIterator it;
  WebPChunkIterator chunk;
  uint8_t* buf = NULL;
  int timestamp = -7;

  data.bytes = bytes;
  data.size = size;
  null_bytes.bytes = NULL;
  null_bytes.size = size;
  empty.bytes = bytes;
  empty.size = 0;

  // WebPAnimDecoderOptionsInitInternal() writes two fields and keeps the rest.
  memset(&options, 0xA5, sizeof(options));
  printf("opts.null=%d\n",
         WebPAnimDecoderOptionsInitInternal(NULL, WEBP_DEMUX_ABI_VERSION));
  printf("opts.major=%d\n", WebPAnimDecoderOptionsInitInternal(
                                &options, WEBP_DEMUX_ABI_VERSION + 0x100));
  printf("opts.after_major=%u,%d,%u\n", RawMode(&options), options.use_threads,
         options.padding[0]);
  printf("opts.minor=%d\n", WebPAnimDecoderOptionsInitInternal(
                                &options, WEBP_DEMUX_ABI_VERSION + 1));
  printf("opts.after_minor=%u,%d,%u,%u\n", RawMode(&options),
         options.use_threads, options.padding[0], options.padding[6]);

  printf("new.null=%d\n", WebPAnimDecoderNewInternal(
                              NULL, NULL, WEBP_DEMUX_ABI_VERSION) != NULL);
  printf("new.major=%d\n", WebPAnimDecoderNewInternal(
                               &data, NULL, WEBP_DEMUX_ABI_VERSION + 0x100) != NULL);
  printf("new.null_bytes=%d\n", WebPAnimDecoderNewInternal(
                                    &null_bytes, NULL, WEBP_DEMUX_ABI_VERSION) != NULL);
  printf("new.empty=%d\n", WebPAnimDecoderNewInternal(
                               &empty, NULL, WEBP_DEMUX_ABI_VERSION) != NULL);
  options.color_mode = MODE_RGB;
  printf("new.mode_rgb=%d\n", WebPAnimDecoderNew(&data, &options) != NULL);
  options.color_mode = MODE_YUV;
  printf("new.mode_yuv=%d\n", WebPAnimDecoderNew(&data, &options) != NULL);
  // The padding keeps its pattern; WebPAnimDecoderNew() does not read it.
  options.color_mode = MODE_bgrA;
  options.use_threads = 1;
  dec = WebPAnimDecoderNewInternal(&data, &options, WEBP_DEMUX_ABI_VERSION + 1);
  printf("new.minor=%d\n", dec != NULL);
  if (dec == NULL) exit(2);

  printf("info.null_dec=%d\n", WebPAnimDecoderGetInfo(NULL, &info));
  printf("info.null_info=%d\n", WebPAnimDecoderGetInfo(dec, NULL));
  printf("next.null_dec=%d\n", WebPAnimDecoderGetNext(NULL, &buf, &timestamp));
  printf("next.null_buf=%d\n", WebPAnimDecoderGetNext(dec, NULL, &timestamp));
  printf("next.null_ts=%d\n", WebPAnimDecoderGetNext(dec, &buf, NULL));
  printf("next.untouched=%d,%d\n", buf == NULL, timestamp);
  printf("more.null=%d\n", WebPAnimDecoderHasMoreFrames(NULL));
  printf("more.dec=%d\n", WebPAnimDecoderHasMoreFrames(dec));
  WebPAnimDecoderReset(NULL);
  printf("demuxer.null=%d\n", WebPAnimDecoderGetDemuxer(NULL) == NULL);
  printf("demuxer.dec=%d\n", WebPAnimDecoderGetDemuxer(dec) != NULL);
  WebPAnimDecoderDelete(NULL);
  WebPAnimDecoderDelete(dec);

  state = (WebPDemuxState)7;
  printf("demux.null=%d\n",
         WebPDemuxInternal(NULL, 0, &state, WEBP_DEMUX_ABI_VERSION) != NULL);
  printf("demux.null.state=%d\n", (int)state);
  state = (WebPDemuxState)7;
  printf("demux.null_bytes=%d\n", WebPDemuxInternal(&null_bytes, 1, &state,
                                                    WEBP_DEMUX_ABI_VERSION) != NULL);
  printf("demux.null_bytes.state=%d\n", (int)state);
  state = (WebPDemuxState)7;
  printf("demux.empty=%d\n",
         WebPDemuxInternal(&empty, 1, &state, WEBP_DEMUX_ABI_VERSION) != NULL);
  printf("demux.empty.state=%d\n", (int)state);
  state = (WebPDemuxState)7;
  printf("demux.major=%d\n", WebPDemuxInternal(&data, 0, &state,
                                               WEBP_DEMUX_ABI_VERSION + 0x100) != NULL);
  printf("demux.major.state=%d\n", (int)state);
  dmux = WebPDemuxInternal(&data, 0, NULL, WEBP_DEMUX_ABI_VERSION + 1);
  printf("demux.minor=%d\n", dmux != NULL);
  if (dmux == NULL) exit(2);

  printf("geti.null=%u\n", WebPDemuxGetI(NULL, WEBP_FF_CANVAS_WIDTH));

  memset(&it, 0x5A, sizeof(it));
  printf("frame.null_dmux=%d\n", WebPDemuxGetFrame(NULL, 1, &it));
  PrintCIterator("frame.null_dmux.iter", &it, NULL, bytes);
  printf("frame.null_iter=%d\n", WebPDemuxGetFrame(dmux, 1, NULL));
  memset(&it, 0x5A, sizeof(it));
  printf("frame.negative=%d\n", WebPDemuxGetFrame(dmux, -1, &it));
  PrintCIterator("frame.negative.iter", &it, dmux, bytes);
  memset(&it, 0x5A, sizeof(it));
  printf("frame.missing=%d\n", WebPDemuxGetFrame(dmux, 99, &it));
  PrintCIterator("frame.missing.iter", &it, dmux, bytes);
  // A zeroed iterator with private_ set: WebPDemuxNextFrame() gives frame 1.
  printf("frame.missing.next=%d\n", WebPDemuxNextFrame(&it));
  PrintCIterator("frame.missing.next.iter", &it, dmux, bytes);
  printf("frame.missing.prev=%d\n", WebPDemuxPrevFrame(&it));
  // frame_num -1: the next frame is SetFrame(0), the last frame.
  it.frame_num = -1;
  printf("frame.minus_one.next=%d\n", WebPDemuxNextFrame(&it));
  PrintCIterator("frame.minus_one.next.iter", &it, dmux, bytes);
  it.frame_num = 1000;
  printf("frame.beyond.next=%d\n", WebPDemuxNextFrame(&it));
  printf("frame.beyond.prev=%d\n", WebPDemuxPrevFrame(&it));
  PrintCIterator("frame.beyond.iter", &it, dmux, bytes);
  it.private_ = NULL;
  printf("frame.null_private.next=%d\n", WebPDemuxNextFrame(&it));
  printf("frame.null_private.prev=%d\n", WebPDemuxPrevFrame(&it));
  printf("frame.next_null=%d\n", WebPDemuxNextFrame(NULL));
  printf("frame.prev_null=%d\n", WebPDemuxPrevFrame(NULL));
  WebPDemuxReleaseIterator(NULL);
  WebPDemuxReleaseIterator(&it);

  memset(&chunk, 0x5A, sizeof(chunk));
  printf("chunk.null_fourcc=%d\n", WebPDemuxGetChunk(dmux, NULL, 1, &chunk));
  PrintCChunk("chunk.null_fourcc.iter", &chunk, dmux, bytes);
  memset(&chunk, 0x5A, sizeof(chunk));
  printf("chunk.null_dmux=%d\n", WebPDemuxGetChunk(NULL, "ICCP", 1, &chunk));
  PrintCChunk("chunk.null_dmux.iter", &chunk, NULL, bytes);
  // WebPDemuxNextChunk() on these zeroed iterators would compute the tag
  // address from chunk.bytes == NULL, which is undefined; it is not called.
  printf("chunk.null_iter=%d\n", WebPDemuxGetChunk(dmux, "ICCP", 1, NULL));
  memset(&chunk, 0x5A, sizeof(chunk));
  printf("chunk.negative=%d\n", WebPDemuxGetChunk(dmux, "ICCP", -1, &chunk));
  PrintCChunk("chunk.negative.iter", &chunk, dmux, bytes);
  memset(&chunk, 0x5A, sizeof(chunk));
  {
    // The last EXIF chunk, then a walk back and forth.
    const int ok = WebPDemuxGetChunk(dmux, "EXIF", 0, &chunk);
    printf("chunk.exif=%d\n", ok);
    PrintCChunk("chunk.exif.iter", &chunk, dmux, bytes);
    if (ok) {
      printf("chunk.exif.prev=%d\n", WebPDemuxPrevChunk(&chunk));
      PrintCChunk("chunk.exif.prev.iter", &chunk, dmux, bytes);
      printf("chunk.exif.next=%d\n", WebPDemuxNextChunk(&chunk));
      PrintCChunk("chunk.exif.next.iter", &chunk, dmux, bytes);
      printf("chunk.exif.next2=%d\n", WebPDemuxNextChunk(&chunk));
      chunk.chunk_num = -1;
      printf("chunk.exif.minus_one.next=%d\n", WebPDemuxNextChunk(&chunk));
      PrintCChunk("chunk.exif.minus_one.iter", &chunk, dmux, bytes);
      chunk.private_ = NULL;
      printf("chunk.null_private.next=%d\n", WebPDemuxNextChunk(&chunk));
      printf("chunk.null_private.prev=%d\n", WebPDemuxPrevChunk(&chunk));
    }
  }
  printf("chunk.next_null=%d\n", WebPDemuxNextChunk(NULL));
  printf("chunk.prev_null=%d\n", WebPDemuxPrevChunk(NULL));
  WebPDemuxReleaseChunkIterator(NULL);
  WebPDemuxReleaseChunkIterator(&chunk);

  WebPDemuxDelete(NULL);
  WebPDemuxDelete(dmux);
  free(bytes);
}

// The rest of the line after the current position, without the newline.
static char* Rest(char* line) {
  size_t length;
  while (*line == ' ' || *line == '\t') ++line;
  length = strlen(line);
  while (length > 0 && (line[length - 1] == '\n' || line[length - 1] == '\r')) {
    line[--length] = '\0';
  }
  return line;
}

int main(int argc, char* argv[]) {
  static char line[8192];
  int command = 0;
  (void)argv;
  ProbeCheckArgs(argc, 0, 0, "anim < commands");
  while (fgets(line, sizeof(line), stdin) != NULL) {
    char name[16];
    int consumed = 0;
    if (sscanf(line, "%15s%n", name, &consumed) != 1) continue;
    printf("command=%d\n", command++);
    if (!strcmp(name, "version")) {
      printf("version=%d\n", WebPGetDemuxVersion());
    } else if (!strcmp(name, "anim")) {
      int mode, threads, reset, dump, partial, more = 0;
      if (sscanf(line + consumed, "%d %d %d %d %d%n", &mode, &threads, &reset,
                 &dump, &partial, &more) != 5) {
        exit(2);
      }
      Anim(mode, threads, reset, dump, partial, Rest(line + consumed + more));
    } else if (!strcmp(name, "nulls")) {
      Nulls(Rest(line + consumed));
    } else {
      fprintf(stderr, "unknown command %s\n", name);
      exit(2);
    }
    fflush(stdout);
  }
  return 0;
}
