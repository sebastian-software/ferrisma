//! Run with: cargo run --example thumbnail --features jpeg-read,webp-write -- photo.jpg thumb.webp

#[cfg(all(feature = "jpeg-read", feature = "webp-write"))]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    use ferrixel::pipeline::{
        ImageFormat, InputProfilePolicy, OutputFormat, ResizeFit, ResizeOptions, TransformOptions,
        transcode_resized,
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
    let image = transcode_resized(
        &jpeg,
        ImageFormat::Jpeg,
        TransformOptions {
            output_format: OutputFormat::WebP,
            // Only use this assumption when it is appropriate for the input.
            profile_policy: InputProfilePolicy::AssumeSrgbIfMissing,
            ..TransformOptions::default()
        },
        ResizeOptions {
            width: Some(320),
            height: Some(320),
            fit: ResizeFit::Cover,
            without_enlargement: true,
        },
    )?;
    std::fs::write(output, &image.bytes)?;
    println!("Wrote {}×{} WebP", image.width, image.height);
    Ok(())
}

#[cfg(not(all(feature = "jpeg-read", feature = "webp-write")))]
fn main() {
    eprintln!("Enable the jpeg-read and webp-write features to run this example.");
}
