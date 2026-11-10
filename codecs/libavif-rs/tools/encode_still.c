// C counterpart of examples/encode_still.rs for the golden outputs of ADR 0059.
//
// It makes the same libavif calls with the same settings (avifImageRGBToYUV,
// then avifEncoderWrite) and is linked to the original C reference: the
// libavif baseline, the complete portable libaom reference (scalar `_c`
// functions only), and the dav1d and SharpYUV root references, all built by
// `tools/root_pipeline.py --phase references`. tools/check_aom_golden.py
// builds and runs it with `--implementation c`.
//
//   encode_still input.bmp output.avif --quality=56 --speed=6 --threads=1
//       --depth=10 [--yuv444] [--alpha] [--repeat=N] [--report-rtcd]
//
// The input is the BMP subset that examples/encode_still.rs reads: 24-bit
// uncompressed, or 32-bit BI_BITFIELDS with byte-aligned channel masks; with
// `--alpha` the alpha mask of a BITMAPV4HEADER is required.
#include <assert.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

#include "avif/avif.h"

static uint32_t word(const uint8_t * data, size_t offset)
{
    return (uint32_t)data[offset] | ((uint32_t)data[offset + 1] << 8) | ((uint32_t)data[offset + 2] << 16) |
           ((uint32_t)data[offset + 3] << 24);
}

static size_t channel(uint32_t mask)
{
    return mask == 0 ? 4 : (size_t)__builtin_ctz(mask) / 8;
}

static void fail(const char * message)
{
    fprintf(stderr, "encode_still: %s\n", message);
    exit(1);
}

static uint8_t * read_bmp(const char * path, int alpha, uint32_t * width_out, uint32_t * height_out)
{
    FILE * file = fopen(path, "rb");
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
    if (size < 54 || data[0] != 'B' || data[1] != 'M') {
        fail("not a BMP file");
    }
    size_t offset = word(data, 10);
    int32_t width = (int32_t)word(data, 18);
    int32_t height = (int32_t)word(data, 22);
    uint16_t bits = (uint16_t)(data[28] | (data[29] << 8));
    if (bits != 24 && bits != 32) {
        fail("BMP input must have 24 or 32 bits per pixel");
    }
    size_t order[4];
    uint32_t compression = word(data, 30);
    if (compression == 0) {
        order[0] = 2, order[1] = 1, order[2] = 0, order[3] = 3;
    } else if (compression == 3 && bits == 32) {
        order[0] = channel(word(data, 54));
        order[1] = channel(word(data, 58));
        order[2] = channel(word(data, 62));
        order[3] = channel(word(data, 14) >= 56 ? word(data, 66) : 0);
    } else {
        fail("compressed BMP input is not supported");
    }
    if (alpha && !(bits == 32 && compression == 3 && word(data, 14) >= 56 && word(data, 66) != 0)) {
        fail("--alpha requires a 32-bit BMP with an alpha mask");
    }
    size_t channels = alpha ? 4 : 3;
    int top_down = height < 0;
    uint32_t rows = (uint32_t)(height < 0 ? -height : height);
    size_t bytes = bits / 8;
    size_t stride = ((size_t)width * bytes + 3) / 4 * 4;
    uint8_t * pixels = malloc((size_t)width * rows * channels);
    if (!pixels) {
        fail("out of memory");
    }
    uint8_t * out = pixels;
    for (uint32_t y = 0; y < rows; ++y) {
        uint32_t row = top_down ? y : rows - 1 - y;
        const uint8_t * start = data + offset + row * stride;
        for (int32_t x = 0; x < width; ++x) {
            const uint8_t * pixel = start + (size_t)x * bytes;
            for (size_t c = 0; c < channels; ++c) {
                *out++ = pixel[order[c]];
            }
        }
    }
    free(data);
    *width_out = (uint32_t)width;
    *height_out = rows;
    return pixels;
}

