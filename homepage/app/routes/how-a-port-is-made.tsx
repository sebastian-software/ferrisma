/*
 * How one port is made, worked through the JPEG XL decoder. Every statement
 * is recorded in codecs/libjxl-rs (upstream.json, docs/PROVENANCE.md, ADR 0001,
 * ADR 0004, PORT_STATUS.md) or in docs/quality-gate.md; the comment above each
 * step names its source.
 */
import type { ArdoMetaFunction } from "ardo/runtime";
import {
  MonoAction,
  MonoClosing,
  MonoEngraving,
  MonoPassage,
  MonoSection,
  MonoWide,
} from "ferramenta-family/monografie";

import { PortMakingDrawing } from "../drawings/PortMaking";
import { path } from "../ferrisma";

export const handle = { layout: "bare" };
export const meta: ArdoMetaFunction = () => [
  { title: "How one port is made: Ferrisma" },
  {
    name: "description",
    content:
      "From a pinned original to a SIMD default that runs: how Ferrisma's JPEG XL decoder was translated from libjxl and proven identical to it, step by step.",
  },
];

const RUN = "https://github.com/sebastian-software/ferrisma/actions/runs/43656118559";
const ADR_0004 = "codecs/jpeg-xl";

export default function HowAPortIsMade() {
  return (
    <div className="fx-home fx-explainer">
      <section className="mono-opening fx-explainer-opening" aria-labelledby="fx-port-title">
        <p className="mono-kicker">Ferrisma · The ports</p>
        <h1 id="fx-port-title">How one port is made</h1>
        <p className="mono-standfirst">
          From a pinned original to a fast path that runs by default, with every step checked
          against the original. The worked example is the JPEG XL decoder, translated from
          libjxl.
        </p>
      </section>
      {/* A section, as MonoSection renders one, for the page's side gutters. */}
      <section className="fx-explainer-overview">
        <div className="mono-axis">
        <MonoPassage
          note={{
            term: "Highway",
            children:
              "The C++ library libjxl writes its fast kernels with: one template per kernel, compiled for each instruction set.",
          }}
        >
          <p>
            Each of Ferrisma's four codecs is made the same way. The original stays as it is and
            is built three ways, only to be compared against. The port follows it step by step,
            and a step counts only when its output is the original's, bit for bit.
          </p>
        </MonoPassage>
        <MonoWide>
          <PortMakingDrawing />
        </MonoWide>
        </div>
      </section>

      <MonoEngraving seed={11} />
      <MonoSection title="1. Pin the original.">
        {/* upstream.json, docs/PROVENANCE.md, ADR 0001 */}
        <p>
          The port starts from one exact source: libjxl at commit <code>a16d32d0</code>, held in
          the repository as a Git submodule. Its license and patent grant are copied unchanged,
          and every ported module keeps the JPEG XL authors' copyright notice and names the source
          files it translates. Moving to a newer libjxl is a separate, reviewed migration that
          has to renew all of the evidence below.
        </p>
      </MonoSection>

      <MonoSection title="2. Translate it.">
        {/* ADR 0001; ADR 0004, Context */}
        <p>
          The translation keeps what the original decided: its module boundaries, type names,
          control flow, tables, field order, numeric widths and optimized algorithms. Only names
          follow Rust's conventions, and ownership, errors and pointers are translated
          explicitly. Nothing is replaced by a different codec, and nothing is handed back to
          C++. The first pass translates the scalar form of libjxl's kernels.
        </p>
      </MonoSection>

      <MonoSection title="3. Compare the scalar path.">
        {/* docs/quality-gate.md, libjxl-rs: SIMD oracle, NaN rule */}
        <p>
          libjxl is built a second time with its fast paths switched off, so it runs the same
          scalar code. Port and original get the same requests, and every output word has to
          match. One exception is written down: where the input itself carries a NaN or an
          infinity and two scalar builds of the original disagree, that position is not
          compared, because the original has no single answer there.
        </p>
      </MonoSection>

      <MonoSection title="4. Rebuild the fast paths.">
        {/* ADR 0004, Context and Decision */}
        <MonoPassage
          note={{
            side: "right",
            term: "Intrinsics",
            children:
              "Functions in Rust's core::arch that stand for single processor instructions, so vector code is written in Rust instead of assembly.",
          }}
        >
          <p>
            The scalar translation cannot simply be reused for speed: on NEON and AVX2 a
            multiply-add is fused into one rounding, minimum and maximum follow each
            instruction's rule for NaN, and sums depend on the number of lanes. So the port
            translates Highway itself. Each of the 67 operations its kernels use is mapped to the
            instruction Highway picks on that target, for example <code>vfmaq_f32</code> on NEON
            and <code>_mm256_fmadd_ps</code> on AVX2 for a multiply-add. A kernel is then written
            once, as in libjxl, and runs on either. All 37 Highway kernels the decoder uses are
            translated this way.
          </p>
        </MonoPassage>
      </MonoSection>

      <MonoSection title="5. Compare per target.">
        {/* ADR 0004, Oracle variants; docs/quality-gate.md, libjxl-rs recorded results */}
        <p>
          libjxl is built once more for each target, as NEON only and as AVX2 only, and the
          port's fast path is compared with the build of the same level. On AArch64 the
          layer's operations (63 at the time) were checked on November 13, 2026 on 85,085
          requests each with no finite word different, and whole
          decodes through the public
          decoder matched word for word on all 352 inputs, in float, 8-bit and 16-bit output. On
          x86-64 the AVX2 path matched in 704 decodes of the same 352 files, run under Rosetta 2
          on the development Mac.
        </p>
        <p>
          One AVX2 instruction, a fast reciprocal estimate, gives different bits on different
          processor makers. The port uses the same instruction as libjxl, so it is identical with
          the original on the same machine, as libjxl is with itself.
        </p>
      </MonoSection>

      <MonoSection title="6. Run it on x86-64 hardware.">
        {/* ADR 0004, Default selection on x86-64; docs/quality-gate.md; run 43656118559 */}
        <p>
          The x86-64 runs with the fast path switched on found two defects, both fixed before
          it became the default: a 16-byte load that assumed an alignment the rows do not
          have, and a NaN payload that one compiler build lost. An emulator is not a processor,
          though: Rosetta 2 does not fault on a misaligned load that real hardware rejects. So
          the comparisons also ran on GitHub's hosted Linux x86-64 machines, and{" "}
          <a href={RUN}>every job passed</a>, the sanitizer builds included.
        </p>
        <p>
          Only then did the fast path become the default: NEON on AArch64 since November 14,
          2026, and AVX2 on x86-64 since commit <code>7b48bc81</code>.
        </p>
        <p className="fx-section-link">
          <a href={path("guide/quality-gate")}>Where each port stands</a>
          <a href={path(ADR_0004)}>The JPEG XL port</a>
          <a href={path("how-it-is-built#ports")}>Back to the four ports</a>
        </p>
      </MonoSection>

      <MonoEngraving seed={12} />
      <MonoClosing
        title="The other three ports take the same steps."
        standfirst="JPEG, WebP and AVIF are each at a different step today; the quality gate guide records which."
        actions={
          <>
            <MonoAction href={path("guide/quality-gate")} primary>
              Read the quality gate
            </MonoAction>
            <MonoAction href={path("how-it-is-built")}>How Ferrisma is built</MonoAction>
          </>
        }
      />
    </div>
  );
}
