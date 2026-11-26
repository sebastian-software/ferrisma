//! Decoder-independent, separable antialiasing weights. No pixel allocation.

use crate::error::Error;

/// Resampling filter, exported as `pipeline::ResizeFilter`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum Filter {
    /// Box/area averaging, suited to strong downscaling.
    Area,
    /// Three-lobe Lanczos, the default.
    Lanczos3,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Tap {
    pub index: u32,
    pub weight: f64,
}

#[derive(Clone, Copy)]
struct TapGeometry {
    ratio: f64,
    scale: f64,
    radius: f64,
    max_index: f64,
}

impl TapGeometry {
    fn new(input: u32, output: u32, filter: Filter) -> Self {
        let ratio = f64::from(input) / f64::from(output);
        let scale = ratio.max(1.0);
        let radius = match filter {
            Filter::Area => scale * 0.5,
            Filter::Lanczos3 => scale * 3.0,
        };
        Self {
            ratio,
            scale,
            radius,
            max_index: f64::from(input - 1),
        }
    }

    #[inline]
    fn center(self, at: u32) -> f64 {
        (f64::from(at) + 0.5) * self.ratio - 0.5
    }

    #[inline]
    fn candidate_range(self, center: f64) -> (u32, u32) {
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let first = (center - self.radius - 1.0).floor().max(0.0) as u32;
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let last = (center + self.radius + 1.0).ceil().min(self.max_index) as u32;
        (first, last)
    }

    #[inline]
    fn raw_weight(self, filter: Filter, center: f64, index: u32) -> f64 {
        let sample = f64::from(index);
        match filter {
            Filter::Area => ((sample + 0.5).min(center + self.radius)
                - (sample - 0.5).max(center - self.radius))
            .max(0.0),
            Filter::Lanczos3 => {
                let d = (sample - center) / self.scale;
                if d.abs() < 3.0 {
                    sinc(d) * sinc(d / 3.0)
                } else {
                    0.0
                }
            }
        }
    }

    /// True if `index` (and every smaller index) lies outside the left edge
    /// of the filter support at `center`, and therefore also at every larger
    /// destination position `at' >= at`.
    #[inline]
    fn strictly_left_of_support(self, filter: Filter, center: f64, index: u32) -> bool {
        let sample = f64::from(index);
        match filter {
            Filter::Area => sample + 0.5 <= center - self.radius,
            Filter::Lanczos3 => (sample - center) / self.scale <= -3.0,
        }
    }

    /// True if `index` (and every larger index) lies outside the right edge
    /// of the filter support at `center`, and therefore also at every smaller
    /// destination position `at' <= at`.
    #[inline]
    fn strictly_right_of_support(self, filter: Filter, center: f64, index: u32) -> bool {
        let sample = f64::from(index);
        match filter {
            Filter::Area => sample - 0.5 >= center + self.radius,
            Filter::Lanczos3 => (sample - center) / self.scale >= 3.0,
        }
    }
}

/// Populate `out` with the normalized filter taps for destination position
/// `at`, reusing `out`'s allocation across calls.
pub(crate) fn taps_into(
    input: u32,
    output: u32,
    at: u32,
    filter: Filter,
    out: &mut Vec<Tap>,
) -> Result<(), Error> {
    if input == 0 || output == 0 || at >= output {
        return Err(Error::InvalidDimensions);
    }
    out.clear();
    if input == output {
        if out.capacity() == 0 {
            out.try_reserve_exact(1)
                .map_err(|_| Error::AllocationFailed)?;
        }
        out.push(Tap {
            index: at,
            weight: 1.0,
        });
        return Ok(());
    }
    let geom = TapGeometry::new(input, output, filter);
    let center = geom.center(at);
    let (first, last) = geom.candidate_range(center);
    let count = usize::try_from(u64::from(last) - u64::from(first) + 1)
        .map_err(|_| Error::GeometryOverflow)?;
    if out.capacity() < count {
        out.try_reserve_exact(count)
            .map_err(|_| Error::AllocationFailed)?;
    }
    let mut sum = 0.0;
    for index in first..=last {
        let weight = geom.raw_weight(filter, center, index);
        if weight.abs() > f64::EPSILON {
            out.push(Tap { index, weight });
            sum += weight;
        }
    }
    if out.is_empty() || !sum.is_finite() || sum.abs() < f64::EPSILON {
        return Err(Error::UnsupportedPixelOperation);
    }
    for tap in out.iter_mut() {
        tap.weight /= sum;
    }
    Ok(())
}

