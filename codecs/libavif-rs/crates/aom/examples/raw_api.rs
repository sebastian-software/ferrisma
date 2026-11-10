//! Minimal Rust consumer of the original raw encoder and image APIs.
use aom::raw::*;

fn main() {
    // SAFETY: The original headers permit zeroed context storage, configuration
    // initialization through enc_config_default, and these dimensions/flags.
    // Each successful initialization/allocation has one matching destroy/free;
    // all pointers remain live and aligned for the duration of their calls.
    unsafe {
        let iface = aom_codec_av1_cx();
        let mut config: aom_codec_enc_cfg_t = core::mem::zeroed();
        assert_eq!(aom_codec_enc_config_default(iface, &raw mut config, AOM_USAGE_ALL_INTRA), AOM_CODEC_OK);
        config.g_w = 32;
        config.g_h = 32;
        config.g_limit = 1;
        config.g_lag_in_frames = 0;
        let mut context: aom_codec_ctx_t = core::mem::zeroed();
        assert_eq!(aom_codec_enc_init_ver(&raw mut context, iface, &raw const config, 0,
                                         AOM_ENCODER_ABI_VERSION), AOM_CODEC_OK);
        let image = aom_img_alloc(core::ptr::null_mut(), AOM_IMG_FMT_I420, 32, 32, 32);
        if image.is_null() {
            aom_codec_destroy(&raw mut context);
            panic!("image allocation failed");
        }
        aom_img_free(image);
        let result = aom_codec_destroy(&raw mut context);
        assert_eq!(result, AOM_CODEC_OK);
        println!("libaom {} raw Rust API initialized and released", aom_codec_version());
    }
}
