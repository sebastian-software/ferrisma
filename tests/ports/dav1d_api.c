/* Differential consumer of the original public dav1d headers. */
#define _POSIX_C_SOURCE 200112L
#include <dav1d/dav1d.h>
#include <errno.h>
#include <inttypes.h>
#include <stdarg.h>
#include <stdatomic.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define CHECK(condition) do { if (!(condition)) { \
    fprintf(stderr, "API probe failed: %s:%d: %s\n", __FILE__, __LINE__, #condition); \
    exit(1); } } while (0)

static atomic_int data_live, user_live, picture_live;
static atomic_int allocation_attempts;
static int fail_at;

static void free_packet(const uint8_t *data, void *cookie) {
    CHECK(data == cookie);
    free(cookie);
    atomic_fetch_sub(&data_live, 1);
}
static void free_user(const uint8_t *data, void *cookie) {
    CHECK(data == cookie);
    free(cookie);
    atomic_fetch_sub(&user_live, 1);
}
static void no_free(const uint8_t *data, void *cookie) { (void)data; (void)cookie; }

static int allocate_picture(Dav1dPicture *picture, void *cookie) {
    CHECK(cookie == &picture_live);
    const int attempt = atomic_fetch_add(&allocation_attempts, 1) + 1;
    if (attempt == fail_at) return DAV1D_ERR(ENOMEM);
    const int bytes = picture->p.bpc > 8 ? 2 : 1;
    const int width = (picture->p.w + 127) & ~127;
    const int height = (picture->p.h + 127) & ~127;
    const int monochrome = picture->p.layout == DAV1D_PIXEL_LAYOUT_I400;
    const int horizontal = picture->p.layout != DAV1D_PIXEL_LAYOUT_I444;
    const int vertical = picture->p.layout == DAV1D_PIXEL_LAYOUT_I420;
    const size_t luma = (size_t)width * height * bytes;
    const size_t chroma = monochrome ? 0 : luma >> (horizontal + vertical);
    uint8_t *storage = NULL;
    if (posix_memalign((void **)&storage, DAV1D_PICTURE_ALIGNMENT,
                       luma + chroma * 2 + DAV1D_PICTURE_ALIGNMENT)) return DAV1D_ERR(ENOMEM);
    memset(storage, 0, luma + chroma * 2 + DAV1D_PICTURE_ALIGNMENT);
    picture->data[0] = storage;
    picture->data[1] = monochrome ? NULL : storage + luma;
    picture->data[2] = monochrome ? NULL : storage + luma + chroma;
    picture->stride[0] = width * bytes;
    picture->stride[1] = monochrome ? 0 : (width >> horizontal) * bytes;
    picture->allocator_data = storage;
    atomic_fetch_add(&picture_live, 1);
    return 0;
}
static void release_picture(Dav1dPicture *picture, void *cookie) {
    CHECK(cookie == &picture_live);
    CHECK(picture->allocator_data != NULL);
    free(picture->allocator_data);
    atomic_fetch_sub(&picture_live, 1);
}

static uint32_t little_endian(const uint8_t *data) {
    return data[0] | (uint32_t)data[1] << 8 | (uint32_t)data[2] << 16 | (uint32_t)data[3] << 24;
}

static int pictures, events;
static void receive(Dav1dContext *context, int *error) {
    for (;;) {
        Dav1dPicture picture = {0};
        const int result = dav1d_get_picture(context, &picture);
        if (result == DAV1D_ERR(EAGAIN)) break;
        if (result < 0) { *error = result; break; }
        CHECK(result == 0);
        CHECK(picture.seq_hdr && picture.frame_hdr);
        CHECK(picture.m.user_data.data);
        CHECK(*picture.m.user_data.data == (uint8_t)(picture.m.timestamp / 100));
        uint64_t hash = UINT64_C(1469598103934665603);
        const int bytes = picture.p.bpc > 8 ? 2 : 1;
        const int monochrome = picture.p.layout == DAV1D_PIXEL_LAYOUT_I400;
        for (int plane = 0; plane < (monochrome ? 1 : 3); plane++) {
            const int horizontal = plane && picture.p.layout != DAV1D_PIXEL_LAYOUT_I444;
            const int vertical = plane && picture.p.layout == DAV1D_PIXEL_LAYOUT_I420;
            const int width = ((picture.p.w + horizontal) >> horizontal) * bytes;
            const int height = (picture.p.h + vertical) >> vertical;
            for (int y = 0; y < height; y++) {
                const uint8_t *row = (const uint8_t *)picture.data[plane] + y * picture.stride[plane != 0];
                for (int x = 0; x < width; x++) hash = (hash ^ row[x]) * UINT64_C(1099511628211);
            }
        }
        printf("%s[\"%016" PRIx64 "\",%d,%d,%d,%d,%" PRId64 ",%" PRId64 "]",
               pictures ? "," : "", hash, picture.p.w, picture.p.h,
               picture.p.bpc, picture.p.layout, picture.m.timestamp, picture.m.duration);
        pictures++;
        enum Dav1dEventFlags flags;
        CHECK(dav1d_get_event_flags(context, &flags) == 0);
        events |= flags;
        dav1d_picture_unref(&picture);
        CHECK(!picture.data[0]);
    }
}

