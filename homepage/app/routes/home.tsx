import { useState } from "ardo/react";
import type { ArdoMetaFunction } from "ardo/runtime";
import {
  MonoAction,
  MonoCaption,
  MonoClosing,
  MonoCode,
  MonoColophon,
  MonoEngraving,
  MonoFacing,
  MonoPassage,
  MonoSection,
  MonoSheet,
  MonoTerms,
  MonoWide,
} from "ferramenta-family/monografie";

export const handle = { layout: "bare" };
export const meta: ArdoMetaFunction = () => [
  { title: "Ferrixel — image processing, with every step in view" },
  {
    name: "description",
    content:
      "An experimental Rust image pipeline and four codec ports. Explore linear-light resizing, explicit color and memory contracts, and reference-based validation.",
  },
];
const path = (route: string) => `${import.meta.env.BASE_URL}${route}`;
const source = "https://github.com/sebastian-software/ferrixel";
const stages = [
  {
    label: "Decode",
    title: "Start with what the decoder actually emits.",
    body: "JPEG, WebP, JPEG XL and AVIF input adapters describe the rendered pixels, their color metadata and their access capabilities. A format supporting regions does not mean every decoder can avoid a full frame.",
    detail: "JPEG, WebP and AVIF use workspace ports; JPEG XL uses jxl-oxide by default, with a local decoder available by explicit choice.",
  },
  {
    label: "Orient & crop",
    title: "Choose a region in the image you see.",
    body: "Orientation is applied before cropping, so crop coordinates refer to the upright image. When reorientation needs a buffer, the engine charges it to the managed budget and includes it in the report.",
    detail: "A crop request is not a promise of partial decoding.",
  },
  {
    label: "Resize",
    title: "Filter light, then return to display samples.",
    body: "Supported color is converted to linear sRGB before Area or Lanczos3 resampling. The default f32 executor is checked against an f64 reference, with explicit bounds on numerical differences.",
    detail: "CPU execution with scalar and runtime-selected SIMD kernels.",
  },
  {
    label: "Encode",
    title: "Make the output boundary explicit.",
    body: "The pipeline converts the resized image to sRGB samples and writes AVIF through rav1e or still WebP through the workspace port. Both sinks report their frame materialization. An explicit backend API supports the separate libavif adapter without requiring rav1e.",
    detail: "WebP supports lossy and lossless output. JPEG and JPEG XL output remain outside the current engine.",
  },
  {
    label: "Report",
    title: "Return the image and an account of the work.",
    body: "The processing report includes decoder requests, known frame allocations, peak managed bytes, selected kernels, threads and stage timings. Flags identify codec memory the engine cannot account for.",
    detail: "Engine-managed memory is not a total-process memory limit.",
  },
];

function PipelineExplorer() {
  const [selected, setSelected] = useState(0);
  const stage = stages[selected];
  return (
    <figure className="fx-flow" aria-labelledby="flow-caption">
      <figcaption id="flow-caption">Follow one image through the current SDR path</figcaption>
      <ol className="fx-flow-stages" aria-label="Pipeline stages">
        {stages.map((item, index) => (
          <li key={item.label}>
            <button
              type="button"
              aria-pressed={selected === index}
              aria-controls="flow-explanation"
              onClick={() => setSelected(index)}
            >
              <span className="fx-step-number" aria-hidden="true">
                {index + 1}
              </span>
              {item.label}
            </button>
          </li>
        ))}
      </ol>
      <div
        id="flow-explanation"
        className="fx-flow-explanation"
        aria-live="polite"
        aria-atomic="true"
      >
        <h3>{stage.title}</h3>
        <p>{stage.body}</p>
        <p className="fx-detail">{stage.detail}</p>
      </div>
    </figure>
  );
}

