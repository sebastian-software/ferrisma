// Regenerate the committed ICC profiles and independent linear-sRGB oracle.
// Build: cc -O2 generate.c -o generate $(pkg-config --cflags --libs lcms2)
// Run from the repository root: /private/tmp/ferrisma-icc-oracle
// LittleCMS 2.19, relative colorimetric intent, floating-point RGB, no BPC.
#include <lcms2.h>
#include <stdio.h>
#include <stdlib.h>

static const float samples[][3] = {
    {1.0f, 0.0f, 0.0f}, {0.0f, 1.0f, 0.0f}, {0.0f, 0.0f, 1.0f},
    {0.0f, 1.0f, 1.0f}, {1.0f, 0.0f, 1.0f}, {1.0f, 1.0f, 0.0f},
    {0.2f, 0.4f, 0.6f}, {1.0f, 1.0f, 1.0f},
    {10.0f / 255.0f, 11.0f / 255.0f, 12.0f / 255.0f},
    {11.0f / 255.0f, 10.0f / 255.0f, 9.0f / 255.0f},
};

static cmsHPROFILE rgb_profile(cmsCIExyYTRIPLE primaries, double gamma) {
    cmsCIExyY white = {0.3127, 0.3290, 1.0};
    cmsToneCurve *curve = cmsBuildGamma(NULL, gamma);
    cmsToneCurve *curves[3] = {curve, curve, curve};
    cmsHPROFILE profile = cmsCreateRGBProfile(&white, &primaries, curves);
    cmsFreeToneCurve(curve);
    return profile;
}

static void write_profile(cmsHPROFILE profile, const char *name) {
    char path[128];
    snprintf(path, sizeof(path), "tests/fixtures/icc/%s.icc", name);
    if (!cmsSaveProfileToFile(profile, path)) exit(1);
    // lcms writes wall-clock time into the ICC header; pin it for byte-for-byte
    // regeneration. This header field does not affect the color transform.
    const unsigned char created[12] = {0x07, 0xea, 0x00, 0x0b, 0x00, 0x0a,
                                       0x00, 0x00, 0x00, 0x00, 0x00, 0x00};
    FILE *file = fopen(path, "r+b");
    if (!file || fseek(file, 24, SEEK_SET) ||
        fwrite(created, 1, sizeof(created), file) != sizeof(created) ||
        fclose(file)) exit(1);
}

static void oracle(cmsHPROFILE source, cmsHPROFILE linear_srgb, const char *name) {
    cmsHTRANSFORM transform = cmsCreateTransform(
        source, TYPE_RGB_FLT, linear_srgb, TYPE_RGB_FLT,
        INTENT_RELATIVE_COLORIMETRIC, cmsFLAGS_NOCACHE);
    if (!transform) exit(1);
    float output[sizeof(samples) / sizeof(samples[0])][3];
    cmsDoTransform(transform, samples, output, sizeof(samples) / sizeof(samples[0]));
    printf("%s\n", name);
    for (size_t i = 0; i < sizeof(samples) / sizeof(samples[0]); ++i)
        printf("  [%.9gf32, %.9gf32, %.9gf32],\n",
               output[i][0], output[i][1], output[i][2]);
    cmsDeleteTransform(transform);
}

int main(void) {
    cmsCIExyYTRIPLE srgb = {{0.64, 0.33, 1.0}, {0.30, 0.60, 1.0},
                            {0.15, 0.06, 1.0}};
    cmsCIExyYTRIPLE p3 = {{0.68, 0.32, 1.0}, {0.265, 0.69, 1.0},
                          {0.15, 0.06, 1.0}};
    cmsCIExyYTRIPLE adobe = {{0.64, 0.33, 1.0}, {0.21, 0.71, 1.0},
                             {0.15, 0.06, 1.0}};
    cmsHPROFILE linear_srgb = rgb_profile(srgb, 1.0);
    cmsHPROFILE display_p3 = rgb_profile(p3, 2.2);
    cmsHPROFILE adobe_rgb = rgb_profile(adobe, 2.19921875);
    if (!linear_srgb || !display_p3 || !adobe_rgb) exit(1);
    write_profile(display_p3, "display-p3-gamma22");
    write_profile(adobe_rgb, "adobe-rgb-1998");
    // All four oracles consume the exact serialized bytes used by Rust.
    cmsHPROFILE generated_p3 =
        cmsOpenProfileFromFile("tests/fixtures/icc/display-p3-gamma22.icc", "r");
    cmsHPROFILE generated_adobe =
        cmsOpenProfileFromFile("tests/fixtures/icc/adobe-rgb-1998.icc", "r");
    cmsHPROFILE real_p3 = cmsOpenProfileFromFile("tests/fixtures/icc/DisplayP3-v4.icc", "r");
    cmsHPROFILE real_adobe = cmsOpenProfileFromFile("tests/fixtures/icc/AdobeCompat-v2.icc", "r");
    if (!generated_p3 || !generated_adobe || !real_p3 || !real_adobe) exit(1);
    oracle(generated_p3, linear_srgb, "display-p3-gamma22");
    oracle(generated_adobe, linear_srgb, "adobe-rgb-1998");
    oracle(real_p3, linear_srgb, "DisplayP3-v4");
    oracle(real_adobe, linear_srgb, "AdobeCompat-v2");
    cmsCloseProfile(real_adobe);
    cmsCloseProfile(real_p3);
    cmsCloseProfile(generated_adobe);
    cmsCloseProfile(generated_p3);
    cmsCloseProfile(adobe_rgb);
    cmsCloseProfile(display_p3);
    cmsCloseProfile(linear_srgb);
    return 0;
}
