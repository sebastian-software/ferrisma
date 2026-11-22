/*
 * Ferrisma's homepage on Monografie: what a user gets, in the owner's order
 * (one pipeline for every format; all Rust; memory safety where files come
 * in; the originals' pictures; an account of every image; the cost in speed),
 * then the status. The technical depth lives on /how-it-is-built and
 * /how-a-port-is-made. Facts come from ../ferrisma.ts, which names its
 * sources; the arc and the open questions are in DESIGN-NOTES.md.
 */
import type { ArdoMetaFunction } from "ardo/runtime";
import {
  MonoAction,
  MonoCode,
  MonoColophon,
  MonoClosing,
  MonoEngraving,
  MonoPassage,
  MonoSection,
  MonoTerms,
  MonoWide,
} from "ferramenta-family/monografie";

import { OneBuildDrawing } from "../drawings/OneBuild";
import { PipelineDrawing } from "../drawings/Pipeline";
import { FAMILY_SITE, REPOSITORY, SPEED_RECORD, SPEED_SUMMARY, blob, path } from "../ferrisma";

export const handle = { layout: "bare" };
export const meta: ArdoMetaFunction = () => [
  { title: "Ferrisma: one image pipeline for Rust, every codec included" },
  {
    name: "description",
    content:
      "One image pipeline for Rust, every codec included: JPEG, WebP, JPEG XL and AVIF through one API, built by Cargo alone. Crop and resize in linear light, with codecs translated from the originals and held to their output bit for bit. Experimental.",
  },
];

/*
 * Verbatim from examples/thumbnail.rs on main (CI compiles every example with
 * `cargo test -p ferrisma -p ferrisma-kernels --all-targets --all-features`), without its
 * argument parsing; `jpeg` holds the input file's bytes.
 */
const RUST_EXAMPLE = `use ferrisma::pipeline::{
    ImageFormat, InputProfilePolicy, OutputFormat, OutputGeometry, OutputSettings, ResizeFit,
    ResizeOptions, TransformOptions, transcode,
};

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
std::fs::write(output, &image.bytes)?;`;

/* Verbatim from docs/getting-started.md. */
const CLI_EXAMPLE =
  "cargo run --release --features cli -- \\\n  photo.jpg scaled.avif 1600 900 --assume-srgb --avif-quality=80 --avif-speed=8";

function Opening() {
  return (
    <section className="mono-opening fx-opening" aria-labelledby="fx-title">
      <figure className="mono-plate fx-instrument">
        <img
          src={path("ferrisma-instrument.webp")}
          width="2400"
          height="1200"
          fetchPriority="high"
          alt="Ferrisma's instrument: a gently curved steel railway whose wagons carry a mountain picture under a guillotine, a smaller copy under a spindle press, and the same picture as square pixels under a loupe."
        />
      </figure>
      <p className="mono-kicker">Ferrisma · An image pipeline for Rust</p>
      <h1 id="fx-title">
        One image pipeline for Rust, every codec included
        <span className="mono-second">
          JPEG, WebP, JPEG XL and AVIF through one API, built by Cargo alone
        </span>
      </h1>
      <p className="mono-standfirst">
        Crop, resize and convert images the way sharp does for Node.js, from a Rust application
        or the command line, with no C library underneath. The codecs are translations of the
        originals, held to their output bit for bit.
      </p>
      <div className="mono-actions">
        <MonoAction href={path("guide/getting-started")} primary detail="from the repository">
          Build from source
        </MonoAction>
        <MonoAction href="#use" detail="Rust and the CLI">
          See it in use
        </MonoAction>
      </div>
      <ul className="mono-facts fx-facts">
        <li>
          <strong>JPEG · WebP · JPEG XL · AVIF</strong>read
        </li>
        <li>
          <strong>AVIF · WebP · JPEG XL</strong>written
        </li>
        <li>
          <strong>
            <em>Bit for bit</em>
          </strong>
          what each port must match
        </li>
        <li>
          <strong>Experimental</strong>no release yet
        </li>
      </ul>
      <nav className="fx-contents" aria-label="On this page">
        <a href="#use">In use</a>
        <a href="#rust">One build</a>
        <a href="#safety">Memory safety</a>
        <a href="#originals">The originals' pictures</a>
        <a href="#report">Every image accounted for</a>
        <a href="#speed">Speed</a>
        <a href="#status">Status</a>
      </nav>
    </section>
  );
}

