//! Visible samples of the WP9 pictures, as the C probes print them: the size, the
//! colorspace, the error code and the planes without their padding. Shared by the
//! rescale and view tests.

use crate::oracle::Fields;
use webp::webp::encode::WebPPicture;
use webp::webp::types::{PlaneOffset, PlaneStorage};

/// The visible samples of a plane of `width` by `height` samples, `stride` apart.
pub fn visible(samples: &[u8], width: usize, height: usize, stride: usize) -> Vec<u8> {
    (0..height)
        .flat_map(|row| samples[row * stride..row * stride + width].iter().copied())
        .collect()
}

pub fn plane_of<'a>(storage: &'a PlaneStorage<'_, u8>, plane: PlaneOffset) -> &'a [u8] {
    &storage.region(plane.region).expect("the region exists")[plane.offset..]
}

/// The probe's `PrintVisible` for a picture: its size, colorspace, error and samples.
pub fn visible_picture(picture: &WebPPicture<'_>) -> Fields {
    let width = usize::try_from(picture.width).unwrap();
    let height = usize::try_from(picture.height).unwrap();
    let uv_width = width.div_ceil(2);
    let uv_height = height.div_ceil(2);
    let mut fields = Fields::new()
        .with("width", picture.width)
        .with("height", picture.height)
        .with("colorspace", picture.colorspace.0)
        .with("use_argb", picture.use_argb)
        .with("error", picture.error_code.0);
    if picture.use_argb != 0 {
        let plane = picture.argb.expect("the ARGB plane exists");
        let stride = usize::try_from(picture.argb_stride).unwrap();
        let words = &picture
            .argb_storage
            .region(plane.region)
            .expect("the region exists")[plane.offset..];
        let bytes: Vec<u8> = (0..height)
            .flat_map(|row| {
                words[row * stride..row * stride + width]
                    .iter()
                    .flat_map(|word| word.to_ne_bytes())
            })
            .collect();
        return fields.hex("argb", &bytes);
    }
    let y = picture.y.expect("the luma plane exists");
    let u = picture.u.expect("the U plane exists");
    let v = picture.v.expect("the V plane exists");
    let y_stride = usize::try_from(picture.y_stride).unwrap();
    let uv_stride = usize::try_from(picture.uv_stride).unwrap();
    fields = fields
        .hex(
            "y",
            &visible(plane_of(&picture.yuva, y), width, height, y_stride),
        )
        .hex(
            "u",
            &visible(plane_of(&picture.yuva, u), uv_width, uv_height, uv_stride),
        )
        .hex(
            "v",
            &visible(plane_of(&picture.yuva, v), uv_width, uv_height, uv_stride),
        );
    if let Some(a) = picture.a {
        let a_stride = usize::try_from(picture.a_stride).unwrap();
        fields = fields.hex(
            "a",
            &visible(plane_of(&picture.yuva, a), width, height, a_stride),
        );
    }
    fields
}
