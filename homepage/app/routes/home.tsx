import type { ArdoMetaFunction } from "ardo/runtime";
import {
  MonoAction,
  MonoClosing,
  MonoColophon,
  MonoEngraving,
  MonoPassage,
  MonoSection,
} from "ferramenta-family/monografie";

export const handle = { layout: "bare" };
export const meta: ArdoMetaFunction = () => [
  { title: "Ferrixel — an image pipeline in Rust" },
  {
    name: "description",
    content: "An experimental image pipeline with explicit color, memory and codec boundaries.",
  },
];
const path = (route: string) => `${import.meta.env.BASE_URL}${route}`;

function Pipeline() {
  return (
    <figure className="fx-pipeline" aria-labelledby="pipeline-caption">
      <svg viewBox="0 0 720 260" role="img" aria-labelledby="pipeline-title pipeline-desc">
        <title id="pipeline-title">An image moving through Ferrixel</title>
        <desc id="pipeline-desc">
          Encoded input passes through decode, a planned sequence of image operations, and an
          encoder to become output.
        </desc>
        <rect className="fx-paper" x="20" y="35" width="680" height="188" rx="2" />
        <path className="fx-rail" d="M76 128H644" />
        <g className="fx-station">
          <rect x="62" y="90" width="92" height="76" />
          <rect x="223" y="90" width="92" height="76" />
          <rect x="405" y="90" width="92" height="76" />
          <rect x="566" y="90" width="92" height="76" />
        </g>
        <g className="fx-lens">
          <circle cx="108" cy="128" r="19" />
          <path d="M99 128h18M108 119v18" />
          <path d="M245 128h48M250 117h38M250 139h38" />
          <path d="M426 117h50M426 128h50M426 139h50" />
          <circle cx="612" cy="128" r="19" />
          <path d="M604 128h16" />
        </g>
        <g className="fx-svg-label">
          <text x="108" y="193">
            Input
          </text>
          <text x="269" y="193">
            Decode
          </text>
          <text x="451" y="193">
            Process
          </text>
          <text x="612" y="193">
            Output
          </text>
        </g>
      </svg>
      <figcaption id="pipeline-caption">
        The current SDR path: decode, orient and crop, resize in linear light, then encode AVIF.
      </figcaption>
    </figure>
  );
}
export default function Home() {
  return (
    <div className="fx-home">
      <section className="fx-opening" aria-labelledby="fx-title">
        <div className="fx-opening-inner">
          <p className="fx-kicker">
            Ferrixel <span> / </span> Experimental Rust image pipeline
          </p>
          <h1 id="fx-title">Images, with every step in view.</h1>
          <p className="mono-standfirst">
            Decode, transform and encode through an explicit pipeline. Ferrixel is being built to
            make image semantics, memory and the work between codecs visible.
          </p>
          <div className="mono-actions">
            <MonoAction href={path("guide/getting-started")} primary detail="A first pipeline">
              Explore the guide
            </MonoAction>
            <MonoAction href={path("guide/status")} detail="What works today">
              Read the status
            </MonoAction>
          </div>
          <Pipeline />
        </div>
      </section>
      <MonoSection
        id="pipeline"
        title="One image. A deliberate path."
        standfirst="The experimental SDR pipeline takes an image from input to AVIF."
      >
        <MonoPassage>
          <p>
            JPEG, JPEG XL and AVIF inputs are decoded, oriented and cropped. Area or Lanczos3
            filtering resizes in linear light before the result is converted to sRGB and encoded as
            AVIF.
          </p>
          <p>
            The public <code>transcode</code> entry point uses the rav1e encoder;{" "}
            <code>transcode_with</code> accepts an encoder backend, including the separate libavif
            adapter.
          </p>
        </MonoPassage>
      </MonoSection>
      <MonoEngraving seed={1} />
      <MonoSection
        id="color"
        title="Color is part of the contract."
        standfirst="Pixels need more than a channel count."
      >
        <MonoPassage>
          <p>
            Color space, transfer function, bit depth and alpha describe what samples mean. Ferrixel
            checks the decoder's emitted samples and performs explicit conversion where supported.
          </p>
          <p>
            The current pipeline produces SDR output. HDR and unsupported or ambiguous color
            profiles fail rather than silently being treated as sRGB.
          </p>
        </MonoPassage>
      </MonoSection>
      <MonoEngraving seed={2} />
      <MonoSection
        id="memory"
        title="Know where the image lives."
        standfirst="The processing report makes important costs visible."
      >
        <MonoPassage>
          <p>
            Each result reports decoder requests, full-frame buffers, orientation-buffer bytes,
            encoder-frame bytes and peak managed memory. It also marks decoder and encoder memory
            that remains opaque to the engine.
          </p>
          <p>
            Partial decoding depends on what the chosen decoder actually exposes. These observations
            do not measure all process memory.
          </p>
        </MonoPassage>
      </MonoSection>
      <MonoEngraving seed={3} />
      <MonoSection
        id="codecs"
        title="A family of codec ports."
        standfirst="A port in the workspace and a codec used by the engine are different things."
      >
        <MonoPassage>
          <p>
            The engine currently uses interim codec dependencies. The reference ports have their own
            APIs, tests and adoption work.
          </p>
          <div className="fx-table-wrap" role="region" aria-label="Codec support" tabIndex={0}>
            <table className="fx-codec-table">
              <thead>
                <tr>
                  <th scope="col">Format</th>
                  <th scope="col">Engine today</th>
                  <th scope="col">Workspace port</th>
                </tr>
              </thead>
              <tbody>
                <tr>
                  <th scope="row">
                    <a href={path("codecs/jpeg")}>JPEG</a>
                  </th>
                  <td>Input via zune-jpeg</td>
                  <td>libjpeg-turbo decoder</td>
                </tr>
                <tr>
                  <th scope="row">
                    <a href={path("codecs/webp")}>WebP</a>
                  </th>
                  <td>Not integrated</td>
                  <td>libwebp decoder and encoder</td>
                </tr>
                <tr>
                  <th scope="row">
                    <a href={path("codecs/jpeg-xl")}>JPEG XL</a>
                  </th>
                  <td>Input via jxl-oxide</td>
                  <td>libjxl port</td>
                </tr>
                <tr>
                  <th scope="row">
                    <a href={path("codecs/avif")}>AVIF</a>
                  </th>
                  <td>Input via avif-decode; output via rav1e</td>
                  <td>libavif port and optional encoder adapter</td>
                </tr>
              </tbody>
            </table>
          </div>
        </MonoPassage>
      </MonoSection>
      <MonoEngraving seed={4} />
      <MonoSection
        id="evidence"
        title="Correctness before acceleration."
        standfirst="Performance work starts with a reference and a way to measure the result."
      >
        <MonoPassage>
          <p>
            Pull requests run scoped Rust and integration checks on affected components. Nightly
            validation runs deeper differential, sanitizer and native-platform suites; selected
            exhaustive investigations remain manually dispatched.
          </p>
          <p>
            Codec ports retain upstream references and record comparison limits. A passing fast
            check is not a substitute for a live oracle run.
          </p>
          <p>
            <a href={path("guide/validation")}>See how validation works →</a>
          </p>
        </MonoPassage>
      </MonoSection>
      <MonoEngraving seed={5} />
      <MonoSection
        id="status"
        title="An engine in development."
        standfirst="Ferrixel is experimental and is not ready for production use."
      >
        <MonoPassage>
          <p>
            The current pipeline is CPU-based. GPU execution, broader color handling and additional
            codec integrations remain future work. The status separates working code from plans.
          </p>
          <p>
            <a href={path("guide/status")}>Read the current status →</a>
          </p>
        </MonoPassage>
      </MonoSection>
      <MonoClosing
        title="Follow the work."
        standfirst="Start with the pipeline, then explore the decisions and evidence behind it."
        actions={
          <>
            <MonoAction href={path("guide/getting-started")} primary>
              Get started
            </MonoAction>
            <MonoAction href="https://github.com/sebastian-software/ferrixel">
              View the source
            </MonoAction>
          </>
        }
      />
      <div className="mono-axis">
        <MonoColophon>Ferrixel is an open source project in the Ferramenta family.</MonoColophon>
      </div>
    </div>
  );
}
