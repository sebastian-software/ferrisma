// Time an AVIF decode through C libavif + libaom the way the Ferrisma bridge
// does: AOM codec, primary item, nearest-neighbor YUV-to-RGB into an RGB8
// (or RGB16 for >8-bit) buffer. Prints decode_us (parse + AV1 decode) and
// rgb_us (conversion), and a FNV-1a checksum of the RGB bytes.
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <time.h>

#include "avif/avif.h"

static long long now_us(void)
{
    struct timespec t;
    clock_gettime(CLOCK_MONOTONIC, &t);
    return (long long)t.tv_sec * 1000000LL + t.tv_nsec / 1000;
}

int main(int argc, char ** argv)
{
    if (argc < 2) {
        fprintf(stderr, "usage: decode_time file.avif [threads]\n");
        return 2;
    }
    int threads = argc > 2 ? atoi(argv[2]) : 1;
    FILE * f = fopen(argv[1], "rb");
    fseek(f, 0, SEEK_END);
    long size = ftell(f);
    fseek(f, 0, SEEK_SET);
    uint8_t * data = malloc(size);
    fread(data, 1, size, f);
    fclose(f);
    long long t0 = now_us();
    avifDecoder * decoder = avifDecoderCreate();
    decoder->codecChoice = AVIF_CODEC_CHOICE_AOM;
    decoder->maxThreads = threads;
    avifImage * image = avifImageCreateEmpty();
    if (avifDecoderReadMemory(decoder, image, data, size) != AVIF_RESULT_OK) {
        fprintf(stderr, "decode failed\n");
        return 1;
    }
    long long t1 = now_us();
    avifRGBImage rgb;
    avifRGBImageSetDefaults(&rgb, image);
    rgb.format = AVIF_RGB_FORMAT_RGB;
    rgb.depth = image->depth > 8 ? 16 : 8;
    rgb.chromaUpsampling = AVIF_CHROMA_UPSAMPLING_NEAREST;
    rgb.maxThreads = 1;
    avifRGBImageAllocatePixels(&rgb);
    if (avifImageYUVToRGB(image, &rgb) != AVIF_RESULT_OK) {
        fprintf(stderr, "convert failed\n");
        return 1;
    }
    long long t2 = now_us();
    uint64_t h = 1469598103934665603ULL;
    for (size_t i = 0; i < (size_t)rgb.rowBytes * rgb.height; ++i) {
        h = (h ^ rgb.pixels[i]) * 1099511628211ULL;
    }
    printf("decode_us=%lld rgb_us=%lld checksum=%016llx\n", t1 - t0, t2 - t1, (unsigned long long)h);
    return 0;
}
