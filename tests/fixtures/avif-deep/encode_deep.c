// The C original's encode of deep RGBA rows, the reference of
// `ferrisma-avif-write`'s deep path (ADR-0021 amendment of 2026-11-15).
//
// It makes the calls of crates/ferrisma-avif-write/src/lib.rs, in its order
// and with its values: avifImageCreate at the depth, CICP 1/13/6 and full
// range, avifRGBImage RGBA at the same depth with uint16_t samples,
// avifImageRGBToYUV, then avifEncoderWrite with the AOM codec, quality,
// qualityAlpha, speed, one thread and no automatic tiling. Linked to the
// pinned C libavif 1.4.2 and the portable libaom reference
// (codecs/libavif-rs `tools/build_aom.py --library-only`, AOM_TARGET_CPU=
// generic, -ffp-contract=off); see README.md.
//
//   encode_deep in.rgba16 out.avif WIDTH HEIGHT DEPTH 444|420 ALPHA(0|1) QUALITY SPEED
//
// in.rgba16: WIDTH*HEIGHT*4 little-endian uint16 samples, RGBA, top to bottom.
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>

#include "avif/avif.h"

static void fail(const char * message)
{
    fprintf(stderr, "encode_deep: %s\n", message);
    exit(1);
}

int main(int argc, char ** argv)
{
    if (argc != 10) {
        fail("usage: encode_deep in.rgba16 out.avif WIDTH HEIGHT DEPTH 444|420 ALPHA QUALITY SPEED");
    }
    uint32_t width = (uint32_t)atoi(argv[3]), height = (uint32_t)atoi(argv[4]);
    uint32_t depth = (uint32_t)atoi(argv[5]);
    int yuv444 = atoi(argv[6]) == 444, alpha = atoi(argv[7]);
    int quality = atoi(argv[8]), speed = atoi(argv[9]);
    size_t samples = (size_t)width * height * 4;
    uint16_t * pixels = malloc(samples * 2);
    FILE * in = fopen(argv[1], "rb");
    if (!in || !pixels) {
        fail("read input");
    }
    uint8_t pair[2];
    for (size_t i = 0; i < samples; ++i) {
        if (fread(pair, 1, 2, in) != 2) {
            fail("short input");
        }
        pixels[i] = (uint16_t)(pair[0] | (pair[1] << 8));
    }
    fclose(in);
    avifImage * image = avifImageCreate(width, height, depth, yuv444 ? AVIF_PIXEL_FORMAT_YUV444 : AVIF_PIXEL_FORMAT_YUV420);
    if (!image) {
        fail("avifImageCreate");
    }
    image->colorPrimaries = 1;
    image->transferCharacteristics = 13;
    image->matrixCoefficients = 6;
    image->yuvRange = AVIF_RANGE_FULL;
    avifRGBImage rgb;
    avifRGBImageSetDefaults(&rgb, image);
    rgb.format = AVIF_RGB_FORMAT_RGBA;
    rgb.depth = depth;
    rgb.ignoreAlpha = alpha ? AVIF_FALSE : AVIF_TRUE;
    rgb.alphaPremultiplied = AVIF_FALSE;
    rgb.pixels = (uint8_t *)pixels;
    rgb.rowBytes = width * 8;
    if (avifImageRGBToYUV(image, &rgb) != AVIF_RESULT_OK) {
        fail("avifImageRGBToYUV");
    }
    avifEncoder * encoder = avifEncoderCreate();
    if (!encoder) {
        fail("avifEncoderCreate");
    }
    encoder->codecChoice = AVIF_CODEC_CHOICE_AOM;
    encoder->quality = quality;
    encoder->qualityAlpha = quality;
    encoder->speed = speed;
    encoder->maxThreads = 1;
    encoder->autoTiling = AVIF_FALSE;
    avifRWData output = AVIF_DATA_EMPTY;
    if (avifEncoderWrite(encoder, image, &output) != AVIF_RESULT_OK) {
        fail("avifEncoderWrite");
    }
    FILE * out = fopen(argv[2], "wb");
    if (!out || fwrite(output.data, 1, output.size, out) != output.size || fclose(out) != 0) {
        fail("write output");
    }
    avifRWDataFree(&output);
    avifEncoderDestroy(encoder);
    avifImageDestroy(image);
    free(pixels);
    return 0;
}
