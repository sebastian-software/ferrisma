//! Integer geometry for the public thumbnail facade.
use crate::error::Error;
use crate::geometry::{Rect, Size};

/// How two requested dimensions relate to the selected upright source region.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[non_exhaustive]
pub enum ResizeFit {
    /// Preserve aspect ratio and fit within the requested bounds.
    #[default]
    Inside,
    /// Center-crop to the requested aspect ratio and fill the bounds.
    Cover,
    /// Stretch independently to each requested dimension.
    Fill,
}

/// Thumbnail geometry. At least one dimension must be nonzero and present.
/// One dimension always preserves aspect ratio. With two dimensions, `fit`
/// selects the behavior. An explicit `TransformOptions::crop` is applied
/// first in upright display coordinates. Derived dimensions round to nearest,
/// with ties up and a minimum of one pixel. Cover floors the derived source
/// crop extent (minimum one), and floors its center origin, leaving an odd
/// excess pixel on the right or bottom. Integer crop rounding can slightly
/// alter the aspect ratio, especially for tiny images.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ResizeOptions {
    /// Desired width in pixels, or `None` to derive it from height.
    pub width: Option<u32>,
    /// Desired height in pixels, or `None` to derive it from width.
    pub height: Option<u32>,
    /// Interpretation of two supplied dimensions.
    pub fit: ResizeFit,
    /// Clamp aspect-preserving scale to one; for fill clamp each axis
    /// independently. Cover may consequently return a smaller output.
    pub without_enlargement: bool,
}

// All inputs are u32, so the product fits u64. Divide before rounding to
// avoid overflowing when the product is near u64::MAX.
fn ratio(value: u32, numerator: u32, denominator: u32, nearest: bool) -> Result<u32, Error> {
    let product = u64::from(value) * u64::from(numerator);
    let divisor = u64::from(denominator);
    let mut result = product / divisor;
    if nearest && product % divisor >= divisor.div_ceil(2) {
        result += 1;
    }
    u32::try_from(result.max(1)).map_err(|_| Error::GeometryOverflow)
}

pub(crate) fn validate(options: ResizeOptions) -> Result<(), Error> {
    if (options.width.is_none() && options.height.is_none())
        || options.width == Some(0)
        || options.height == Some(0)
    {
        return Err(Error::InvalidDimensions);
    }
    Ok(())
}

