/*
 * Ferrisma's homepage on Monografie, in the shape of the family kit's sample
 * engine page: the instrument as the opening plate, then one reading column
 * that widens for the drawings. Every fact comes from ../ferrisma.ts, which
 * names its source document; the arc and the open questions are in
 * DESIGN-NOTES.md.
 */
import type { ArdoMetaFunction } from "ardo/runtime";
import {
  MonoAction,
  MonoCaption,
  MonoClosing,
  MonoCode,
  MonoColophon,
  MonoEngraving,
  MonoPassage,
  MonoSection,
  MonoTerms,
  MonoWide,
} from "ferramenta-family/monografie";

import { AnatomyDrawing } from "../drawings/Anatomy";
import { GateDrawing } from "../drawings/Gate";
import { PipelineDrawing } from "../drawings/Pipeline";
import { RefuseOrReportDrawing } from "../drawings/RefuseOrReport";
import {
  FAMILY_SITE,
  GATE_AS_OF,
  GATE_STATE,
  JXL_DECODE_RECORD,
  PORTS,
  REPOSITORY,
  SPEED,
  SPEED_RECORD,
  blob,
  path,
} from "../ferrisma";

export const handle = { layout: "bare" };
export const meta: ArdoMetaFunction = () => [
  { title: "Ferrisma: an image pipeline in Rust, on codecs translated from the originals" },
  {
    name: "description",
    content:
      "Ferrisma reads JPEG, WebP, JPEG XL and AVIF, crops and resizes in linear light, and writes AVIF, WebP or JPEG XL. Its codecs are Rust translations of libjpeg-turbo, libwebp, libjxl and libavif, checked against the originals bit for bit. Experimental.",
  },
];

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
      <p className="mono-kicker">Ferrisma · An image pipeline in Rust</p>
      <h1 id="fx-title">
        Read, crop, resize and write images in Rust
        <span className="mono-second">
          on codecs translated from the originals, and held to them bit for bit
        </span>
      </h1>
      <p className="mono-standfirst">
        Ferrisma reads JPEG, WebP, JPEG XL and AVIF, turns the picture upright, crops and resizes
        it in linear light, and writes AVIF, WebP or JPEG XL. Its four codecs are Rust
        translations of libjpeg-turbo, libwebp, libjxl and libavif. For image services, build
        tools and Rust applications that need to know what a conversion did to every picture.
      </p>
      <div className="mono-actions">
        <MonoAction href={path("guide/getting-started")} primary detail="from the repository">
          Build from source
        </MonoAction>
        <MonoAction href="#gate" detail="the quality gate">
          How the ports are checked
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
        <a href="#pipeline">The pipeline</a>
        <a href="#ports">The ports</a>
        <a href="#gate">The gate</a>
        <a href="#report">Refuse or report</a>
        <a href="#speed">Speed</a>
        <a href="#status">Status</a>
      </nav>
    </section>
  );
}

function Pipeline() {
  return (
    <MonoSection
      id="pipeline"
      title="One pass from file to file."
      standfirst="Every picture takes the same route, whatever format it arrives in, and leaves with an account of what happened to it."
    >
      <MonoPassage
        note={{
          term: "Linear light",
          children:
            "Values proportional to light, rather than the curved values a file stores for display. Averaging black and white in light and encoding the result gives 188, not 128.",
        }}
      >
        <p>
          A file is decoded by the port of its format's original library. The pixels then move
          into linear light, with the image's own color data read rather than guessed, so that a
          resize mixes light instead of display values. Orientation comes first, so a crop refers
          to the picture as you see it; then the crop, then the resize. At the end the samples are
          quantized at the output's depth and encoded.
        </p>
      </MonoPassage>
      <MonoWide>
        <PipelineDrawing />
      </MonoWide>
      <MonoTerms
        columns={2}
        prose
        className="fx-terms"
        items={[
          {
            term: "Depth follows the input",
            description:
              "AVIF keeps 8, 10 or 12 bits and JPEG XL up to 16. WebP holds 8, and a deeper input says so in the report.",
          },
          {
            term: "A budget for memory",
            description:
              "Every frame, row and scratch buffer the engine allocates is charged to one budget, 2 GiB by default; the peak comes back in the report. A codec's own scratch is flagged as outside it.",
          },
          {
            term: "Errors, not crashes",
            description:
              "A panic inside a conversion comes back from the call as an error, so one bad file does not take the service down with it.",
          },
          {
            term: "Many images at once",
            description:
              "A parallel row pipeline and batch execution, with scalar, NEON or AVX2 resampling kernels chosen at run time.",
          },
        ]}
      />
      <p className="fx-section-link">
        <a href={path("guide/architecture")}>The architecture</a>
        <a href={path("guide/color")}>The color contract</a>
        <a href={path("guide/supported-matrix")}>Every supported input and output</a>
      </p>
    </MonoSection>
  );
}