function InUse() {
  return (
    <MonoSection
      id="use"
      title="One pipeline for every format."
      standfirst="Bytes in, bytes out, AVIF, WebP or JPEG XL. Orientation, crop, resize and color work the same whatever comes in; only the encoder settings belong to the format you write."
    >
      <p>
        A thumbnail from a JPEG takes one call. Here it becomes a 320 × 320 WebP that covers the
        square and never enlarges the source:
      </p>
      <MonoWide>
        <MonoCode caption="From examples/thumbnail.rs, compiled in CI" file="main.rs">
          {RUST_EXAMPLE}
        </MonoCode>
      </MonoWide>
      <MonoPassage
        note={{
          side: "right",
          term: "--assume-srgb",
          children:
            "An image without color data is not sRGB by default. This flag says that it is, for this run.",
        }}
      >
        <p>The same pipeline from the command line, here writing AVIF:</p>
      </MonoPassage>
      <MonoWide>
        <MonoCode caption="From docs/getting-started.md, in a clone of the repository" file="Terminal">
          {CLI_EXAMPLE}
        </MonoCode>
      </MonoWide>
      <p>
        Behind both, every picture takes the same route: decoded, moved into linear light,
        turned upright, cropped, resized, quantized at the output's depth and encoded, with a
        report beside the file.
      </p>
      <MonoWide>
        <PipelineDrawing />
      </MonoWide>
      <p className="fx-section-link">
        <a href={path("guide/getting-started")}>Getting started</a>
        <a href={path("guide/supported-matrix")}>Every supported input and output</a>
      </p>
    </MonoSection>
  );
}

function OneBuild() {
  return (
    <MonoSection
      id="rust"
      title="All Rust, one build."
      standfirst="The codecs are Rust too, so Cargo builds the whole thing. No C compiler, no CMake, no image libraries to install on the build machine."
    >
      <p>
        Image codecs are usually C and C++ libraries, and a Rust application that uses them
        inherits their build: compilers for both languages, CMake, an assembler for the fastest
        x86 code, and a bundled copy of each library or a system package of the right version.
        Ferrisma's four codecs are Rust, so <code>cargo build</code> with the features you choose
        is the whole build, AVIF included.
      </p>
      <MonoWide>
        <OneBuildDrawing />
      </MonoWide>
      <p>
        Every feature is verified today on macOS ARM64 and GNU Linux x86-64, with Rust 1.99 or
        later. Other targets are not verified yet.
      </p>
    </MonoSection>
  );
}

function Safety() {
  return (
    <MonoSection
      id="safety"
      title="Memory safety where the files come in."
      standfirst="A decoder reads files someone else made. Ferrisma keeps the code that handles them in Rust, and says exactly where Rust's checks are switched off."
    >
      <MonoPassage
        note={{
          side: "right",
          term: "unsafe",
          children:
            "A Rust block where the compiler's memory checks do not apply and the author has to argue why the code is correct.",
        }}
      >
        <p>
          The engine forbids <code>unsafe</code> code outright, and so does the color
          conversion it shares between WebP and AVIF. The JPEG and WebP ports allow it only in
          their SIMD code; WebP also has a C interface, which the engine does not use. The JPEG XL and AVIF ports are translations that still carry much of
          their originals' unsafe code; the engine reaches them only through small bridges in which
          every unsafe block carries a written safety argument.
        </p>
        <p>
          That is not a claim that the whole stack is memory-safe. It is a map of where the
          risk sits, kept in the repository's{" "}
          <a href={blob("docs/unsafe-review.md")}>unsafe review</a>, so it can shrink port by
          port.
        </p>
      </MonoPassage>
    </MonoSection>
  );
}

function Originals() {
  return (
    <MonoSection
      id="originals"
      title="The same pictures as the originals."
      standfirst="Going to Rust does not mean trading quality or compatibility for it."
    >
      <p>
        Each codec is a translation of the library its format is measured against: libjpeg-turbo,
        libwebp, libjxl, and libavif with libaom for encoding and dav1d for decoding. Version 1.0 is defined by one test: every port
        produces exactly the original's output, bit for bit, in its plain and its fast code, on
        ARM and on x86. Today the recorded comparisons pass for WebP and the JPEG XL decoder on
        both targets, and for AVIF on ARM; the rest is the work before 1.0.
      </p>
      <p className="fx-section-link">
        <a href={path("how-it-is-built")}>How Ferrisma is built</a>
        <a href={path("how-a-port-is-made")}>How one port is made</a>
        <a href={path("how-it-is-built#gate")}>Where each port stands</a>
      </p>
    </MonoSection>
  );
}

