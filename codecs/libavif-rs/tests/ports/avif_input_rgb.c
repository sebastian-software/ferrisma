/* Supplemental pinned-C input oracle for Ferrixel's safe AVIF bridge.
 * Decode image 0 with the AOM reference, nearest chroma upsampling, and
 * straight interleaved RGB(A), writing native-endian 8/16-bit raw samples.
 * This consumer is linked only to the original C libavif and AOM archives.
 */
#include "avif/avif.h"
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
int main(int argc, char **argv) {
    if (argc != 3) return 2;
    FILE *input = fopen(argv[1], "rb");
    if (!input) return 3;
    if (fseek(input, 0, SEEK_END) || ftell(input) < 0) return 4;
    size_t size = (size_t)ftell(input);
    if (fseek(input, 0, SEEK_SET)) return 5;
    uint8_t *bytes = malloc(size);
    if (!bytes || fread(bytes, 1, size, input) != size) return 6;
    fclose(input);
    avifDecoder *decoder = avifDecoderCreate();
    if (!decoder) return 7;
    decoder->codecChoice = AVIF_CODEC_CHOICE_AOM;
    decoder->requestedSource = AVIF_DECODER_SOURCE_PRIMARY_ITEM;
    decoder->imageSizeLimit = UINT32_MAX;
    decoder->imageCountLimit = 1;
    avifResult result = avifDecoderSetIOMemory(decoder, bytes, size);
    if (result != AVIF_RESULT_OK) return 8;
    result = avifDecoderParse(decoder);
    if (result != AVIF_RESULT_OK) { fprintf(stderr, "parse: %s\n", avifResultToString(result)); return 9; }
    if (decoder->imageCount != 1) return 10;
    result = avifDecoderNthImage(decoder, 0);
    if (result != AVIF_RESULT_OK) { fprintf(stderr, "decode: %s\n", avifResultToString(result)); return 11; }
    avifRGBImage rgb;
    avifRGBImageSetDefaults(&rgb, decoder->image);
    rgb.depth = decoder->image->depth <= 8 ? 8 : 16;
    rgb.format = decoder->alphaPresent ? AVIF_RGB_FORMAT_RGBA : AVIF_RGB_FORMAT_RGB;
    rgb.alphaPremultiplied = AVIF_FALSE;
    rgb.chromaUpsampling = AVIF_CHROMA_UPSAMPLING_NEAREST;
    unsigned channels = decoder->alphaPresent ? 4 : 3;
    size_t row_bytes = (size_t)rgb.width * channels * (rgb.depth == 8 ? 1 : 2);
    size_t total = row_bytes * rgb.height;
    if (row_bytes > UINT32_MAX || total / row_bytes != rgb.height) return 12;
    rgb.rowBytes = (uint32_t)row_bytes;
    rgb.pixels = malloc(total);
    if (!rgb.pixels) return 13;
    result = avifImageYUVToRGB(decoder->image, &rgb);
    if (result != AVIF_RESULT_OK) { fprintf(stderr, "convert: %s\n", avifResultToString(result)); return 14; }
    FILE *output = fopen(argv[2], "wb");
    if (!output || fwrite(rgb.pixels, 1, total, output) != total) return 15;
    fclose(output);
    fprintf(stderr, "%ux%u depth=%u alpha=%d bytes=%zu\n", rgb.width, rgb.height, rgb.depth, decoder->alphaPresent, total);
    free(rgb.pixels);
    avifDecoderDestroy(decoder);
    free(bytes);
    return 0;
}