function ColorIllustration() {
  const [white, setWhite] = useState(50);
  const weight = white / 100;
  const encoded = Math.round(weight * 255);
  const linear = Math.round(
    255 * (weight <= 0.0031308 ? 12.92 * weight : 1.055 * weight ** (1 / 2.4) - 0.055),
  );
  return (
    <div className="fx-color-example">
      <MonoFacing>
        <MonoSheet label="Average the stored sRGB values" meta="Encoded samples">
          <div
            className="fx-color-swatch"
            style={{ backgroundColor: `rgb(${encoded} ${encoded} ${encoded})` }}
            aria-hidden="true"
          />
          <p>
            <strong>{encoded}</strong> / 255 per channel
          </p>
          <p className="fx-detail">
            Mixing display values directly loses the relationship to light.
          </p>
        </MonoSheet>
        <MonoSheet label="Average light, then encode sRGB" meta="Linear light">
          <div
            className="fx-color-swatch"
            style={{ backgroundColor: `rgb(${linear} ${linear} ${linear})` }}
            aria-hidden="true"
          />
          <p>
            <strong>{linear}</strong> / 255 per channel
          </p>
          <p className="fx-detail">
            This is the transfer-domain principle behind the supported resize path.
          </p>
        </MonoSheet>
      </MonoFacing>
      <div className="fx-color-control">
        <label htmlFor="white-weight">White contribution to a black–white mix</label>
        <div>
          <span aria-hidden="true">0%</span>
          <input
            id="white-weight"
            type="range"
            min="0"
            max="100"
            value={white}
            aria-valuetext={`${white}% white, ${100 - white}% black`}
            onChange={(event) => setWhite(Number(event.currentTarget.value))}
          />
          <output htmlFor="white-weight">{white}%</output>
        </div>
      </div>
      <MonoCaption>
        An sRGB transfer illustration, not a browser execution of Ferrixel. At an equal mix, the two
        results are 128 and 188. The Rust reference tests cover this midpoint.
      </MonoCaption>
    </div>
  );
}

function Opening() {
  return (
    <section className="mono-opening fx-opening" aria-labelledby="fx-title">
      <figure className="mono-plate fx-instrument">
        <img
          src={path("ferrixel-instrument.webp")}
          width="2400"
          height="1200"
          fetchPriority="high"
          alt="Ferrixel's instrument: a steel image registration bench with a pixel mosaic, an oxide crop frame, a filter carriage and a smaller output mosaic."
        />
      </figure>
      <p className="mono-kicker">Ferrixel, an experimental image pipeline in Rust</p>
      <h1 id="fx-title">
        Image processing,
        <br />
        with every step in view.
      </h1>
      <p className="mono-standfirst">
        From an encoded image to a resized AVIF or WebP, Ferrixel makes color, geometry and memory part of
        the contract. An engine, CPU kernels and four codec ports, developed together in one Rust
        workspace.
      </p>
      <div className="mono-actions">
        <MonoAction href={path("guide/getting-started")} primary detail="Build from source">
          Try the pipeline
        </MonoAction>
        <MonoAction href={path("guide/status")} detail="Capabilities and open gates">
          Explore the status
        </MonoAction>
      </div>
      <ul className="mono-facts fx-facts">
        <li>
          <strong>JPEG · WebP · JXL · AVIF</strong>current engine inputs
        </li>
        <li>
          <strong>AVIF · WebP</strong>current engine outputs
        </li>
        <li>
          <strong>4 codec ports</strong>with independent adoption gates
        </li>
        <li>
          <strong>Experimental</strong>unpublished, under validation
        </li>
      </ul>
      <nav className="fx-contents" aria-label="On this page">
        <a href="#pipeline">The pipeline</a>
        <a href="#codecs">Codec workspace</a>
        <a href="#color">Color</a>
        <a href="#memory">Memory</a>
        <a href="#evidence">Validation</a>
      </nav>
    </section>
  );
}

