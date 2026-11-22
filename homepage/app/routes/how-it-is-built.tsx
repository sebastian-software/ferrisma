/*
 * How Ferrisma is built: the technical depth behind the homepage. The ports
 * and what each translates, the quality gate and where each port stands,
 * refuse or report, and the speed table in full. Facts come from
 * ../ferrisma.ts and the generated gate state, as on the homepage.
 */
import type { ArdoMetaFunction } from "ardo/runtime";
import {
  MonoAction,
  MonoCaption,
  MonoClosing,
  MonoEngraving,
  MonoPassage,
  MonoSection,
  MonoTerms,
  MonoWide,
} from "ferramenta-family/monografie";

import { AnatomyDrawing } from "../drawings/Anatomy";
import { GateDrawing } from "../drawings/Gate";
import { RefuseOrReportDrawing } from "../drawings/RefuseOrReport";
import { PORTS, SPEED, SPEED_RECORD, SPEED_SUMMARY, blob, path } from "../ferrisma";
import { GATE_AS_OF, GATE_STATE } from "../gate-state.generated";

export const handle = { layout: "bare" };
export const meta: ArdoMetaFunction = () => [
  { title: "How Ferrisma is built" },
  {
    name: "description",
    content:
      "The four codec ports and the originals they translate, the bit-for-bit quality gate and where each port stands, refuse or report, and Ferrisma's speed against the originals.",
  },
];

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
          libjpeg-turbo, libwebp, libjxl, and libavif with the AV1 codecs libaom and dav1d are the libraries
          their formats are measured against: reference implementations, or in JPEG's case the
          SIMD-accelerated descendant of the reference libjpeg. They hold years of careful work,
          much of the fastest of it in hand-tuned vector code, and they are written in C and C++.
        </p>
        <p>
          Ferrisma translates each of them into Rust at a pinned version, keeping what the
          original decided instead of deciding it again. The reason is what Rust adds: memory
          safety the compiler checks, with <code>unsafe</code> code to be confined to small,
          argued boundaries (the JPEG XL and AVIF ports still carry much of their originals'); one
          toolchain for the whole build; and no C compiler on the machine that builds your
          application.
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
          kernels. The inline assembly that the translations still carried, in the AVIF port's
          CPU queries and in the JPEG XL port's Brotli decoder, color management and
          half-precision conversions, has been replaced by Rust; none is left, and a check on
          every change fails if any returns.
        </p>
      </MonoPassage>
      <p className="fx-section-link">
        <a href={path("how-a-port-is-made")}>How one port is made, step by step</a>
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
            "The same bytes, the same integers, the same floating-point words, down to the sign of a zero. A NaN must meet a NaN; only its payload may differ. Inputs that themselves hold NaN or infinity follow a separate recorded rule.",
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
          12-megapixel inputs on GitHub-hosted ubuntu-24.04 (x86-64) and the macos-15 virtual
          machine (ARM64), one job per codec and machine, <code>main</code> at{" "}
          <code>f4be1596</code>: <a href={SPEED_RECORD.href}>{SPEED_RECORD.title}</a>, which
          also has the 24-megapixel inputs, Blacksmith x86-64 and ARM64, and Linux ARM64.
          Evidence for these machines, not a universal claim.
        </MonoCaption>
      </div>
      <p>
        Across all {SPEED_SUMMARY.cells} measurements on {SPEED_SUMMARY.machines} machines the
        ports take {SPEED_SUMMARY.min} to {SPEED_SUMMARY.max} times as long as the originals' SIMD
        builds on one thread; {SPEED_SUMMARY.countAtOrBelow} of them are at or below{" "}
        {SPEED_SUMMARY.atOrBelow}. The cells above 1.4 are WebP's lossy encoder on x86-64 (1.42
        to 1.63), JPEG XL's lossless decode on Linux ARM64 (1.41 to 1.48), and one cell each of
        JPEG XL's lossy encoder, WebP's lossy decode and AVIF's decode (1.42 to 1.46). JPEG decoding is level with
        libjpeg-turbo (0.91 to 1.06), and JPEG XL's lossy encoder, now on its SIMD paths, takes
        0.99 to 1.42 times as long as libjxl's.
      </p>
    </MonoSection>
  );
}

export default function HowItIsBuilt() {
  return (
    <div className="fx-home fx-explainer">
      <section className="mono-opening fx-explainer-opening" aria-labelledby="fx-built-title">
        <p className="mono-kicker">Ferrisma · Under the pipeline</p>
        <h1 id="fx-built-title">How Ferrisma is built</h1>
        <p className="mono-standfirst">
          Four codecs translated from the originals, a gate that holds them to the originals
          bit for bit, a pipeline that refuses or reports, and what all of it costs in speed.
        </p>
        <nav className="fx-contents" aria-label="On this page">
          <a href="#ports">The ports</a>
          <a href="#gate">The gate</a>
          <a href="#report">Refuse or report</a>
          <a href="#speed">Speed</a>
        </nav>
      </section>
      <MonoEngraving seed={1} />
      <Ports />
      <MonoEngraving seed={2} />
      <Gate />
      <MonoEngraving seed={3} />
      <Report />
      <MonoEngraving seed={4} />
      <Speed />
      <MonoEngraving seed={5} />
      <MonoClosing
        title="See one port made, step by step."
        standfirst="The JPEG XL decoder, from the pinned libjxl to a fast path that runs by default."
        actions={
          <>
            <MonoAction href={path("how-a-port-is-made")} primary>
              How one port is made
            </MonoAction>
            <MonoAction href={path("")}>Back to the overview</MonoAction>
          </>
        }
      />
    </div>
  );
}
