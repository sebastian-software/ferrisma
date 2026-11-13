// C counterpart of examples/decode_still.rs for the SIMD-against-SIMD gate
// (tools/check_aom_simd_original.py).
//
// It decodes the primary item of one still AVIF with libavif's AOM decoder on
// one thread, the way crates/ferrisma-avif-read does, and writes two raw files:
//
//   decode_still input.avif planes.raw rgb.raw
//
// planes.raw holds the decoded Y, U, V and alpha planes row by row without
// padding (two bytes per sample, native order, above 8 bits); rgb.raw holds
// avifImageYUVToRGB with nearest chroma upsampling, RGB or RGBA (with alpha),
// 8-bit for 8-bit images and 16-bit otherwise. It prints the image layout.
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "avif/avif.h"

static void fail(const char * message)
{
    fprintf(stderr, "decode_still: %s\n", message);
    exit(1);
}

static void write_plane(FILE * file, const uint8_t * plane, uint32_t row_bytes, uint32_t width, uint32_t height, uint32_t bytes)
{
    for (uint32_t y = 0; y < height; ++y) {
        if (fwrite(plane + (size_t)y * row_bytes, 1, (size_t)width * bytes, file) != (size_t)width * bytes) {
            fail("write planes");
        }
    }
}

int main(int argc, char ** argv)
{
    if (argc != 4) {
        fail("usage: decode_still input.avif planes.raw rgb.raw");
    }
    FILE * file = fopen(argv[1], "rb");
    if (!file) {
        fail("read input");
    }
    fseek(file, 0, SEEK_END);
    long size = ftell(file);
    fseek(file, 0, SEEK_SET);
    uint8_t * data = malloc((size_t)size);
    if (!data || fread(data, 1, (size_t)size, file) != (size_t)size) {
        fail("read input");
    }
    fclose(file);

    avifDecoder * decoder = avifDecoderCreate();
    if (!decoder) {
        fail("avifDecoderCreate");
    }
    decoder->codecChoice = AVIF_CODEC_CHOICE_AOM;
    decoder->maxThreads = 1;
    decoder->requestedSource = AVIF_DECODER_SOURCE_PRIMARY_ITEM;
    decoder->imageSizeLimit = 0xFFFFFFFFu;
    decoder->imageCountLimit = 1;
    if (avifDecoderSetIOMemory(decoder, data, (size_t)size) != AVIF_RESULT_OK || avifDecoderParse(decoder) != AVIF_RESULT_OK) {
        fail("parse");
    }
    if (avifDecoderNthImage(decoder, 0) != AVIF_RESULT_OK) {
        fail("decode");
    }
    const avifImage * image = decoder->image;
    uint32_t bytes = image->depth > 8 ? 2 : 1;
    avifPixelFormatInfo info;
    avifGetPixelFormatInfo(image->yuvFormat, &info);
    FILE * planes = fopen(argv[2], "wb");
    if (!planes) {
        fail("write planes");
    }
    write_plane(planes, image->yuvPlanes[AVIF_CHAN_Y], image->yuvRowBytes[AVIF_CHAN_Y], image->width, image->height, bytes);
    if (!info.monochrome) {
        uint32_t width = (image->width + info.chromaShiftX) >> info.chromaShiftX;
        uint32_t height = (image->height + info.chromaShiftY) >> info.chromaShiftY;
        write_plane(planes, image->yuvPlanes[AVIF_CHAN_U], image->yuvRowBytes[AVIF_CHAN_U], width, height, bytes);
        write_plane(planes, image->yuvPlanes[AVIF_CHAN_V], image->yuvRowBytes[AVIF_CHAN_V], width, height, bytes);
    }
    int alpha = image->alphaPlane != NULL;
    if (alpha) {
        write_plane(planes, image->alphaPlane, image->alphaRowBytes, image->width, image->height, bytes);
    }
    if (fclose(planes) != 0) {
        fail("write planes");
    }

    avifRGBImage rgb;
    avifRGBImageSetDefaults(&rgb, image);
    rgb.depth = image->depth > 8 ? 16 : 8;
    rgb.format = alpha ? AVIF_RGB_FORMAT_RGBA : AVIF_RGB_FORMAT_RGB;
    rgb.alphaPremultiplied = AVIF_FALSE;
    rgb.chromaUpsampling = AVIF_CHROMA_UPSAMPLING_NEAREST;
    rgb.maxThreads = 1;
    if (avifRGBImageAllocatePixels(&rgb) != AVIF_RESULT_OK || avifImageYUVToRGB(image, &rgb) != AVIF_RESULT_OK) {
        fail("YUV to RGB");
    }
    FILE * out = fopen(argv[3], "wb");
    if (!out) {
        fail("write rgb");
    }
    write_plane(out, rgb.pixels, rgb.rowBytes, rgb.width, rgb.height, (alpha ? 4 : 3) * (rgb.depth / 8));
    if (fclose(out) != 0) {
        fail("write rgb");
    }
    printf("width=%u height=%u depth=%u format=%d alpha=%d\n", image->width, image->height, image->depth, (int)image->yuvFormat, alpha);
    avifRGBImageFreePixels(&rgb);
    avifDecoderDestroy(decoder);
    free(data);
    return 0;
}