pub(crate) fn resolve(mut crop: Rect, options: ResizeOptions) -> Result<(Rect, Size), Error> {
    validate(options)?;
    let (w, h) = (crop.width, crop.height);
    let target = match (options.width, options.height) {
        (None, None) => return Err(Error::InvalidDimensions),
        (Some(width), None) => {
            let width = if options.without_enlargement {
                width.min(w)
            } else {
                width
            };
            Size::new(width, ratio(h, width, w, true)?)?
        }
        (None, Some(height)) => {
            let height = if options.without_enlargement {
                height.min(h)
            } else {
                height
            };
            Size::new(ratio(w, height, h, true)?, height)?
        }
        (Some(width), Some(height)) => match options.fit {
            ResizeFit::Fill => Size::new(
                if options.without_enlargement {
                    width.min(w)
                } else {
                    width
                },
                if options.without_enlargement {
                    height.min(h)
                } else {
                    height
                },
            )?,
            ResizeFit::Inside => {
                if options.without_enlargement && width >= w && height >= h {
                    Size::new(w, h)?
                } else if u64::from(width) * u64::from(h) <= u64::from(height) * u64::from(w) {
                    Size::new(width, ratio(h, width, w, true)?.min(height))?
                } else {
                    Size::new(ratio(w, height, h, true)?.min(width), height)?
                }
            }
            ResizeFit::Cover => {
                if u64::from(w) * u64::from(height) > u64::from(h) * u64::from(width) {
                    let new_width = ratio(h, width, height, false)?.min(w);
                    crop.x += (w - new_width) / 2;
                    crop.width = new_width;
                } else {
                    let new_height = ratio(w, height, width, false)?.min(h);
                    crop.y += (h - new_height) / 2;
                    crop.height = new_height;
                }
                if options.without_enlargement && (width > crop.width || height > crop.height) {
                    if u64::from(crop.width) * u64::from(height)
                        <= u64::from(crop.height) * u64::from(width)
                    {
                        Size::new(
                            crop.width,
                            ratio(height, crop.width, width, true)?.min(crop.height),
                        )?
                    } else {
                        Size::new(
                            ratio(width, crop.height, height, true)?.min(crop.width),
                            crop.height,
                        )?
                    }
                } else {
                    Size::new(width, height)?
                }
            }
        },
    };
    Ok((crop, target))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn request(
        width: Option<u32>,
        height: Option<u32>,
        fit: ResizeFit,
        without_enlargement: bool,
    ) -> ResizeOptions {
        ResizeOptions {
            width,
            height,
            fit,
            without_enlargement,
        }
    }

    #[test]
    fn fits_and_centers_with_crop_offsets_and_odd_remainders() {
        let crop = Rect::new(5, 7, 9, 5).unwrap();
        let (inside, size) =
            resolve(crop, request(Some(4), Some(4), ResizeFit::Inside, false)).unwrap();
        assert_eq!(inside, crop);
        assert_eq!(size, Size::new(4, 2).unwrap());
        let (cover, size) =
            resolve(crop, request(Some(4), Some(4), ResizeFit::Cover, false)).unwrap();
        assert_eq!(cover, Rect::new(7, 7, 5, 5).unwrap());
        assert_eq!(size, Size::new(4, 4).unwrap());
        let (cover, _) = resolve(
            Rect::new(0, 0, 8, 5).unwrap(),
            request(Some(1), Some(1), ResizeFit::Cover, false),
        )
        .unwrap();
        assert_eq!(cover, Rect::new(1, 0, 5, 5).unwrap());
    }

    #[test]
    fn single_dimension_rounds_ties_up_and_detects_overflow() {
        let crop = Rect::new(0, 0, 4, 3).unwrap();
        let (_, target) = resolve(crop, request(Some(2), None, ResizeFit::Cover, false)).unwrap();
        assert_eq!(target, Size::new(2, 2).unwrap());
        let huge = Rect::new(0, 0, 1, u32::MAX).unwrap();
        assert!(matches!(
            resolve(huge, request(Some(2), None, ResizeFit::Inside, false)),
            Err(Error::GeometryOverflow)
        ));
        let (_, tiny) = resolve(huge, request(None, Some(1), ResizeFit::Inside, false)).unwrap();
        assert_eq!(tiny, Size::new(1, 1).unwrap());
        assert!(resolve(crop, request(None, None, ResizeFit::Inside, false)).is_err());
        assert!(resolve(crop, request(Some(0), None, ResizeFit::Inside, false)).is_err());
    }

    #[test]
    fn no_enlargement_clamps_cover_and_fill_as_documented() {
        let crop = Rect::new(0, 0, 3, 5).unwrap();
        let (cover, output) =
            resolve(crop, request(Some(9), Some(9), ResizeFit::Cover, true)).unwrap();
        assert_eq!(cover, Rect::new(0, 1, 3, 3).unwrap());
        assert_eq!(output, Size::new(3, 3).unwrap());
        let (_, output) = resolve(crop, request(Some(9), Some(2), ResizeFit::Fill, true)).unwrap();
        assert_eq!(output, Size::new(3, 2).unwrap());
        let (_, output) =
            resolve(crop, request(Some(9), Some(9), ResizeFit::Inside, true)).unwrap();
        assert_eq!(output, Size::new(3, 5).unwrap());
        let (cover, output) = resolve(
            Rect::new(0, 0, 1, 2).unwrap(),
            request(Some(100), Some(1), ResizeFit::Cover, true),
        )
        .unwrap();
        assert_eq!(cover, Rect::new(0, 0, 1, 1).unwrap());
        assert_eq!(output, Size::new(1, 1).unwrap());
    }

    #[test]
    fn small_geometries_never_leave_the_source_or_enlarge_when_clamped() {
        for w in 1..=12 {
            for h in 1..=12 {
                for tw in 1..=12 {
                    for th in 1..=12 {
                        for fit in [ResizeFit::Inside, ResizeFit::Cover, ResizeFit::Fill] {
                            let source = Rect::new(2, 3, w, h).unwrap();
                            let (crop, output) =
                                resolve(source, request(Some(tw), Some(th), fit, true)).unwrap();
                            assert!(
                                crop.x >= source.x
                                    && crop.y >= source.y
                                    && crop.right() <= source.right()
                                    && crop.bottom() <= source.bottom()
                            );
                            assert!(output.width <= crop.width && output.height <= crop.height);
                            assert!(output.width <= tw && output.height <= th);
                        }
                    }
                }
            }
        }
    }
}