function Ports() {
  return (
    <MonoSection
      id="ports"
      title="Four codecs, translated from the originals."
      standfirst="Not new codecs that aim to be compatible, but the original libraries, carried into Rust with their behavior intact."
    >
      <MonoPassage
        note={{
          side: "right",
          term: "Port",
          children:
            "A translation of a library into another language that keeps its behavior: the same algorithms, branches and results. Each port here is checked against the original it translates.",
        }}
      >
        <p>
          libjpeg-turbo, libwebp, libjxl, and libavif with the AV1 codec libaom are the libraries
          their formats are measured against: reference implementations, or in JPEG's case the
          SIMD-accelerated descendant of the reference libjpeg. They hold years of careful work,
          much of the fastest of it in hand-tuned vector code, and they are written in C and C++.
        </p>
        <p>
          Ferrisma translates each of them into Rust at a pinned version, keeping what the
          original decided instead of deciding it again. The reason is what Rust adds: memory
          safety the compiler checks, with <code>unsafe</code> confined to small, argued
          boundaries; one toolchain for the whole build; and no C compiler on the machine that
          builds your application.
        </p>
      </MonoPassage>
      <MonoWide>
        <AnatomyDrawing />
      </MonoWide>
      <MonoPassage>
        <p>
          No assembly is a rule, not a hope. Where an original's fastest kernels exist only as
          assembly, the port runs the translated scalar code until they are rebuilt with Rust
          intrinsics and proven against the original; the JPEG port has done this for its x86-64
          kernels. Six inline assembly sites remain in the JPEG XL port's Brotli decoder, held
          by a guard while their replacement is decided.
        </p>
      </MonoPassage>
      <p className="fx-section-link">
        {PORTS.map((port) => (
          <a key={port.format} href={path(port.docs)}>
            The {port.format} port
          </a>
        ))}
      </p>
    </MonoSection>
  );
}

function Gate() {
  return (
    <MonoSection
      id="gate"
      title="The same bytes as the original, or it is not done."
      standfirst="Version 1.0 has one definition: every port produces exactly what its original produces, on ARM and on x86."
    >
      <MonoPassage
        note={{
          term: "Bit-identical",
          children:
            "The same bytes, the same integers, the same floating-point words, down to the sign of a zero. The only exemption is the payload of a NaN.",
        }}
      >
        <p>
          The comparison runs build against build. The port's scalar code is compared with the
          original built without SIMD, and its SIMD code with the original's SIMD build at the
          same level: NEON on AArch64, AVX2 and the SSE levels the original selects on x86-64. The
          second half is the hard one, because hand-tuned vector code has to come out with the
          same bits when it is rewritten with Rust intrinsics.
        </p>
      </MonoPassage>
      <MonoWide>
        <GateDrawing />
      </MonoWide>
      <h3 className="fx-subhead">Where each port stands</h3>
      <MonoTerms
        columns={2}
        prose
        className="fx-terms"
        items={GATE_STATE.map((entry) => ({ term: entry.port, description: entry.state }))}
      />
      <MonoCaption>
        As the <a href={path("guide/quality-gate")}>quality gate guide</a> records it on{" "}
        {GATE_AS_OF}. No port meets the whole gate on both targets yet; that is the work between
        here and 1.0.
      </MonoCaption>
    </MonoSection>
  );
}

function Report() {
  return (
    <MonoSection
      id="report"
      title="Nothing disappears quietly."
      standfirst="A converter that drops what it cannot handle hands you a different picture without saying so. Ferrisma refuses, or it tells you."
    >
      <p>
        Every part of an input has one of three fates. Animation, HDR transfer functions and
        unsupported alpha are refused, with an error that names the reason. EXIF, XMP, a gain map,
        or an embedded profile once its colors are converted may be left out of a correct
        picture, and the report lists each class that applied. Everything else is carried into
        the output.
      </p>
      <MonoWide>
        <RefuseOrReportDrawing />
      </MonoWide>
      <p>
        The same holds across formats. Premultiplied alpha, a gray image or an orientation tag
        leads to the same outcome whether it arrives as AVIF or as JPEG XL, unless a format cannot
        represent it, and the error or report entry uses the same words for the same cause.
      </p>
      <p className="fx-section-link">
        <a href={blob("docs/adr/0042-no-silent-loss-refuse-or-report.md")}>
          The decision: refuse or report
        </a>
        <a href={blob("docs/adr/0043-formats-behave-alike.md")}>
          The decision: formats behave alike
        </a>
      </p>
    </MonoSection>
  );
}

