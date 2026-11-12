//! Errors returned by the experimental engine and decoder execution paths.

use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Error {
    InvalidDimensions,
    GeometryOverflow,
    OutOfBounds,
    UnknownNode,
    AllocationFailed,
    ThreadSpawnFailed,
    #[cfg_attr(
        not(test),
        allow(dead_code, reason = "used by test-only architecture experiments")
    )]
    InvalidPlaneStride,
    #[cfg_attr(
        not(test),
        allow(dead_code, reason = "used by test-only architecture experiments")
    )]
    InvalidPlaneDimensions,
    #[cfg_attr(
        not(test),
        allow(dead_code, reason = "used by test-only architecture experiments")
    )]
    InsufficientPlaneStorage,
    ChromaPhaseRequiresResampling,
    PixelFormatMismatch,
    SourceBindingMismatch,
    UnsupportedPixelOperation,
    BudgetExceeded {
        requested: usize,
        available: usize,
    },
    #[cfg_attr(
        not(test),
        allow(dead_code, reason = "used by test-only architecture experiments")
    )]
    Cancelled,
    // Constructed only when an optional real codec is enabled.
    #[allow(dead_code)]
    DecodeFailed,
    IncompleteDecoderOutput,
    NonFinitePixelSample,
    InvalidAlphaValue,
    #[cfg(any(
        feature = "avif-read",
        feature = "webp-read",
        feature = "jpeg-xl-own-read"
    ))]
    InputPixelLimitExceeded {
        pixels: u64,
        limit: u64,
    },
    #[cfg(feature = "avif-read")]
    ContradictoryColorSignals,
    #[cfg_attr(
        not(any(feature = "jpeg-read", feature = "webp-read", feature = "avif-read")),
        allow(dead_code, reason = "constructed only by codec adapters")
    )]
    UnsupportedImageTransform,
    #[cfg_attr(
        not(any(
            feature = "avif-write",
            feature = "webp-write",
            feature = "jpeg-xl-write"
        )),
        allow(dead_code)
    )]
    InvalidEncodeOptions,
    #[cfg(any(
        feature = "avif-write",
        feature = "webp-write",
        feature = "jpeg-xl-write"
    ))]
    IncompleteOutput,
    /// A request outside the admitted JPEG XL encoder profile (ADR-0039).
    #[cfg(feature = "jpeg-xl-write")]
    UnsupportedEncoderProfile(&'static str),
    EncodeFailed,
    UnsupportedColorEncoding,
    /// CICP evidence beside the rendered ICC profile that the rendered-ICC
    /// stage cannot reconcile with it; the reason names the conflict. ICC is
    /// never given precedence over CICP silently (ADR-0015 rule 4).
    #[cfg(feature = "icc")]
    CicpBesideIcc(&'static str),
    UnsupportedAlphaMode,
    /// Animated input; only still images are decoded (no first-frame
    /// fallback).
    #[cfg_attr(
        not(any(
            feature = "webp-read",
            feature = "jpeg-xl-read",
            feature = "jpeg-xl-own-read",
            feature = "avif-read"
        )),
        allow(dead_code, reason = "constructed only by animated-format adapters")
    )]
    UnsupportedAnimation,
    #[allow(dead_code)]
    UnsupportedCodecLayout,
}

impl fmt::Display for Error {
    #[allow(clippy::too_many_lines)] // One message per variant.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidDimensions => write!(f, "image dimensions must be nonzero"),
            Self::GeometryOverflow => write!(f, "image geometry or byte count overflow"),
            Self::OutOfBounds => write!(f, "requested region is outside image bounds"),
            Self::UnknownNode => write!(f, "graph node does not exist"),
            Self::AllocationFailed => write!(f, "could not allocate the requested buffer"),
            Self::ThreadSpawnFailed => write!(f, "could not start an image processing worker"),
            Self::InvalidPlaneStride => write!(f, "plane stride is smaller than its active row"),
            Self::InvalidPlaneDimensions => {
                write!(f, "plane dimensions do not match sampling geometry")
            }
            Self::InsufficientPlaneStorage => {
                write!(f, "plane storage is shorter than its declared extent")
            }
            Self::ChromaPhaseRequiresResampling => {
                write!(f, "unaligned 4:2:0 crop requires chroma phase handling")
            }
            Self::PixelFormatMismatch => write!(
                f,
                "the requested executor does not support this pixel format"
            ),
            Self::SourceBindingMismatch => {
                write!(f, "the borrowed source does not match graph metadata")
            }
            Self::UnsupportedPixelOperation => write!(
                f,
                "the operation has no validated implementation for this pixel format"
            ),
            Self::BudgetExceeded {
                requested,
                available,
            } => write!(
                f,
                "managed buffer needs {requested} bytes with {available} bytes available"
            ),
            Self::Cancelled => write!(f, "processing was cancelled by the output sink"),
            Self::DecodeFailed => write!(f, "decoder rejected input or could not render it"),
            Self::NonFinitePixelSample => write!(f, "non-finite sample during resampling"),
            Self::InvalidEncodeOptions => {
                write!(f, "encoder quality/speed or output dimensions are invalid")
            }
            #[cfg(any(
                feature = "avif-write",
                feature = "webp-write",
                feature = "jpeg-xl-write"
            ))]
            Self::IncompleteOutput => write!(f, "output sink has not received every scanline"),
            #[cfg(feature = "jpeg-xl-write")]
            Self::UnsupportedEncoderProfile(reason) => {
                write!(f, "JPEG XL encoder profile: {reason}")
            }
            Self::EncodeFailed => write!(f, "encoder failed"),
            #[cfg(any(
                feature = "avif-read",
                feature = "webp-read",
                feature = "jpeg-xl-own-read"
            ))]
            Self::InputPixelLimitExceeded { pixels, limit } => {
                write!(
                    f,
                    "input has {pixels} pixels, exceeding the configured {limit} pixel limit"
                )
            }
            #[cfg(feature = "avif-read")]
            Self::ContradictoryColorSignals => write!(
                f,
                "container color information contradicts the coded bitstream"
            ),
            Self::UnsupportedImageTransform => write!(
                f,
                "orientation, mirroring or clean-aperture transforms are not supported yet"
            ),
            Self::InvalidAlphaValue => {
                write!(f, "straight alpha must be finite and between zero and one")
            }
            Self::UnsupportedColorEncoding => write!(
                f,
                "linear-sRGB resize requires unambiguous full-range RGB sRGB CICP; ICC/unknown/HDR need a managed transform"
            ),
            #[cfg(feature = "icc")]
            Self::CicpBesideIcc(reason) => write!(
                f,
                "CICP beside the rendered ICC profile {reason}; the ICC profile is not given precedence over CICP automatically"
            ),
            Self::UnsupportedAlphaMode => write!(
                f,
                "the channel layout requires an explicit, compatible alpha-association mode"
            ),
            Self::UnsupportedAnimation => write!(
                f,
                "animated input is not supported; only still images are decoded"
            ),
            Self::IncompleteDecoderOutput => {
                write!(f, "decoder did not provide the complete output region")
            }
            Self::UnsupportedCodecLayout => {
                write!(f, "decoder pixel or color layout is not supported")
            }
        }
    }
}

impl std::error::Error for Error {}
