// Generated from original export lists and translated signatures.
// Public pointers, ownership, callbacks, and bounds follow the original headers.
pub use crate::av1__av1_cx_iface::*;
pub use crate::aom_dsp__bitreader_buffer::aom_read_bit_buffer;
pub use crate::av1__common__obu_util::ObuHeader;
pub type aom_codec_dec_cfg_t = aom_codec_dec_cfg;
extern "C-unwind" {
    pub fn aom_codec_av1_dx() -> *const aom_codec_iface_t;
    pub fn aom_codec_build_config() -> *const core::ffi::c_char;
    pub fn aom_codec_control(
    ctx: *mut aom_codec_ctx_t,
    ctrl_id: core::ffi::c_int,
    ...
) -> aom_codec_err_t;
    pub fn aom_codec_dec_init_ver(
    ctx: *mut aom_codec_ctx_t,
    iface: *const aom_codec_iface_t,
    cfg: *const aom_codec_dec_cfg_t,
    flags: aom_codec_flags_t,
    ver: core::ffi::c_int,
) -> aom_codec_err_t;
    pub fn aom_codec_decode(
    ctx: *mut aom_codec_ctx_t,
    data: *const uint8_t,
    data_sz: size_t,
    user_priv: *mut core::ffi::c_void,
) -> aom_codec_err_t;
    pub fn aom_codec_destroy(
    ctx: *mut aom_codec_ctx_t,
) -> aom_codec_err_t;
    pub fn aom_codec_enc_config_default(
    iface: *const aom_codec_iface_t,
    cfg: *mut aom_codec_enc_cfg_t,
    usage: core::ffi::c_uint,
) -> aom_codec_err_t;
    pub fn aom_codec_enc_config_set(
    ctx: *mut aom_codec_ctx_t,
    cfg: *const aom_codec_enc_cfg_t,
) -> aom_codec_err_t;
    pub fn aom_codec_enc_init_ver(
    ctx: *mut aom_codec_ctx_t,
    iface: *const aom_codec_iface_t,
    cfg: *const aom_codec_enc_cfg_t,
    flags: aom_codec_flags_t,
    ver: core::ffi::c_int,
) -> aom_codec_err_t;
    pub fn aom_codec_encode(
    ctx: *mut aom_codec_ctx_t,
    img: *const aom_image_t,
    pts: aom_codec_pts_t,
    duration: core::ffi::c_ulong,
    flags: aom_enc_frame_flags_t,
) -> aom_codec_err_t;
    pub fn aom_codec_err_to_string(
    err: aom_codec_err_t,
) -> *const core::ffi::c_char;
    pub fn aom_codec_error(
    ctx: *const aom_codec_ctx_t,
) -> *const core::ffi::c_char;
    pub fn aom_codec_error_detail(
    ctx: *const aom_codec_ctx_t,
) -> *const core::ffi::c_char;
    pub fn aom_codec_get_caps(
    iface: *const aom_codec_iface_t,
) -> aom_codec_caps_t;
    pub fn aom_codec_get_cx_data(
    ctx: *mut aom_codec_ctx_t,
    iter: *mut aom_codec_iter_t,
) -> *const aom_codec_cx_pkt_t;
    pub fn aom_codec_get_frame(
    ctx: *mut aom_codec_ctx_t,
    iter: *mut aom_codec_iter_t,
) -> *mut aom_image_t;
    pub fn aom_codec_get_global_headers(
    ctx: *mut aom_codec_ctx_t,
) -> *mut aom_fixed_buf_t;
    pub fn aom_codec_get_preview_frame(
    ctx: *mut aom_codec_ctx_t,
) -> *const aom_image_t;
    pub fn aom_codec_get_stream_info(
    ctx: *mut aom_codec_ctx_t,
    si: *mut aom_codec_stream_info_t,
) -> aom_codec_err_t;
    pub fn aom_codec_iface_name(
    iface: *const aom_codec_iface_t,
) -> *const core::ffi::c_char;
    pub fn aom_codec_peek_stream_info(
    iface: *const aom_codec_iface_t,
    data: *const uint8_t,
    data_sz: size_t,
    si: *mut aom_codec_stream_info_t,
) -> aom_codec_err_t;
    pub fn aom_codec_set_cx_data_buf(
    ctx: *mut aom_codec_ctx_t,
    buf: *const aom_fixed_buf_t,
    pad_before: core::ffi::c_uint,
    pad_after: core::ffi::c_uint,
) -> aom_codec_err_t;
    pub fn aom_codec_set_frame_buffer_functions(
    ctx: *mut aom_codec_ctx_t,
    cb_get: aom_get_frame_buffer_cb_fn_t,
    cb_release: aom_release_frame_buffer_cb_fn_t,
    cb_priv: *mut core::ffi::c_void,
) -> aom_codec_err_t;
    pub fn aom_codec_set_option(
    ctx: *mut aom_codec_ctx_t,
    name: *const core::ffi::c_char,
    value: *const core::ffi::c_char,
) -> aom_codec_err_t;
    pub fn aom_codec_version() -> core::ffi::c_int;
    pub fn aom_codec_version_extra_str() -> *const core::ffi::c_char;
    pub fn aom_codec_version_str() -> *const core::ffi::c_char;
    pub fn aom_img_add_metadata(
    img: *mut aom_image_t,
    type_0: uint32_t,
    data: *const uint8_t,
    sz: size_t,
    insert_flag: aom_metadata_insert_flags_t,
) -> core::ffi::c_int;
    pub fn aom_img_alloc(
    img: *mut aom_image_t,
    fmt: aom_img_fmt_t,
    d_w: core::ffi::c_uint,
    d_h: core::ffi::c_uint,
    align: core::ffi::c_uint,
) -> *mut aom_image_t;
    pub fn aom_img_alloc_with_border(
    img: *mut aom_image_t,
    fmt: aom_img_fmt_t,
    d_w: core::ffi::c_uint,
    d_h: core::ffi::c_uint,
    align: core::ffi::c_uint,
    size_align: core::ffi::c_uint,
    border: core::ffi::c_uint,
) -> *mut aom_image_t;
    pub fn aom_img_flip(img: *mut aom_image_t);
    pub fn aom_img_free(img: *mut aom_image_t);
    pub fn aom_img_get_metadata(
    img: *const aom_image_t,
    index: size_t,
) -> *const aom_metadata_t;
    pub fn aom_img_metadata_alloc(
    type_0: uint32_t,
    data: *const uint8_t,
    sz: size_t,
    insert_flag: aom_metadata_insert_flags_t,
) -> *mut aom_metadata_t;
    pub fn aom_img_metadata_free(metadata: *mut aom_metadata_t);
    pub fn aom_img_num_metadata(img: *const aom_image_t) -> size_t;
    pub fn aom_img_plane_height(
    img: *const aom_image_t,
    plane: core::ffi::c_int,
) -> core::ffi::c_int;
    pub fn aom_img_plane_width(
    img: *const aom_image_t,
    plane: core::ffi::c_int,
) -> core::ffi::c_int;
    pub fn aom_img_remove_metadata(img: *mut aom_image_t);
    pub fn aom_img_set_rect(
    img: *mut aom_image_t,
    x: core::ffi::c_uint,
    y: core::ffi::c_uint,
    w: core::ffi::c_uint,
    h: core::ffi::c_uint,
    border: core::ffi::c_uint,
) -> core::ffi::c_int;
    pub fn aom_img_wrap(
    img: *mut aom_image_t,
    fmt: aom_img_fmt_t,
    d_w: core::ffi::c_uint,
    d_h: core::ffi::c_uint,
    stride_align: core::ffi::c_uint,
    img_data: *mut core::ffi::c_uchar,
) -> *mut aom_image_t;
    pub fn aom_obu_type_to_string(
    type_0: OBU_TYPE,
) -> *const core::ffi::c_char;
    pub fn aom_rb_bytes_read(
    rb: *const aom_read_bit_buffer,
) -> size_t;
    pub fn aom_rb_read_bit(
    rb: *mut aom_read_bit_buffer,
) -> core::ffi::c_int;
    pub fn aom_rb_read_literal(
    rb: *mut aom_read_bit_buffer,
    bits: core::ffi::c_int,
) -> core::ffi::c_int;
    pub fn aom_rb_read_unsigned_literal(
    rb: *mut aom_read_bit_buffer,
    bits: core::ffi::c_int,
) -> uint32_t;
    pub fn aom_rb_read_uvlc(rb: *mut aom_read_bit_buffer) -> uint32_t;
    pub fn aom_read_obu_header(
    buffer: *mut uint8_t,
    buffer_length: size_t,
    consumed: *mut size_t,
    header: *mut ObuHeader,
    is_annexb: core::ffi::c_int,
) -> aom_codec_err_t;
    pub fn aom_read_obu_header_and_size(
    data: *const uint8_t,
    bytes_available: size_t,
    is_annexb: core::ffi::c_int,
    obu_header: *mut ObuHeader,
    payload_size: *mut size_t,
    bytes_read: *mut size_t,
) -> aom_codec_err_t;
    pub fn aom_uleb_decode(
    buffer: *const uint8_t,
    available: size_t,
    value: *mut uint64_t,
    length: *mut size_t,
) -> core::ffi::c_int;
    pub fn aom_uleb_encode(
    value: uint64_t,
    available: size_t,
    coded_value: *mut uint8_t,
    coded_size: *mut size_t,
) -> core::ffi::c_int;
    pub fn aom_uleb_encode_fixed_size(
    value: uint64_t,
    available: size_t,
    pad_to_size: size_t,
    coded_value: *mut uint8_t,
    coded_size: *mut size_t,
) -> core::ffi::c_int;
    pub fn aom_uleb_size_in_bytes(value: uint64_t) -> size_t;
}
pub const AOM_CODEC_ABI_VERSION: core::ffi::c_int = 16;
pub const AOM_CODEC_CAP_DECODER: aom_codec_caps_t = 1;
pub const AOM_CODEC_CAP_ENCODER: aom_codec_caps_t = 2;
pub const AOM_CODEC_CAP_EXTERNAL_FRAME_BUFFER: aom_codec_caps_t = 2097152;
pub const AOM_CODEC_CAP_HIGHBITDEPTH: aom_codec_caps_t = 262144;
pub const AOM_CODEC_CAP_PSNR: aom_codec_caps_t = 65536;
pub const AOM_CODEC_USE_EXPERIMENTAL: aom_codec_flags_t = 1;
pub const AOM_CODEC_USE_HIGHBITDEPTH: aom_codec_flags_t = 262144;
pub const AOM_CODEC_USE_PSNR: aom_codec_flags_t = 65536;
pub const AOM_DECODER_ABI_VERSION: core::ffi::c_int = 22;
pub const AOM_EFLAG_CALCULATE_PSNR: aom_enc_frame_flags_t = 2;
pub const AOM_EFLAG_ERROR_RESILIENT: aom_enc_frame_flags_t = 268435456;
pub const AOM_EFLAG_FORCE_KF: aom_enc_frame_flags_t = 1;
pub const AOM_EFLAG_FREEZE_INTERNAL_STATE: aom_enc_frame_flags_t = 4;
pub const AOM_EFLAG_NO_REF_ARF: aom_enc_frame_flags_t = 1048576;
pub const AOM_EFLAG_NO_REF_ARF2: aom_enc_frame_flags_t = 4194304;
pub const AOM_EFLAG_NO_REF_BWD: aom_enc_frame_flags_t = 2097152;
pub const AOM_EFLAG_NO_REF_FRAME_MVS: aom_enc_frame_flags_t = 134217728;
pub const AOM_EFLAG_NO_REF_GF: aom_enc_frame_flags_t = 524288;
pub const AOM_EFLAG_NO_REF_LAST: aom_enc_frame_flags_t = 65536;
pub const AOM_EFLAG_NO_REF_LAST2: aom_enc_frame_flags_t = 131072;
pub const AOM_EFLAG_NO_REF_LAST3: aom_enc_frame_flags_t = 262144;
pub const AOM_EFLAG_NO_UPD_ARF: aom_enc_frame_flags_t = 33554432;
pub const AOM_EFLAG_NO_UPD_ENTROPY: aom_enc_frame_flags_t = 67108864;
pub const AOM_EFLAG_NO_UPD_GF: aom_enc_frame_flags_t = 16777216;
pub const AOM_EFLAG_NO_UPD_LAST: aom_enc_frame_flags_t = 8388608;
pub const AOM_EFLAG_SET_PRIMARY_REF_NONE: aom_enc_frame_flags_t = 1073741824;
pub const AOM_EFLAG_SET_S_FRAME: aom_enc_frame_flags_t = 536870912;
pub const AOM_ENCODER_ABI_VERSION: core::ffi::c_int = 29;
pub const AOM_EXT_PART_ABI_VERSION: core::ffi::c_int = 8;
pub const AOM_EXT_RATECTRL_ABI_VERSION: core::ffi::c_int = 3;
pub const AOM_FRAME_IS_DELAYED_RANDOM_ACCESS_POINT: aom_codec_frame_flags_t = 128;
pub const AOM_FRAME_IS_DROPPABLE: aom_codec_frame_flags_t = 2;
pub const AOM_FRAME_IS_ERROR_RESILIENT: aom_codec_frame_flags_t = 64;
pub const AOM_FRAME_IS_INTRAONLY: aom_codec_frame_flags_t = 16;
pub const AOM_FRAME_IS_KEY: aom_codec_frame_flags_t = 1;
pub const AOM_FRAME_IS_SWITCH: aom_codec_frame_flags_t = 32;
pub const AOM_IMAGE_ABI_VERSION: core::ffi::c_int = 9;
pub const AOM_IMG_FMT_HIGHBITDEPTH: core::ffi::c_uint = 2048;
pub const AOM_IMG_FMT_PLANAR: core::ffi::c_uint = 256;
pub const AOM_IMG_FMT_UV_FLIP: core::ffi::c_uint = 512;
pub const AOM_TPL_ABI_VERSION: core::ffi::c_int = 1;
pub const AOM_USAGE_ALL_INTRA: core::ffi::c_uint = 2;
pub const AOM_USAGE_GOOD_QUALITY: core::ffi::c_uint = 0;
pub const AOM_USAGE_REALTIME: core::ffi::c_uint = 1;