pub(crate) fn taps(input: u32, output: u32, at: u32, filter: Filter) -> Result<Vec<Tap>, Error> {
    let mut weights = Vec::new();
    taps_into(input, output, at, filter, &mut weights)?;
    Ok(weights)
}

fn sinc(x: f64) -> f64 {
    if x.abs() < 1.0e-12 {
        1.0
    } else {
        let p = x * std::f64::consts::PI;
        p.sin() / p
    }
}

pub(crate) fn footprint(
    input: u32,
    output: u32,
    start: u32,
    count: u32,
    filter: Filter,
) -> Result<(u32, u32), Error> {
    if count == 0 || start.checked_add(count).is_none_or(|end| end > output) {
        return Err(Error::OutOfBounds);
    }
    if input == 0 {
        return Err(Error::InvalidDimensions);
    }
    if input == output {
        return Ok((start, count));
    }
    let end = start.checked_add(count).ok_or(Error::GeometryOverflow)?;
    let geom = TapGeometry::new(input, output, filter);

    // Lanczos sinc zero crossings can make the lowest *nonzero* tap
    // nonmonotonic, so endpoint-only evaluation can miss a smaller source
    // index needed by a nearby interior position. However, `center(at)` is
    // monotonically non-decreasing in `at`: once `first_source - 1` lies
    // strictly left of the filter support at `at`, no later `at' >= at` can
    // reference any index `< first_source`, so the scan stops after 1–2
    // positions in practice with zero heap allocations.
    let mut first_source = u32::MAX;
    for at in start..end {
        let center = geom.center(at);
        if first_source == 0
            || (first_source != u32::MAX
                && geom.strictly_left_of_support(filter, center, first_source - 1))
        {
            break;
        }
        let (first, last) = geom.candidate_range(center);
        let limit = if first_source == u32::MAX {
            last
        } else {
            last.min(first_source - 1)
        };
        if first <= limit {
            for index in first..=limit {
                if geom.raw_weight(filter, center, index).abs() > f64::EPSILON {
                    first_source = index;
                    break;
                }
            }
        }
        if first_source == 0
            || (first_source != u32::MAX
                && geom.strictly_left_of_support(filter, center, first_source - 1))
        {
            break;
        }
    }
    if first_source == u32::MAX {
        return Err(Error::UnsupportedPixelOperation);
    }

    // Symmetrically scan downward from `end - 1` for the largest nonzero
    // source index.
    let max_index = input - 1;
    let mut last_source = None;
    for at in (start..end).rev() {
        let center = geom.center(at);
        if let Some(best) = last_source
            && (best == max_index || geom.strictly_right_of_support(filter, center, best + 1))
        {
            break;
        }
        let (first, last) = geom.candidate_range(center);
        let lower = match last_source {
            None => first,
            Some(best) => first.max(best + 1),
        };
        if lower <= last {
            for index in (lower..=last).rev() {
                if geom.raw_weight(filter, center, index).abs() > f64::EPSILON {
                    last_source = Some(index);
                    break;
                }
            }
        }
        if let Some(best) = last_source
            && (best == max_index || geom.strictly_right_of_support(filter, center, best + 1))
        {
            break;
        }
    }
    let last_source = last_source.ok_or(Error::UnsupportedPixelOperation)?;
    Ok((first_source, last_source - first_source + 1))
}