function Report() {
  return (
    <MonoSection
      id="report"
      title="You know what happened to every image."
      standfirst="Nothing disappears quietly: the pipeline carries a part of the input, refuses the file with a reason, or says in the report what it left out."
    >
      <MonoTerms
        columns={2}
        prose
        className="fx-terms"
        items={[
          {
            term: "Refused, with a reason",
            description:
              "Animation, HDR and unsupported alpha stop with an error that names the cause, instead of a picture that is quietly wrong.",
          },
          {
            term: "Left out, and named",
            description:
              "EXIF, XMP, a gain map or an embedded profile after conversion: each class that applied is listed in the report.",
          },
          {
            term: "A budget for memory",
            description:
              "Frames, rows and scratch buffers are charged to one budget, 2 GiB by default, and the peak comes back in the report.",
          },
          {
            term: "Errors, not crashes",
            description:
              "A panic inside a conversion comes back from the call as an error, so one bad file does not take the service down.",
          },
        ]}
      />
      <p className="fx-section-link">
        <a href={path("how-it-is-built#report")}>Refuse or report, drawn out</a>
      </p>
    </MonoSection>
  );
}

function Speed() {
  return (
    <MonoSection
      id="speed"
      title="The cost: close to the originals' speed."
      standfirst="Every codec is timed against the original it translates, in the same job on the same machine."
    >
      <p>
        In the record of November 17, 2026, the ports take {SPEED_SUMMARY.min} to{" "}
        {SPEED_SUMMARY.max} times as long as the originals' SIMD builds on one thread, across{" "}
        {SPEED_SUMMARY.cells} measurements on {SPEED_SUMMARY.machines} machines;{" "}
        {SPEED_SUMMARY.countAtOrBelow} of them are at or below {SPEED_SUMMARY.atOrBelow}. The
        widest gaps are WebP's lossy encoder on x86-64 (up to 1.63) and JPEG XL's lossless decode
        on Linux ARM64 (up to 1.48).
      </p>
      <p className="fx-section-link">
        <a href={path("how-it-is-built#speed")}>The full table</a>
        <a href={SPEED_RECORD.href}>{SPEED_RECORD.title}</a>
      </p>
    </MonoSection>
  );
}

function Status() {
  return (
    <MonoSection
      id="status"
      title="Experimental, and plain about it."
      standfirst="The pipeline runs today, built from source. Nothing functional is published yet."
    >
      <MonoWide>
        <div className="fx-ledger">
          <div>
            <h3>Works today</h3>
            <ul>
              <li>The command-line tool and the Rust facade, built from the repository with Rust 1.99 or later.</li>
              <li>Still images in SDR: JPEG, WebP, JPEG XL and AVIF in; AVIF, WebP and JPEG XL out, WebP and JPEG XL lossy or lossless.</li>
              <li>Verified on macOS ARM64 and GNU Linux x86-64.</li>
            </ul>
          </div>
          <div>
            <h3>Not there yet</h3>
            <ul>
              <li>Published packages: the crate and the npm package at 0.0.0 are placeholders, and there is no Node.js binding.</li>
              <li>A stable API: the compatibility policy is still a proposal.</li>
              <li>Animation, HDR and JPEG output.</li>
              <li>Windows and other targets.</li>
            </ul>
          </div>
        </div>
      </MonoWide>
      <p>
        Next on the way to 1.0: the bit-for-bit test on both targets for every port and the
        independent evidence for image quality,
        hardening, resources and dependencies. Until then, releases will be 0.x previews.
      </p>
      <p className="fx-section-link">
        <a href={path("guide/status")}>Current status</a>
        <a href={path("guide/release-gates")}>Open release gates</a>
      </p>
    </MonoSection>
  );
}

export default function Home() {
  return (
    <div className="fx-home">
      <Opening />
      <MonoEngraving seed={10} />
      <InUse />
      <MonoEngraving seed={1} />
      <OneBuild />
      <MonoEngraving seed={2} />
      <Safety />
      <MonoEngraving seed={3} />
      <Originals />
      <MonoEngraving seed={4} />
      <Report />
      <MonoEngraving seed={5} />
      <Speed />
      <MonoEngraving seed={6} />
      <Status />
      <MonoEngraving seed={7} />
      <div className="mono-axis">
        <MonoColophon>
          Ferrisma is one of the Ferramenta engines, made by Sebastian Software; each stands on
          its own. The engine is MIT OR Apache-2.0, and the ports keep their originals' licenses
          and attribution. The instrument at the top is a Blender scene built with the family's
          shared studio.
        </MonoColophon>
      </div>
      <MonoClosing
        title="Build it. Check it. Tell us what breaks."
        standfirst="Run your own images through the pipeline, compare a port with its original, or take on one of the open gates."
        actions={
          <>
            <MonoAction href={path("guide/getting-started")} primary>
              Build from source
            </MonoAction>
            <MonoAction href={path("how-it-is-built")}>How Ferrisma is built</MonoAction>
          </>
        }
        links={
          <>
            <a href={REPOSITORY}>GitHub repository</a>
            <a href={path("guide/development")}>Contribute</a>
            <a href={FAMILY_SITE}>The Ferramenta family</a>
          </>
        }
      />
    </div>
  );
}
