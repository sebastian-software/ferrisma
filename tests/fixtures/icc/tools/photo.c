// Independent LittleCMS oracle for a real photograph crop (issue #17).
// Build: cc -O2 tests/fixtures/icc/tools/photo.c -o /tmp/ferrisma-icc-photo $(pkg-config --cflags --libs lcms2)
// Run from the repository root: /tmp/ferrisma-icc-photo
// Same settings as generate.c: LittleCMS 2.19, relative colorimetric intent,
// floating-point RGB, no black-point compensation, no cache, and a
// linear-light sRGB destination (sRGB primaries, D65, gamma 1.0).
#include <lcms2.h>
#include <stdio.h>
#include <stdlib.h>

#define WIDTH 96
#define HEIGHT 64
#define SAMPLES (WIDTH * HEIGHT * 3)

static unsigned char rgb8[SAMPLES];
static float input[SAMPLES];
static float output[SAMPLES];

static void read_all(const char *path, void *data, size_t size) {
    FILE *file = fopen(path, "rb");
    if (!file || fread(data, 1, size, file) != size || fclose(file)) exit(1);
}

static void run(const char *profile_path, cmsHPROFILE linear_srgb, const char *out_path) {
    cmsHPROFILE source = cmsOpenProfileFromFile(profile_path, "r");
    if (!source) exit(1);
    cmsHTRANSFORM transform = cmsCreateTransform(
        source, TYPE_RGB_FLT, linear_srgb, TYPE_RGB_FLT,
        INTENT_RELATIVE_COLORIMETRIC, cmsFLAGS_NOCACHE);
    if (!transform) exit(1);
    cmsDoTransform(transform, input, output, WIDTH * HEIGHT);
    FILE *file = fopen(out_path, "wb");
    // Little-endian hosts only (ARM64, x86-64), as the Rust reader expects.
    if (!file || fwrite(output, sizeof(float), SAMPLES, file) != SAMPLES ||
        fclose(file)) exit(1);
    cmsDeleteTransform(transform);
    cmsCloseProfile(source);
}

int main(void) {
    read_all("tests/fixtures/icc/paris-crop.rgb", rgb8, sizeof(rgb8));
    for (size_t i = 0; i < SAMPLES; ++i) input[i] = rgb8[i] / 255.0f;
    cmsCIExyY white = {0.3127, 0.3290, 1.0};
    cmsCIExyYTRIPLE srgb = {{0.64, 0.33, 1.0}, {0.30, 0.60, 1.0},
                            {0.15, 0.06, 1.0}};
    cmsToneCurve *curve = cmsBuildGamma(NULL, 1.0);
    cmsToneCurve *curves[3] = {curve, curve, curve};
    cmsHPROFILE linear_srgb = cmsCreateRGBProfile(&white, &srgb, curves);
    cmsFreeToneCurve(curve);
    if (!linear_srgb) exit(1);
    run("tests/fixtures/icc/paris-srgb.icc", linear_srgb,
        "tests/fixtures/icc/paris-crop-srgb-icc.f32");
    run("tests/fixtures/icc/DisplayP3-v4.icc", linear_srgb,
        "tests/fixtures/icc/paris-crop-as-p3.f32");
    cmsCloseProfile(linear_srgb);
    return 0;
}
