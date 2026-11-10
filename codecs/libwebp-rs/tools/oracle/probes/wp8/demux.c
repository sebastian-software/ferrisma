// WP8 probe: the demuxer of src/demux/demux.c (ADR 0002). Commands on standard
// input, one per line, tokens separated by white space:
//
//   version                     prints WebPGetDemuxVersion()
//   demux HEX ALLOW             parses the bytes HEX ("-" is empty) through
//                               WebPDemuxInternal with allow_partial ALLOW, then
//                               walks the demuxer and prints every observation
//   file PATH OFFSET SIZE ALLOW the same on SIZE bytes of PATH from OFFSET
//
// For each demux the probe prints the state, whether the public wrapper
// (WebPDemux or WebPDemuxPartial) returns a demuxer, the features of
// WebPDemuxGetI, every frame that WebPDemuxGetFrame finds for frames 0 to
// frame_count + 1 with all iterator fields, the forward walk from frame 1 and the
// backward walk from the last frame, and the same for the chunk tags of
// eleven tags, with the iterator fields. A fragment or payload is printed as its
// size, its 64-bit FNV-1a digest and its first 16 bytes.
//
// tests/wp8_demux.rs generates the commands and the expected output with the
// Rust translation, reproducing the walks of this file.

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "src/webp/demux.h"
#include "probe_support.h"

// The tags that the walks query; "ZZZZ" is never present.
static const char* const kTags[] = {
  "ICCP", "EXIF", "XMP ", "ALPH", "VP8 ", "VP8L", "VP8X", "ANIM", "ANMF", "UNKN",
  "ZZZZ",
};
#define NUM_TAGS ((int)(sizeof(kTags) / sizeof(kTags[0])))

// Positions in the command text.
typedef struct {
  char* text;
  size_t length;
  size_t position;
} Commands;

static char* NextToken(Commands* commands) {
  char* token;
  size_t start, end;
  while (commands->position < commands->length &&
         (commands->text[commands->position] == ' ' ||
          commands->text[commands->position] == '\n' ||
          commands->text[commands->position] == '\t' ||
          commands->text[commands->position] == '\r')) {
    ++commands->position;
  }
  if (commands->position >= commands->length) return NULL;
  start = commands->position;
  while (commands->position < commands->length &&
         commands->text[commands->position] != ' ' &&
         commands->text[commands->position] != '\n' &&
         commands->text[commands->position] != '\t' &&
         commands->text[commands->position] != '\r') {
    ++commands->position;
  }
  end = commands->position;
  token = (char*)malloc(end - start + 1);
  if (token == NULL) exit(2);
  memcpy(token, commands->text + start, end - start);
  token[end - start] = '\0';
  return token;
}

static long NextLong(Commands* commands) {
  char* token = NextToken(commands);
  char* end;
  long value;
  if (token == NULL) exit(2);
  value = strtol(token, &end, 10);
  if (*end != '\0') exit(2);
  free(token);
  return value;
}

// Decodes a hex token into a buffer of at least one byte; "-" is empty.
static uint8_t* NextHexBytes(Commands* commands, size_t* size) {
  char* token = NextToken(commands);
  size_t length, i;
  uint8_t* data;
  if (token == NULL) exit(2);
  length = strcmp(token, "-") == 0 ? 0 : strlen(token) / 2;
  if (length != 0 && strlen(token) != 2 * length) exit(2);
  data = (uint8_t*)malloc(length + 1);
  if (data == NULL) exit(2);
  for (i = 0; i < length; ++i) {
    unsigned int byte;
    if (sscanf(token + 2 * i, "%2x", &byte) != 1) exit(2);
    data[i] = (uint8_t)byte;
  }
  data[length] = 0;
  *size = length;
  free(token);
  return data;
}

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

static void PrintBytes(const char* prefix, const uint8_t* data, size_t size) {
  char key[64];
  size_t head = size < 16 ? size : 16;
  printf("%s.size=%lu\n", prefix, (unsigned long)size);
  printf("%s.digest=%016llx\n", prefix, Digest(data, size));
  snprintf(key, sizeof(key), "%s.head", prefix);
  if (size == 0) {
    printf("%s=\n", key);
  } else {
    ProbePrintHex(key, data, head);
  }
}