export default function Home() {
  return (
    <div className="fx-home">
      <Opening />
      <MonoEngraving seed={1} />
      <MonoSection
        id="pipeline"
        title="One image. A path you can explain."
        standfirst="An image pipeline is the work between opening a file and delivering the right pixels."
      >
        <MonoPassage
          note={{
            term: "Pipeline",
            children:
              "A sequence of decoding, geometry, color conversion, filtering and encoding operations.",
          }}
        >
          <p>
            A thumbnail needs more than fewer pixels. A portrait may carry an orientation tag. A
            camera profile changes what its colors mean. A filter needs neighboring samples, and a
            decoder may need a full image even when the caller asks for a small crop.
          </p>
          <p>
            Ferrixel makes these decisions explicit. The current CPU path accepts supported SDR
            input, applies orientation and crop, resizes in linear light, and encodes AVIF or WebP.
            Unsupported color and geometry return errors instead of silent reinterpretation.
          </p>
        </MonoPassage>
        <MonoWide>
          <PipelineExplorer />
        </MonoWide>
        <p className="fx-section-link">
          <a href={path("guide/architecture")}>
            Read the architecture and its implementation boundaries
          </a>
        </p>
      </MonoSection>

      <MonoEngraving seed={2} />
      <MonoSection
        id="try"
        title="A first image, from source."
        standfirst="Build the experimental CLI, choose the output size, and make the input-color policy explicit."
      >
        <p>
          The repository includes a working command-line path. The example below resizes a JPEG to
          1600 × 900 and writes AVIF using the default rav1e encoder. Run it from a clone of the
          repository; Ferrixel is not published on crates.io yet. Choose a <code>.webp</code> output
          for still WebP; add <code>--webp-lossless</code> instead of quality and speed for lossless
          compression of the pipeline's finished sRGB8 samples.
        </p>
        <MonoWide>
          <MonoCode caption="From the repository root" file="Terminal">
            {
              "cargo run --release --features cli -- \\\n  photo.jpg scaled.avif 1600 900 \\\n  --assume-srgb --quality=80 --speed=8"
            }
          </MonoCode>
        </MonoWide>
        <MonoPassage
          note={{
            side: "right",
            term: "An explicit assumption",
            children:
              "An untagged image is not automatically sRGB. Use this flag only when that assumption fits your input.",
          }}
        >
          <p>
            <code>--assume-srgb</code> supplies a policy for missing color information. A supported
            rendered RGB ICC profile has its own opt-in path, <code>--use-rendered-icc</code>.
            Neither flag adds HDR support or resolves conflicting metadata by guessing.
          </p>
        </MonoPassage>
        <details className="fx-rust-example">
          <summary>Use the same pipeline from Rust</summary>
          <p>
            In a local consumer with the <code>jpeg-read</code> and <code>avif-write</code> features
            enabled:
          </p>
          <MonoCode caption="The experimental public facade" file="main.rs">
            {`use ferrixel::pipeline::{
    transcode, ImageFormat, InputProfilePolicy,
    TransformOptions,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read("photo.jpg")?;
    let options = TransformOptions {
        output_width: 1600,
        output_height: 900,
        profile_policy: InputProfilePolicy::AssumeSrgbIfMissing,
        ..TransformOptions::default()
    };
    let output = transcode(&input, ImageFormat::Jpeg, options)?;
    std::fs::write("scaled.avif", &output.bytes)?;
    println!("Peak managed bytes: {}", output.report.peak_managed_bytes);
    Ok(())
}`}
          </MonoCode>
        </details>
        <p className="fx-section-link">
          <a href={path("guide/getting-started")}>Toolchain, prerequisites and the first run</a>
        </p>
      </MonoSection>

      <MonoEngraving seed={3} />
      <MonoSection
        id="codecs"
        title="Four codec projects. One place to develop."
        standfirst="The engine and its codec ports share a repository, a Cargo workspace and a validation entry point."
      >
        <MonoPassage>
          <p>
            JPEG, WebP, JPEG XL and AVIF bring different APIs, histories and reference
            implementations. Keeping their ports beside the engine makes an end-to-end change
            reviewable in one pull request. Package licenses, provenance and eventual releases
            remain independent.
          </p>
          <p>
            JPEG and WebP use the workspace ports for input; WebP also uses its
            port for output. AVIF input uses the workspace libavif/AOM port
            through an isolated safe wrapper. JPEG XL defaults to jxl-oxide,
            with the local reference decoder available by explicit opt-in.
          </p>
        </MonoPassage>
        <MonoWide>
          <p className="fx-table-hint">Scroll horizontally to compare all four columns.</p>
          <div
            className="fx-table-wrap"
            role="region"
            aria-label="Codec capabilities and engine integration"
            tabIndex={0}
          >
            <table className="fx-codec-table">
              <thead>
                <tr>
                  <th scope="col">Format</th>
                  <th scope="col">Through the engine today</th>
                  <th scope="col">The workspace port</th>
                  <th scope="col">Next boundary</th>
                </tr>
              </thead>
              <tbody>
                <tr>
                  <th scope="row">
                    <a href={path("codecs/jpeg")}>JPEG</a>
                    <small>libjpeg-turbo-rs</small>
                  </th>
                  <td>
                    Input through <code>libjpeg-turbo-rs</code>; full-resolution scanlines and
                    crop/skip requests.
                  </td>
                  <td>8-bit decode, scaled decoding, crop/skip, metadata and safe scanlines.</td>
                  <td>
                    Broaden real-photo performance and memory evidence. JPEG encoding is outside
                    the port's scope.
                  </td>
                </tr>
                <tr>
                  <th scope="row">
                    <a href={path("codecs/webp")}>WebP</a>
                    <small>libwebp-rs</small>
                  </th>
                  <td>
                    Still-image input and lossy or lossless output through <code>libwebp-rs</code>,
                    with budgeted pixel buffers and an explicit encoded-output limit.
                  </td>
                  <td>Decode and encode, incremental input, containers, animation and SharpYUV.</td>
                  <td>Consolidate SharpYUV with AVIF; keep animation outside the engine contract.</td>
                </tr>
                <tr>
                  <th scope="row">
                    <a href={path("codecs/jpeg-xl")}>JPEG XL</a>
                    <small>libjxl-rs</small>
                  </th>
                  <td>
                    Input through <code>jxl-oxide</code> by default, with codec-level region
                    requests; the local decoder is an explicit full-frame fallback.
                  </td>
                  <td>Incremental decoder and C-ABI surface with reference comparison suites.</td>
                  <td>Provisional APIs, incomplete encoder and full selection/conformance gates.</td>
                </tr>
                <tr>
                  <th scope="row">
                    <a href={path("codecs/avif")}>AVIF</a>
                    <small>libavif-rs and companions</small>
                  </th>
                  <td>
                    Workspace libavif/AOM input through a safe wrapper; <code>rav1e</code>
                    output. An explicit libavif encoder adapter is available separately.
                  </td>
                  <td>Container, AOM, dav1d and SharpYUV ports with generated and owned Rust.</td>
                  <td>
                    Pinned nightly and supported native hosts for AVIF input; full Argon and
                    release gates remain open.
                  </td>
                </tr>
              </tbody>
            </table>
          </div>
          <MonoCaption>
            Current integration is maintained in the{" "}
            <a href={path("guide/status")}>status overview</a>. AVIF input and the optional
            libavif encoder can share a binary; the former dav1d symbol collision is removed.
          </MonoCaption>
        </MonoWide>
        <p className="fx-section-link">
          <a href={path("guide/codec-development")}>
            Work on a codec: setup, references and checks
          </a>
        </p>
      </MonoSection>

      <MonoEngraving seed={4} />
      <MonoSection
        id="color"
        title="The numbers are only half the image."
        standfirst="Color metadata says what a pixel means. The transfer function says how its values relate to light."
      >
        <MonoPassage
          note={{
            term: "Linear light",
            children:
              "Values proportional to light, rather than the nonlinear values stored for display.",
          }}
        >
          <p>
            Take black and white. Averaging their stored sRGB values gives 128. Averaging their
            light and converting back to sRGB gives 188. A resize filter combines samples all over
            an image, so the domain of that arithmetic is part of the result.
          </p>
        </MonoPassage>
        <MonoWide>
          <ColorIllustration />
        </MonoWide>
        <MonoPassage>
          <p>
            Ferrixel inspects the decoder's emitted samples, not just the original file's tags.
            Supported RGB ICC conversion is explicit. ICC and CICP signals are not silently
            prioritized, and P3, PQ or HLG samples are never simply relabeled as sRGB.
          </p>
          <p>
            The production goal is broader than the current implementation. Today the pipeline is
            SDR; HDR, contradictory metadata and unsupported alpha cases fail closed.
          </p>
        </MonoPassage>
        <p className="fx-section-link">
          <a href={path("guide/color")}>Inspect the color contract and its scope</a>
        </p>
      </MonoSection>

      <MonoEngraving seed={5} />
      <MonoSection
        id="memory"
        title="Know where the image lives."
        standfirst="A crop may be small. The work needed to produce it may not be."
      >
        <p>
          The row executor keeps filtered rows in a bounded cache and releases them when they are no
          longer needed. JPEG emits scanlines; WebP buffers the rows needed for a crop, including
          preceding rows and filter context; AVIF input requires a decoded frame. Codec internals
          can allocate additional memory, reorientation can need another buffer, and both output
          encoders need a frame. WebP also reserves its predictable YUV planes before lossy encoding.
        </p>
        <MonoWide>
          <figure className="fx-memory-path" aria-labelledby="memory-caption">
            <div>
              <span className="fx-buffer-frame" aria-hidden="true" />
              <h3>Decoder boundary</h3>
              <p>Requested regions, full-frame buffers and opaque codec memory.</p>
            </div>
            <div>
              <span className="fx-buffer-rows" aria-hidden="true">
                <i />
                <i />
                <i />
                <i />
              </span>
              <h3>Engine working set</h3>
              <p>Budgeted rows, scratch space and explicit orientation buffers.</p>
            </div>
            <div>
              <span className="fx-buffer-frame fx-buffer-output" aria-hidden="true" />
              <h3>Encoder boundary</h3>
              <p>A reported output frame, plus opaque encoder allocations.</p>
            </div>
            <figcaption id="memory-caption">
              Allocation categories in the current path; the shapes are not a memory-size
              comparison.
            </figcaption>
          </figure>
        </MonoWide>
        <MonoPassage
          note={{
            side: "right",
            term: "Processing report",
            children:
              "Returned alongside the encoded image, with stage timings and the selected execution settings.",
          }}
        >
          <p>
            Every result distinguishes known materializations from opaque allocations. The
            managed-memory peak tells you what the engine tracked. It does not claim to cap the
            whole process or describe every allocation inside a codec.
          </p>
        </MonoPassage>
        <MonoTerms
          columns={2}
          items={[
            {
              term: "peak_managed_bytes",
              description: "Peak bytes charged to the shared engine budget.",
            },
            {
              term: "decoder_full_frame_buffers",
              description: "Known full-frame buffers at the decode boundary.",
            },
            {
              term: "orientation_buffer_bytes",
              description: "The explicit cost of reorienting source samples.",
            },
            {
              term: "opaque_encoder_memory",
              description: "Signals allocations outside the engine's accounting.",
            },
          ]}
        />
      </MonoSection>

      <MonoEngraving seed={6} />
      <MonoSection
        id="execution"
        title="Optimize a result you can check."
        standfirst="The reference comes first. Faster execution has to preserve the agreed behavior."
      >
        <p>
          The default executor uses <code>f32</code> working values, compared against a scalar
          <code> f64</code> reference. CPU kernels select scalar, NEON, AVX2 or AVX-512 paths at
          runtime where available. The parallel row pipeline must produce the same bytes as
          sequential execution; batch work can also run across images.
        </p>
        <p>
          These are implementation choices, not a universal speed claim. Performance evidence
          belongs to a specific revision, machine, corpus and quality setting. GPU execution and a
          general multi-output graph remain design work.
        </p>
        <p className="fx-section-link">
          <a href={path("guide/architecture")}>Follow the execution design and its evidence</a>
        </p>
      </MonoSection>

      <MonoEngraving seed={7} />
      <MonoSection
        id="evidence"
        title="Quick feedback. Deeper evidence."
        standfirst="A shared repository needs checks that follow the change, with heavier reference work on its own schedule."
      >
        <MonoPassage
          note={{
            term: "Oracle",
            children:
              "A pinned reference implementation used to compare behavior. Each suite defines exactly what must agree.",
          }}
        >
          <p>
            A codec port needs more than tests of its own assumptions. The reference suites compare
            Rust behavior with the original C or C++ implementation, using recorded upstream
            revisions and comparison rules. There are three complementary levels.
          </p>
        </MonoPassage>
        <MonoWide>
          <div className="fx-validation">
            <div>
              <h3>On a pull request</h3>
              <p>
                Affected-component tests, retained reference fixtures, provenance, lint and
                toolchain checks. Common changes fan out; website-only changes stay in website CI.
              </p>
              <p className="fx-detail">Fast feedback without rebuilding every live oracle.</p>
            </div>
            <div>
              <h3>Nightly and deep branch runs</h3>
              <p>
                Live oracle comparisons, sanitizers, profiles and native architecture checks where
                supported. Changes to SIMD, unsafe ABI, goldens or floating-point behavior need
                relevant deep evidence.
              </p>
              <p className="fx-detail">A missing required oracle is a failure.</p>
            </div>
            <div>
              <h3>Selected exhaustive runs</h3>
              <p>
                Long AOM original-consumer, recovery, translation and acceptance investigations run
                explicitly, with their native environment and artifacts retained.
              </p>
              <p className="fx-detail">Release gates remain independent of a green PR.</p>
            </div>
          </div>
        </MonoWide>
        <MonoCode caption="Select the component you are changing" file="Repository root">
          {"cargo xtask codecs --codec webp --stage pr"}
        </MonoCode>
        <p>
          Required nightly failures block release. Expected output is not refreshed just to make a
          test pass. The shared runner is Rust; existing harnesses move over incrementally while
          preserving their evidence.
        </p>
        <p className="fx-section-link">
          <a href={path("guide/validation")}>Validation policy</a>
          <a href={`${source}/actions`}>Current GitHub Actions runs</a>
        </p>
      </MonoSection>

      <MonoEngraving seed={8} />
      <MonoSection
        id="status"
        title="Working code, with work still ahead."
        standfirst="Ferrixel is experimental and unpublished. The M1 release gates are still open."
      >
        <p>
          You can exercise the SDR pipeline, inspect the report and work on the codec ports today.
          That is different from a production-readiness claim. The next steps are recorded as
          acceptance work with evidence, not a percentage-complete estimate.
        </p>
        <MonoTerms
          columns={2}
          prose
          items={[
            {
              term: "Image quality and hardening",
              description: "Complete independent quality, security and resource evidence for M1.",
            },
            {
              term: "Codec adoption",
              description:
                "Connect workspace ports through real end-to-end cases, preserving color, geometry and ownership.",
            },
            {
              term: "Shared SharpYUV",
              description:
                "Consolidate only after API, allocation, parity and measurement gates are satisfied.",
            },
            {
              term: "Release acceptance",
              description:
                "Review dependency licenses, native-host limits and the remaining release gates.",
            },
          ]}
        />
        <p className="fx-section-link">
          <a href={path("guide/status")}>Current component status</a>
          <a href={path("guide/release-gates")}>M1 evidence and open gates</a>
        </p>
      </MonoSection>
      <MonoEngraving seed={9} />
      <div className="mono-axis">
        <MonoColophon>
          Ferrixel is part of the Ferramenta family, made by Sebastian Software. The engine is MIT
          OR Apache-2.0; codec packages retain their own licenses and attribution. Its instrument is
          an original Blender scene built with the family's shared studio.
        </MonoColophon>
      </div>
      <MonoClosing
        title="Start with an image. Follow the evidence."
        standfirst="Build the current pipeline, explore a codec, or help close an acceptance gate. The guides and status come from the same repository as the code."
        actions={
          <>
            <MonoAction href={path("guide/getting-started")} primary>
              Build the pipeline
            </MonoAction>
            <MonoAction href={path("guide/development")}>Contribute to Ferrixel</MonoAction>
          </>
        }
        links={
          <>
            <a href={source}>GitHub repository</a>
            <a href={path("guide/status")}>Project status</a>
            <a href="https://ferramenta.dev">The Ferramenta family</a>
          </>
        }
      />
    </div>
  );
}