static void decode(const char *path, int threads, int delay, int grain, int custom, int fail) {
    Dav1dSettings settings;
    dav1d_default_settings(&settings);
    settings.n_threads = threads;
    settings.max_frame_delay = delay;
    settings.apply_grain = grain;
    settings.logger.callback = NULL;
    fail_at = fail;
    if (custom) {
        settings.allocator.cookie = &picture_live;
        settings.allocator.alloc_picture_callback = allocate_picture;
        settings.allocator.release_picture_callback = release_picture;
    }
    Dav1dContext *context = NULL;
    CHECK(dav1d_open(&context, &settings) == 0);
    CHECK(context != NULL);
    FILE *input = fopen(path, "rb");
    CHECK(input);
    uint8_t header[32];
    CHECK(fread(header, 1, 32, input) == 32 && !memcmp(header, "DKIF", 4));
    int packet_index = 0, error = 0;
    printf("{\"pictures\":[");
    uint8_t packet_header[12];
    while (!error && fread(packet_header, 1, 12, input) == 12) {
        const size_t size = little_endian(packet_header);
        CHECK(size && size < 16 * 1024 * 1024);
        uint8_t *packet = malloc(size);
        CHECK(packet && fread(packet, 1, size, input) == size);
        Dav1dData data = {0};
        atomic_fetch_add(&data_live, 1);
        CHECK(dav1d_data_wrap(&data, packet, size, free_packet, packet) == 0);
        uint8_t *user = malloc(1);
        CHECK(user);
        *user = (uint8_t)packet_index;
        atomic_fetch_add(&user_live, 1);
        CHECK(dav1d_data_wrap_user_data(&data, user, free_user, user) == 0);
        data.m.timestamp = packet_index * 100;
        data.m.duration = 17;
        data.m.offset = ftell(input) - (int64_t)size;
        while (data.sz && !error) {
            const int result = dav1d_send_data(context, &data);
            if (result < 0 && result != DAV1D_ERR(EAGAIN)) error = result;
            if (!error) receive(context, &error);
        }
        dav1d_data_unref(&data);
        CHECK(!data.data && !data.sz);
        packet_index++;
    }
    if (!error) receive(context, &error);
    dav1d_flush(context);
    Dav1dPicture empty = {0};
    CHECK(dav1d_get_picture(context, &empty) == DAV1D_ERR(EAGAIN));
    dav1d_close(&context);
    CHECK(context == NULL);
    fclose(input);
    CHECK(atomic_load(&data_live) == 0 && atomic_load(&user_live) == 0 && atomic_load(&picture_live) == 0);
    CHECK(!fail || error == DAV1D_ERR(ENOMEM));
    CHECK(fail || !error);
    printf("],\"events\":%d,\"error\":%d,\"live\":[%d,%d,%d]}\n", events, error,
           atomic_load(&data_live), atomic_load(&user_live), atomic_load(&picture_live));
}

