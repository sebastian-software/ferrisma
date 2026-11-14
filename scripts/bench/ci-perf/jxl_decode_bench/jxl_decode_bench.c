// Decode timing harness for the pinned libjxl, the C side of a pair with
// src/main.rs (the same call sequence through the port's public decoder API).
//
//   jxl_decode_bench in.jxl workers [out.f32]
//
// Reads the file, then times JxlDecoderCreate through the final
// JXL_DEC_SUCCESS: a JxlThreadParallelRunner with `workers` worker threads
// (0: every task on the calling thread, as djxl --num_threads=0), events
// BASIC_INFO | COLOR_ENCODING | FULL_IMAGE, the whole upright image into one
// interleaved buffer of 32-bit float, native (little) endian, color channels
// plus alpha when present, allocated with calloc inside the timed region (as
// DecodeImageJXL allocates its PackedFrame). The output file, when given, is
// written after the timer: the raw samples, for the equivalence check.
// Prints one line: elapsed_ms=<ms> width= height= channels= workers=.
// JXL_BENCH_REPEAT=n (profiling only) repeats the whole timed decode n times
// in the process and reports the last one.
#include <jxl/decode.h>
#include <jxl/thread_parallel_runner.h>
#include <stdio.h>
#include <stdlib.h>
#include <time.h>

static double now_ms(void) {
  struct timespec t;
  clock_gettime(CLOCK_MONOTONIC, &t);
  return t.tv_sec * 1e3 + t.tv_nsec / 1e6;
}

int main(int argc, char** argv) {
  if (argc != 3 && argc != 4) {
    fprintf(stderr, "usage: %s in.jxl workers [out.f32]\n", argv[0]);
    return 2;
  }
  FILE* f = fopen(argv[1], "rb");
  if (!f) return 2;
  fseek(f, 0, SEEK_END);
  long n = ftell(f);
  fseek(f, 0, SEEK_SET);
  unsigned char* data = malloc((size_t)n);
  if (fread(data, 1, (size_t)n, f) != (size_t)n) return 2;
  fclose(f);
  size_t workers = (size_t)atol(argv[2]);
  const char* repeat_env = getenv("JXL_BENCH_REPEAT");
  int repeat = repeat_env ? atoi(repeat_env) : 1;
  double t0 = 0, t1 = 0;
  JxlBasicInfo info;
  JxlPixelFormat format = {0, JXL_TYPE_FLOAT, JXL_NATIVE_ENDIAN, 0};
  float* out = NULL;
  size_t out_bytes = 0;
  for (int r = 0; r < repeat; ++r) {
  t0 = now_ms();
  void* runner = JxlThreadParallelRunnerCreate(NULL, workers);
  JxlDecoder* dec = JxlDecoderCreate(NULL);
  if (!runner || !dec ||
      JxlDecoderSetParallelRunner(dec, JxlThreadParallelRunner, runner) != JXL_DEC_SUCCESS ||
      JxlDecoderSubscribeEvents(dec, JXL_DEC_BASIC_INFO | JXL_DEC_COLOR_ENCODING |
                                         JXL_DEC_FULL_IMAGE) != JXL_DEC_SUCCESS ||
      JxlDecoderSetInput(dec, data, (size_t)n) != JXL_DEC_SUCCESS) {
    fprintf(stderr, "setup failed\n");
    return 1;
  }
  JxlDecoderCloseInput(dec);
  for (;;) {
    JxlDecoderStatus s = JxlDecoderProcessInput(dec);
    if (s == JXL_DEC_BASIC_INFO) {
      if (JxlDecoderGetBasicInfo(dec, &info) != JXL_DEC_SUCCESS) return 1;
      format.num_channels = info.num_color_channels + (info.alpha_bits ? 1 : 0);
    } else if (s == JXL_DEC_COLOR_ENCODING) {
    } else if (s == JXL_DEC_NEED_IMAGE_OUT_BUFFER) {
      if (JxlDecoderImageOutBufferSize(dec, &format, &out_bytes) != JXL_DEC_SUCCESS) return 1;
      free(out);
      out = calloc(out_bytes, 1);
      if (!out || JxlDecoderSetImageOutBuffer(dec, &format, out, out_bytes) != JXL_DEC_SUCCESS)
        return 1;
    } else if (s == JXL_DEC_FULL_IMAGE) {
    } else if (s == JXL_DEC_SUCCESS) {
      break;
    } else {
      fprintf(stderr, "decode failed: status %d\n", (int)s);
      return 1;
    }
  }
  JxlDecoderDestroy(dec);
  JxlThreadParallelRunnerDestroy(runner);
  t1 = now_ms();
  }

  double checksum = 0;
  for (size_t i = 0; i < out_bytes / 4; i += 4099) checksum += out[i];
  printf("elapsed_ms=%.3f width=%u height=%u channels=%u workers=%zu impl=libjxl checksum=%.6f\n",
         t1 - t0, info.xsize, info.ysize, format.num_channels, workers, checksum);
  if (argc == 4) {
    FILE* o = fopen(argv[3], "wb");
    if (!o || fwrite(out, 1, out_bytes, o) != out_bytes) return 1;
    fclose(o);
  }
  free(out);
  free(data);
  return 0;
}