static void PrintIterator(const char* prefix, const WebPIterator* it) {
  char key[64];
  printf("%s.frame_num=%d\n", prefix, it->frame_num);
  printf("%s.num_frames=%d\n", prefix, it->num_frames);
  printf("%s.x_offset=%d\n", prefix, it->x_offset);
  printf("%s.y_offset=%d\n", prefix, it->y_offset);
  printf("%s.width=%d\n", prefix, it->width);
  printf("%s.height=%d\n", prefix, it->height);
  printf("%s.duration=%d\n", prefix, it->duration);
  printf("%s.dispose=%d\n", prefix, (int)it->dispose_method);
  printf("%s.complete=%d\n", prefix, it->complete);
  printf("%s.has_alpha=%d\n", prefix, it->has_alpha);
  printf("%s.blend=%d\n", prefix, (int)it->blend_method);
  printf("%s.pad=%lu,%lu\n", prefix, (unsigned long)it->pad[0],
         (unsigned long)it->pad[1]);
  snprintf(key, sizeof(key), "%s.fragment", prefix);
  PrintBytes(key, it->fragment.bytes, it->fragment.size);
}

static void PrintChunk(const char* prefix, const WebPChunkIterator* it) {
  char key[64];
  printf("%s.chunk_num=%d\n", prefix, it->chunk_num);
  printf("%s.num_chunks=%d\n", prefix, it->num_chunks);
  snprintf(key, sizeof(key), "%s.chunk", prefix);
  PrintBytes(key, it->chunk.bytes, it->chunk.size);
}

// Walks the frames of `dmux`: the random access of frames 0 to frame_count + 1,
// the forward walk from frame 1 and the backward walk from the last frame.
static void WalkFrames(const WebPDemuxer* dmux, uint32_t frame_count) {
  int k;
  for (k = 0; k <= (int)frame_count + 1; ++k) {
    WebPIterator it;
    char prefix[64];
    const int ok = WebPDemuxGetFrame(dmux, k, &it);
    printf("get.%d=%d\n", k, ok);
    if (ok) {
      snprintf(prefix, sizeof(prefix), "get.%d", k);
      PrintIterator(prefix, &it);
      WebPDemuxReleaseIterator(&it);
    }
  }
  {
    WebPIterator it;
    int count = 0;
    if (WebPDemuxGetFrame(dmux, 1, &it)) {
      do {
        char prefix[64];
        snprintf(prefix, sizeof(prefix), "fwd.%d", count);
        PrintIterator(prefix, &it);
        ++count;
      } while (WebPDemuxNextFrame(&it));
      WebPDemuxReleaseIterator(&it);
    }
    printf("fwd.count=%d\n", count);
  }
  {
    WebPIterator it;
    int count = 0;
    if (WebPDemuxGetFrame(dmux, 0, &it)) {
      do {
        char prefix[64];
        snprintf(prefix, sizeof(prefix), "back.%d", count);
        PrintIterator(prefix, &it);
        ++count;
      } while (WebPDemuxPrevFrame(&it));
      WebPDemuxReleaseIterator(&it);
    }
    printf("back.count=%d\n", count);
  }
}

// Walks the chunks of each tag like WalkFrames.
static void WalkChunks(const WebPDemuxer* dmux) {
  int tag;
  for (tag = 0; tag < NUM_TAGS; ++tag) {
    int k;
    for (k = 0; k <= 5; ++k) {
      WebPChunkIterator it;
      const int ok = WebPDemuxGetChunk(dmux, kTags[tag], k, &it);
      printf("chunk.%d.get.%d=%d\n", tag, k, ok);
      if (ok) {
        char prefix[64];
        snprintf(prefix, sizeof(prefix), "chunk.%d.get.%d", tag, k);
        PrintChunk(prefix, &it);
        WebPDemuxReleaseChunkIterator(&it);
      }
    }
    {
      WebPChunkIterator it;
      int count = 0;
      if (WebPDemuxGetChunk(dmux, kTags[tag], 1, &it)) {
        do {
          char prefix[64];
          snprintf(prefix, sizeof(prefix), "chunk.%d.fwd.%d", tag, count);
          PrintChunk(prefix, &it);
          ++count;
        } while (WebPDemuxNextChunk(&it));
        WebPDemuxReleaseChunkIterator(&it);
      }
      printf("chunk.%d.fwd.count=%d\n", tag, count);
    }
    {
      WebPChunkIterator it;
      int count = 0;
      if (WebPDemuxGetChunk(dmux, kTags[tag], 0, &it)) {
        do {
          char prefix[64];
          snprintf(prefix, sizeof(prefix), "chunk.%d.back.%d", tag, count);
          PrintChunk(prefix, &it);
          ++count;
        } while (WebPDemuxPrevChunk(&it));
        WebPDemuxReleaseChunkIterator(&it);
      }
      printf("chunk.%d.back.count=%d\n", tag, count);
    }
  }
}

