//! A panic inside the pipeline does not unwind out of the public entry
//! points (ADR-0040 amendment of 2026-11-16, working decision, delegated,
//! revisable by the owner): it is returned as `ProcessingErrorKind::Internal`,
//! and in a batch only the panicking item fails. The panic is provoked
//! deterministically through the public backend API, at each stage an
//! external encoder takes part in.
#![cfg(feature = "jpeg-read")]

use ferrisma::pipeline::Threads;
use ferrisma::pipeline::{
    BatchItem, EncodeError, EncodeRequest, EncodedImage, EncoderBackend, Engine, ImageFormat,
    InputProfilePolicy, OutputFormat, ProcessingErrorKind, RowEncoder, TransformOptions,
    transcode_with,
};
use ferrisma::pipeline::{OutputGeometry, OutputSettings};

const JPEG: &[u8] = include_bytes!("fixtures/sample.jpg");

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Stage {
    Never,
    FrameBytes,
    Begin,
    WriteRow,
    Finish,
    /// A payload that is neither `&str` nor `String`.
    OpaquePayload,
}

struct Backend(Stage);
struct Rows {
    stage: Stage,
    rows: u32,
}

fn boom(stage: Stage) -> ! {
    if stage == Stage::OpaquePayload {
        std::panic::panic_any(42_u32);
    }
    // A `String` payload: the message carries the formatted text.
    panic!("test backend panics in {stage:?}");
}

impl EncoderBackend for Backend {
    fn format(&self) -> OutputFormat {
        OutputFormat::WebP
    }
    fn name(&self) -> &'static str {
        "test-panic"
    }
    fn required_frame_bytes(&self, request: &EncodeRequest) -> Result<usize, EncodeError> {
        // A `&'static str` payload.
        assert!(
            self.0 != Stage::FrameBytes,
            "test backend panics in FrameBytes"
        );
        // A reservation that a leaked budget would make visible.
        Ok(request.width as usize * request.height as usize * 4)
    }
    fn begin(&self, _: &EncodeRequest) -> Result<Box<dyn RowEncoder + '_>, EncodeError> {
        if matches!(self.0, Stage::Begin | Stage::OpaquePayload) {
            boom(self.0);
        }
        Ok(Box::new(Rows {
            stage: self.0,
            rows: 0,
        }))
    }
}

impl RowEncoder for Rows {
    fn write_row(&mut self, _: u32, _: &[u8]) -> Result<(), EncodeError> {
        self.rows += 1;
        if self.stage == Stage::WriteRow && self.rows == 3 {
            boom(self.stage);
        }
        Ok(())
    }
    fn finish(self: Box<Self>) -> Result<EncodedImage, EncodeError> {
        if self.stage == Stage::Finish {
            boom(self.stage);
        }
        Ok(EncodedImage::new(OutputFormat::WebP, vec![0; 8], 0, false))
    }
}

fn options(transform_threads: usize) -> TransformOptions {
    TransformOptions::new(OutputGeometry::Exact {
        width: 8,
        height: 8,
    })
    .with_output(OutputSettings::default_for(OutputFormat::WebP))
    .with_profile_policy(InputProfilePolicy::AssumeSrgbIfMissing)
    .with_transform_threads(
        std::num::NonZeroUsize::new(transform_threads).map_or(Threads::Automatic, Threads::Count),
    )
    // Room for one reservation of the test backend, not for two.
    .with_max_managed_bytes(1 << 20)
}

#[test]
fn a_panic_in_any_backend_stage_is_returned_as_internal() {
    for threads in [1, 4] {
        for stage in [
            Stage::FrameBytes,
            Stage::Begin,
            Stage::WriteRow,
            Stage::Finish,
        ] {
            let error = transcode_with(JPEG, ImageFormat::Jpeg, &options(threads), &Backend(stage))
                .expect_err("the panic must become an error");
            assert_eq!(error.kind(), ProcessingErrorKind::Internal, "{stage:?}");
            assert_eq!(error.refusal_reason(), None);
            let message = error.to_string();
            assert!(message.contains("a panic was caught"), "{message}");
            assert!(
                message.contains(&format!("test backend panics in {stage:?}")),
                "{stage:?}: {message}"
            );
            // The next call is unaffected: no reservation of the panicking
            // call is left in any budget, and nothing is poisoned.
            let output = transcode_with(
                JPEG,
                ImageFormat::Jpeg,
                &options(threads),
                &Backend(Stage::Never),
            )
            .expect("a call after a caught panic succeeds");
            assert_eq!((output.width, output.height), (8, 8));
        }
    }
    let opaque = transcode_with(
        JPEG,
        ImageFormat::Jpeg,
        &options(1),
        &Backend(Stage::OpaquePayload),
    )
    .unwrap_err();
    assert_eq!(opaque.kind(), ProcessingErrorKind::Internal);
    assert!(
        opaque.to_string().contains("a panic was caught"),
        "{opaque}"
    );
}

#[test]
fn a_panicking_batch_item_fails_alone() {
    let item = BatchItem::new(JPEG, ImageFormat::Jpeg, options(1));
    let items: [BatchItem<'_>; 6] = std::array::from_fn(|_| item.clone());
    let engine = Engine::new(Threads::Count(std::num::NonZeroUsize::new(3).unwrap())).unwrap();
    for stage in [Stage::Begin, Stage::WriteRow, Stage::Finish] {
        // The backend is shared by the items; it panics for one of them.
        struct OneOf {
            stage: Stage,
            count: std::sync::atomic::AtomicUsize,
        }
        impl EncoderBackend for OneOf {
            fn format(&self) -> OutputFormat {
                OutputFormat::WebP
            }
            fn name(&self) -> &'static str {
                "test-panic-once"
            }
            fn begin(
                &self,
                request: &EncodeRequest,
            ) -> Result<Box<dyn RowEncoder + '_>, EncodeError> {
                let nth = self
                    .count
                    .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                let stage = if nth == 2 { self.stage } else { Stage::Never };
                if stage == Stage::Begin {
                    boom(stage);
                }
                let _ = request;
                Ok(Box::new(Rows { stage, rows: 0 }))
            }
        }
        let backend = OneOf {
            stage,
            count: std::sync::atomic::AtomicUsize::new(0),
        };
        let results = engine.transcode_batch_with(&items, &backend);
        assert_eq!(results.len(), items.len());
        let failed: Vec<_> = results.iter().filter(|result| result.is_err()).collect();
        assert_eq!(failed.len(), 1, "{stage:?}: exactly one item panicked");
        let error = failed[0].as_ref().unwrap_err();
        assert_eq!(error.kind(), ProcessingErrorKind::Internal, "{stage:?}");
        assert!(error.to_string().contains("a panic was caught"), "{error}");
        for output in results.iter().filter_map(|result| result.as_ref().ok()) {
            assert_eq!((output.width, output.height), (8, 8));
        }
        // The same engine runs the next batch completely.
        let again = engine.transcode_batch_with(&items, &Backend(Stage::Never));
        assert!(again.iter().all(Result::is_ok), "{stage:?}");
    }
}