function Speed() {
  return (
    <MonoSection
      id="speed"
      title="Measured against the originals, gaps included."
      standfirst="There is no speed gate for 1.0. Every codec is still timed against the original it translates, in the same job on the same machine."
    >
      <div>
        <div
          className="fx-table-wrap"
          role="region"
          aria-labelledby="fx-speed-caption"
          tabIndex={0}
        >
          <table className="mono-table fx-speed">
            <caption id="fx-speed-caption">
              Port time divided by the original's SIMD build, one thread. Above 1× the port is
              slower.
            </caption>
            <thead>
              <tr>
                <th scope="col">Operation</th>
                <th scope="col" className="mono-num">
                  x86-64
                </th>
                <th scope="col" className="mono-num">
                  ARM64
                </th>
              </tr>
            </thead>
            <tbody>
              {SPEED.map((row) => (
                <tr key={row.operation}>
                  <th scope="row">
                    {row.operation}
                    {row.note !== undefined && <small>{row.note}</small>}
                  </th>
                  <td className="mono-num" data-gap={row.x86.startsWith("not") ? "" : undefined}>
                    {row.x86}
                  </td>
                  <td className="mono-num">{row.arm}</td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
        <MonoCaption>
          12-megapixel inputs on GitHub-hosted ubuntu-24.04 (x86-64) and macos-15 (ARM64), one job
          per machine: <a href={SPEED_RECORD.href}>{SPEED_RECORD.title}</a>. JPEG XL decode, after
          its SIMD path landed: eight inputs on one Apple M1 Pro,{" "}
          <a href={JXL_DECODE_RECORD.href}>{JXL_DECODE_RECORD.title}</a>. Evidence for these
          machines, not a universal claim.
        </MonoCaption>
      </div>
      <p>
        Most operations take between one and one and a half times as long as the original's SIMD
        build. Two are far behind, and the table keeps them: WebP's lossy encoder on x86-64, and
        JPEG XL's lossy encoder, which does not use its SIMD paths yet.
      </p>
      <p>
        Reading AVIF goes through the translated libaom decoder, which is clearly slower than
        stock libavif with dav1d. dav1d is fast because of its assembly, so the next step is to
        rebuild its kernels with Rust intrinsics and measure them before the engine settles on a
        decoder.
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
              <li>Published packages: the crate and the npm package at 0.0.0 are placeholders.</li>
              <li>A stable API: the compatibility policy is still a proposal.</li>
              <li>Animation, HDR and JPEG output.</li>
              <li>Windows and other targets.</li>
            </ul>
          </div>
        </div>
      </MonoWide>
      <MonoPassage
        note={{
          side: "right",
          term: "--assume-srgb",
          children:
            "An image without color data is not sRGB by default. This flag says that it is, for this run.",
        }}
      >
        <p>
          To try it on your own images, clone the repository. This resizes a JPEG to 1600 × 900
          and writes AVIF:
        </p>
      </MonoPassage>
      <MonoCode caption="From a clone of the repository" file="Terminal">
        {
          "cargo run --release --features cli -- \\\n  photo.jpg scaled.avif 1600 900 \\\n  --assume-srgb --avif-quality=80 --avif-speed=8"
        }
      </MonoCode>
      <MonoPassage>
        <p>
          Next on the way to 1.0: the gate on both targets for every port, dav1d's kernels rebuilt
          in Rust, and the independent evidence for image quality, hardening, resources and
          dependencies. Until then, releases will be 0.x previews.
        </p>
      </MonoPassage>
      <p className="fx-section-link">
        <a href={path("guide/status")}>Current status</a>
        <a href={path("guide/release-gates")}>Open release gates</a>
        <a href={path("guide/getting-started")}>Getting started</a>
      </p>
    </MonoSection>
  );
}

export default function Home() {
  return (
    <div className="fx-home">
      <Opening />
      <MonoEngraving seed={10} />
      <Pipeline />
      <MonoEngraving seed={1} />
      <Ports />
      <MonoEngraving seed={2} />
      <Gate />
      <MonoEngraving seed={3} />
      <Report />
      <MonoEngraving seed={4} />
      <Speed />
      <MonoEngraving seed={5} />
      <Status />
      <MonoEngraving seed={6} />
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
            <MonoAction href={path("guide/quality-gate")}>Read the quality gate</MonoAction>
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