int main(int argc, char ** argv)
{
    const char * positional[2];
    int count = 0;
    int quality = 56, speed = 6, threads = 1, yuv444 = 0, alpha = 0, repeat = 1, report_rtcd = 0;
    uint32_t depth = 10;
    for (int i = 1; i < argc; ++i) {
        const char * argument = argv[i];
        if (!strncmp(argument, "--quality=", 10)) {
            quality = atoi(argument + 10);
        } else if (!strncmp(argument, "--speed=", 8)) {
            speed = atoi(argument + 8);
        } else if (!strncmp(argument, "--threads=", 10)) {
            threads = atoi(argument + 10);
        } else if (!strncmp(argument, "--depth=", 8)) {
            depth = (uint32_t)atoi(argument + 8);
        } else if (!strcmp(argument, "--yuv444")) {
            yuv444 = 1;
        } else if (!strcmp(argument, "--alpha")) {
            alpha = 1;
        } else if (!strncmp(argument, "--repeat=", 9)) {
            repeat = atoi(argument + 9);
        } else if (!strcmp(argument, "--report-rtcd")) {
            report_rtcd = 1;
        } else if (!strncmp(argument, "--", 2)) {
            fprintf(stderr, "unknown option %s\n", argument);
            return 1;
        } else if (count < 2) {
            positional[count++] = argument;
        } else {
            count = 3;
        }
    }
    if (count != 2) {
        fail("usage: encode_still input.bmp output.avif [options]");
    }
    if (threads == 0) {
        threads = (int)sysconf(_SC_NPROCESSORS_ONLN);
    }
    uint32_t width, height;
    uint8_t * pixels = read_bmp(positional[0], alpha, &width, &height);
    avifPixelFormat format = yuv444 ? AVIF_PIXEL_FORMAT_YUV444 : AVIF_PIXEL_FORMAT_YUV420;
    avifRWData last = AVIF_DATA_EMPTY;
    for (int round = 0; round < repeat; ++round) {
        avifImage * image = avifImageCreate(width, height, depth, format);
        if (!image) {
            fail("avifImageCreate");
        }
        image->colorPrimaries = 1;
        image->transferCharacteristics = 13;
        image->matrixCoefficients = 6;
        image->yuvRange = AVIF_RANGE_FULL;
        avifRGBImage rgb;
        avifRGBImageSetDefaults(&rgb, image);
        rgb.format = alpha ? AVIF_RGB_FORMAT_RGBA : AVIF_RGB_FORMAT_RGB;
        rgb.depth = 8;
        rgb.alphaPremultiplied = AVIF_FALSE;
        rgb.pixels = pixels;
        rgb.rowBytes = width * (alpha ? 4 : 3);
        if (avifImageRGBToYUV(image, &rgb) != AVIF_RESULT_OK) {
            fail("avifImageRGBToYUV");
        }
        avifEncoder * encoder = avifEncoderCreate();
        if (!encoder) {
            fail("avifEncoderCreate");
        }
        encoder->quality = quality;
        encoder->qualityAlpha = quality;
        encoder->speed = speed;
        encoder->maxThreads = threads;
        encoder->autoTiling = threads != 1;
        avifRWData output = AVIF_DATA_EMPTY;
        if (avifEncoderWrite(encoder, image, &output) != AVIF_RESULT_OK) {
            fail("avifEncoderWrite");
        }
        printf("bytes=%zu\n", output.size);
        avifRWDataFree(&last);
        last = output;
        avifEncoderDestroy(encoder);
        avifImageDestroy(image);
    }
    FILE * file = fopen(positional[1], "wb");
    if (!file || fwrite(last.data, 1, last.size, file) != last.size || fclose(file) != 0) {
        fail("write output");
    }
    avifRWDataFree(&last);
    free(pixels);
    if (report_rtcd) {
        // The libaom reference is built for AOM_TARGET_CPU=generic: every
        // RTCD function is its `_c` implementation.
        printf("rtcd=c(reference)\n");
    }
    return 0;
}