static void Demux(const uint8_t* bytes, size_t size, int allow_partial) {
  WebPData data;
  WebPDemuxState state = WEBP_DEMUX_PARSE_ERROR;
  WebPDemuxState public_state = WEBP_DEMUX_PARSE_ERROR;
  WebPDemuxer* dmux;
  WebPDemuxer* public_dmux;
  uint32_t frame_count;

  data.bytes = bytes;
  data.size = size;
  dmux = WebPDemuxInternal(&data, allow_partial, &state, WEBP_DEMUX_ABI_VERSION);
  printf("state=%d\n", (int)state);

  if (allow_partial) {
    public_dmux = WebPDemuxPartial(&data, &public_state);
    printf("public_state=%d\n", (int)public_state);
  } else {
    public_dmux = WebPDemux(&data);
  }
  printf("public=%d\n", public_dmux != NULL);
  WebPDemuxDelete(public_dmux);

  printf("dmux=%d\n", dmux != NULL);
  if (dmux != NULL) {
    printf("flags=%u\n", WebPDemuxGetI(dmux, WEBP_FF_FORMAT_FLAGS));
    printf("canvas_width=%u\n", WebPDemuxGetI(dmux, WEBP_FF_CANVAS_WIDTH));
    printf("canvas_height=%u\n", WebPDemuxGetI(dmux, WEBP_FF_CANVAS_HEIGHT));
    printf("loop_count=%u\n", WebPDemuxGetI(dmux, WEBP_FF_LOOP_COUNT));
    printf("bgcolor=%u\n", WebPDemuxGetI(dmux, WEBP_FF_BACKGROUND_COLOR));
    frame_count = WebPDemuxGetI(dmux, WEBP_FF_FRAME_COUNT);
    printf("frame_count=%u\n", frame_count);
    printf("feature6=%u\n", WebPDemuxGetI(dmux, (WebPFormatFeature)6));
    printf("feature99=%u\n", WebPDemuxGetI(dmux, (WebPFormatFeature)99));
    WalkFrames(dmux, frame_count);
    WalkChunks(dmux);
  }
  WebPDemuxDelete(dmux);
}

int main(int argc, char* argv[]) {
  Commands commands;
  ProbeCheckArgs(argc, 0, 0, "demux < commands");
  (void)argv;
  commands.text = (char*)ProbeReadInput(NULL, &commands.length);
  commands.position = 0;
  for (;;) {
    char* const token = NextToken(&commands);
    if (token == NULL) break;
    if (strcmp(token, "version") == 0) {
      printf("version=%d\n", WebPGetDemuxVersion());
    } else if (strcmp(token, "demux") == 0) {
      size_t size;
      uint8_t* const bytes = NextHexBytes(&commands, &size);
      const long allow_partial = NextLong(&commands);
      Demux(bytes, size, (int)allow_partial);
      free(bytes);
    } else if (strcmp(token, "file") == 0) {
      char* const path = NextToken(&commands);
      const long offset = NextLong(&commands);
      const long size = NextLong(&commands);
      const long allow_partial = NextLong(&commands);
      size_t length;
      uint8_t* bytes;
      if (path == NULL || offset < 0 || size < 0) exit(2);
      bytes = ProbeReadInput(path, &length);
      if ((size_t)offset + (size_t)size > length) exit(2);
      Demux(bytes + offset, (size_t)size, (int)allow_partial);
      free(bytes);
      free(path);
    } else {
      fprintf(stderr, "wp8 demux: unknown command %s\n", token);
      return 2;
    }
    free(token);
  }
  free(commands.text);
  return 0;
}