static void invalid_inputs(void) {
    Dav1dSettings settings, original;
    dav1d_default_settings(&original);
    Dav1dContext *context = NULL;
    uint8_t bytes[8] = {0};
    Dav1dSequenceHeader sequence;
    Dav1dData data = {0};
    printf("[\"%s\",%u", dav1d_version(), dav1d_version_api());
    printf(",%d,%d", dav1d_open(NULL, &original), dav1d_open(&context, NULL));
    const int invalid[] = {-1, 257};
    for (int i = 0; i < 2; i++) {
        settings = original; settings.n_threads = invalid[i];
        printf(",%d", dav1d_open(&context, &settings));
        settings = original; settings.max_frame_delay = invalid[i];
        printf(",%d", dav1d_open(&context, &settings));
    }
    settings = original; settings.operating_point = 32;
    printf(",%d", dav1d_open(&context, &settings));
    settings = original; settings.allocator.alloc_picture_callback = NULL;
    printf(",%d", dav1d_open(&context, &settings));
    settings = original; settings.allocator.release_picture_callback = NULL;
    printf(",%d", dav1d_open(&context, &settings));
    printf(",%d,%d,%d", dav1d_parse_sequence_header(NULL, bytes, 8),
           dav1d_parse_sequence_header(&sequence, NULL, 8),
           dav1d_parse_sequence_header(&sequence, bytes, 0));
    printf(",%d,%d,%d", dav1d_data_wrap(NULL, bytes, 8, no_free, NULL),
           dav1d_data_wrap(&data, NULL, 8, no_free, NULL),
           dav1d_data_wrap(&data, bytes, 8, NULL, NULL));
    printf(",%d,%d]\n", dav1d_data_wrap_user_data(NULL, bytes, no_free, NULL),
           dav1d_data_wrap_user_data(&data, bytes, NULL, NULL));
    CHECK(context == NULL);
}

static char log_text[2048];
static size_t log_size;
static void capture_log(void *cookie, const char *format, va_list arguments) {
    CHECK(cookie == log_text);
    const int length = vsnprintf(log_text + log_size, sizeof(log_text) - log_size, format, arguments);
    CHECK(length >= 0 && (size_t)length < sizeof(log_text) - log_size);
    log_size += length;
}
static void logging(void) {
    Dav1dSettings settings;
    dav1d_default_settings(&settings);
    settings.n_threads = settings.max_frame_delay = 1;
    settings.strict_std_compliance = 1;
    settings.logger.cookie = log_text;
    settings.logger.callback = capture_log;
    Dav1dContext *context = NULL;
    CHECK(dav1d_open(&context, &settings) == 0);
    Dav1dData data = {0};
    uint8_t *bytes = dav1d_data_create(&data, 2);
    CHECK(bytes);
    bytes[0] = 0x4a; bytes[1] = 0;
    data.m.timestamp = 321;
    const int sent = dav1d_send_data(context, &data);
    Dav1dPicture picture = {0};
    const int received = dav1d_get_picture(context, &picture);
    if (received == 0) dav1d_picture_unref(&picture);
    dav1d_data_unref(&data);
    bytes = dav1d_data_create(&data, 1);
    CHECK(bytes);
    bytes[0] = 0x80;
    data.m.timestamp = 654;
    const int sent_error = dav1d_send_data(context, &data);
    const int received_error = dav1d_get_picture(context, &picture);
    CHECK(sent_error == DAV1D_ERR(EINVAL) || received_error == DAV1D_ERR(EINVAL));
    Dav1dDataProps error_props = {0};
    const int properties = dav1d_get_decode_error_data_props(context, &error_props);
    const int64_t timestamp = error_props.timestamp;
    dav1d_data_props_unref(&error_props);
    dav1d_data_unref(&data);
    dav1d_close(&context);
    CHECK(log_size > 0);
    CHECK(timestamp == 654);
    printf("[%d,%d,%d,%d,%d,%" PRId64 ",\"", sent, received,
           sent_error, received_error, properties, timestamp);
    for (size_t i = 0; i < log_size; i++) {
        const unsigned char value = log_text[i];
        if (value < 32 || value == '\\' || value == '"') printf("\\u%04x", value);
        else putchar(value);
    }
    printf("\"]\n");
}

int main(int argc, char **argv) {
    CHECK(argc >= 2);
    if (!strcmp(argv[1], "invalid")) invalid_inputs();
    else if (!strcmp(argv[1], "logging")) logging();
    else {
        CHECK(argc == 7);
        decode(argv[1], atoi(argv[2]), atoi(argv[3]), atoi(argv[4]), atoi(argv[5]), atoi(argv[6]));
    }
    return 0;
}
