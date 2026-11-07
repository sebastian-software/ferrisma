// WP9 probe: WebPConfigInitInternal, the WebPConfigInit and WebPConfigPreset
// wrappers, WebPConfigLosslessPreset and WebPValidateConfig (src/enc/config_enc.c).
// Commands, one per line on standard input:
//
//   init VERSION PRESET QUALITY_BITS   WebPConfigInitInternal on a config filled
//                                      with 0xa5 bytes
//   preset PRESET QUALITY_BITS         WebPConfigPreset (the inline wrapper)
//   default                            WebPConfigInit (the inline wrapper)
//   lossless LEVEL                     WebPConfigInit, then WebPConfigLosslessPreset
//   validate V1 ... V29                fills the fields in struct order, with the two
//                                      floats as their IEEE bits, then validates
//
// Each command prints `ret=` and every field of the config.

#include <stdint.h>
#include <stdio.h>
#include <string.h>

#include "src/webp/encode.h"
#include "wp9_support.h"

static uint32_t FloatBits(float value) {
  uint32_t bits;
  memcpy(&bits, &value, sizeof(bits));
  return bits;
}

static void PrintConfig(const WebPConfig* config) {
  printf("lossless=%d\n", config->lossless);
  printf("quality=%08x\n", FloatBits(config->quality));
  printf("method=%d\n", config->method);
  printf("image_hint=%u\n", (unsigned)config->image_hint);
  printf("target_size=%d\n", config->target_size);
  printf("target_psnr=%08x\n", FloatBits(config->target_PSNR));
  printf("segments=%d\n", config->segments);
  printf("sns_strength=%d\n", config->sns_strength);
  printf("filter_strength=%d\n", config->filter_strength);
  printf("filter_sharpness=%d\n", config->filter_sharpness);
  printf("filter_type=%d\n", config->filter_type);
  printf("autofilter=%d\n", config->autofilter);
  printf("alpha_compression=%d\n", config->alpha_compression);
  printf("alpha_filtering=%d\n", config->alpha_filtering);
  printf("alpha_quality=%d\n", config->alpha_quality);
  printf("pass=%d\n", config->pass);
  printf("show_compressed=%d\n", config->show_compressed);
  printf("preprocessing=%d\n", config->preprocessing);
  printf("partitions=%d\n", config->partitions);
  printf("partition_limit=%d\n", config->partition_limit);
  printf("emulate_jpeg_size=%d\n", config->emulate_jpeg_size);
  printf("thread_level=%d\n", config->thread_level);
  printf("low_memory=%d\n", config->low_memory);
  printf("near_lossless=%d\n", config->near_lossless);
  printf("exact=%d\n", config->exact);
  printf("use_delta_palette=%d\n", config->use_delta_palette);
  printf("use_sharp_yuv=%d\n", config->use_sharp_yuv);
  printf("qmin=%d\n", config->qmin);
  printf("qmax=%d\n", config->qmax);
}

static void Reset(WebPConfig* config) { memset(config, 0xa5, sizeof(*config)); }

int main(int argc, char* argv[]) {
  char command[32];
  (void)argc;
  (void)argv;
  while (scanf("%31s", command) == 1) {
    WebPConfig config;
    int ret = 0;
    if (strcmp(command, "init") == 0) {
      char version[32], preset[32], quality[32];
      if (scanf("%31s %31s %31s", version, preset, quality) != 3) return 2;
      Reset(&config);
      ret = WebPConfigInitInternal(&config, (WebPPreset)WP9ParseInt(preset),
                                   WP9ParseFloatBits(quality),
                                   (int)WP9ParseInt(version));
      printf("ret=%d\n", ret);
      PrintConfig(&config);
    } else if (strcmp(command, "preset") == 0) {
      char preset[32], quality[32];
      if (scanf("%31s %31s", preset, quality) != 2) return 2;
      Reset(&config);
      ret = WebPConfigPreset(&config, (WebPPreset)WP9ParseInt(preset),
                             WP9ParseFloatBits(quality));
      printf("ret=%d\n", ret);
      PrintConfig(&config);
    } else if (strcmp(command, "default") == 0) {
      Reset(&config);
      ret = WebPConfigInit(&config);
      printf("ret=%d\n", ret);
      PrintConfig(&config);
    } else if (strcmp(command, "lossless") == 0) {
      char level[32];
      if (scanf("%31s", level) != 1) return 2;
      Reset(&config);
      (void)WebPConfigInit(&config);
      ret = WebPConfigLosslessPreset(&config, (int)WP9ParseInt(level));
      printf("ret=%d\n", ret);
      PrintConfig(&config);
    } else if (strcmp(command, "validate") == 0) {
      char text[29][32];
      int i;
      for (i = 0; i < 29; ++i) {
        if (scanf("%31s", text[i]) != 1) return 2;
      }
      Reset(&config);
      config.lossless = (int)WP9ParseInt(text[0]);
      config.quality = WP9ParseFloatBits(text[1]);
      config.method = (int)WP9ParseInt(text[2]);
      config.image_hint = (WebPImageHint)WP9ParseInt(text[3]);
      config.target_size = (int)WP9ParseInt(text[4]);
      config.target_PSNR = WP9ParseFloatBits(text[5]);
      config.segments = (int)WP9ParseInt(text[6]);
      config.sns_strength = (int)WP9ParseInt(text[7]);
      config.filter_strength = (int)WP9ParseInt(text[8]);
      config.filter_sharpness = (int)WP9ParseInt(text[9]);
      config.filter_type = (int)WP9ParseInt(text[10]);
      config.autofilter = (int)WP9ParseInt(text[11]);
      config.alpha_compression = (int)WP9ParseInt(text[12]);
      config.alpha_filtering = (int)WP9ParseInt(text[13]);
      config.alpha_quality = (int)WP9ParseInt(text[14]);
      config.pass = (int)WP9ParseInt(text[15]);
      config.show_compressed = (int)WP9ParseInt(text[16]);
      config.preprocessing = (int)WP9ParseInt(text[17]);
      config.partitions = (int)WP9ParseInt(text[18]);
      config.partition_limit = (int)WP9ParseInt(text[19]);
      config.emulate_jpeg_size = (int)WP9ParseInt(text[20]);
      config.thread_level = (int)WP9ParseInt(text[21]);
      config.low_memory = (int)WP9ParseInt(text[22]);
      config.near_lossless = (int)WP9ParseInt(text[23]);
      config.exact = (int)WP9ParseInt(text[24]);
      config.use_delta_palette = (int)WP9ParseInt(text[25]);
      config.use_sharp_yuv = (int)WP9ParseInt(text[26]);
      config.qmin = (int)WP9ParseInt(text[27]);
      config.qmax = (int)WP9ParseInt(text[28]);
      ret = WebPValidateConfig(&config);
      printf("ret=%d\n", ret);
    } else {
      fprintf(stderr, "config: unknown command %s\n", command);
      return 2;
    }
  }
  return 0;
}
