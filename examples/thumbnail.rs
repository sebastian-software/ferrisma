//! Run with: cargo run --example thumbnail --features jpeg-read,webp-write -- photo.jpg thumb.webp

#[cfg(all(feature = "jpeg-read", feature = "webp-write"))]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    use ferrisma::pipeline::{
        ImageFormat, InputProfilePolicy, OutputFormat, OutputGeometry, OutputSettings, ResizeFit,
        ResizeOptions, TransformOptions, transcode,
    };

    let mut args = std::env::args_os().skip(1);
    let input = args
        .next()
        .ok_or("expected input JPEG and output WebP paths")?;
    let output = args.next().ok_or("expected output WebP path")?;
    if args.next().is_some() {
        return Err("expected exactly two paths".into());
    }
    let jpeg = std::fs::read(input)?;
    let image = transcode(
        &jpeg,
        ImageFormat::Jpeg,
        &TransformOptions::new(OutputGeometry::Exact {
            width: 1,
            height: 1,
        })
        .with_output(OutputSettings::default_for(OutputFormat::WebP))
        // Only use this assumption when it is appropriate for the input.
        .with_profile_policy(InputProfilePolicy::AssumeSrgbIfMissing)
        .with_geometry(OutputGeometry::Resize(
            ResizeOptions::default()
                .with_width(320)
                .with_height(320)
                .with_fit(ResizeFit::Cover)
                .with_without_enlargement(true),
        )),
    )?;
    std::fs::write(output, &image.bytes)?;
    println!("Wrote {}×{} WebP", image.width, image.height);
    Ok(())
}

#[cfg(not(all(feature = "jpeg-read", feature = "webp-write")))]
fn main() {
    eprintln!("Enable the jpeg-read and webp-write features to run this example.");
}
