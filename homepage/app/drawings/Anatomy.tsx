/*
 * The anatomy of the project: the engine, the safe bridges, the four ports
 * and what each translates, the shared SharpYUV core, the originals that are
 * built only as references, and what is left out on purpose.
 * Sources: README.md ("What's here"), docs/status.md ("Components in the
 * workspace": the engine forbids unsafe code; the wrappers isolate the unsafe
 * ABI; one SharpYUV core serves WebP and AVIF), docs/adr/0044-no-assembly-
 * acceleration-through-rust-intrinsics.md (D1: no assembly in the project;
 * the originals' snapshots only build the references), Cargo.lock (no `cc`
 * build dependency: no C compiler in the build).
 */
import { PORTS } from "../ferrisma";
import { Box, Drawing, Markers, Note, Wire, useMarker } from "./parts";

const BRIDGES = ["scanline API", "decode & encode API", "decoder & encoder", "read & write"];

const ABSENT = ["A C compiler in the build", "Assembly kernels", "The originals' code in a package"];

/** A struck-out item: a small cross and the words. */
function Absent({ x, y, text }: { x: number; y: number; text: string }) {
  return (
    <g>
      <path className="fx-d-cross" d={`M${x} ${y - 9}l9 9M${x + 9} ${y - 9}l-9 9`} />
      <Note x={x + 20} y={y}>
        {text}
      </Note>
    </g>
  );
}

function Wide() {
  const marker = useMarker();
  const col = 160;
  const gap = 20;
  const xs = PORTS.map((_, index) => index * (col + gap));
  return (
    <>
      <Markers id={marker} />
      <Box x={0} y={0} w={700} h={44} kind="quiet" title="Your Rust code, or the command line" center />
      <Wire points={[[350, 44], [350, 64]]} arrow marker={marker} />

      <rect className="fx-d-accent" x={0} y={66} width={700} height={112} rx={2} />
      <Note x={18} y={96} kind="title">
        The engine
      </Note>
      <Note x={18} y={120}>
        decode, color, geometry, quantize, encode
      </Note>
      <Note x={18} y={138}>
        the report and the memory budget
      </Note>
      <Note x={18} y={160} kind="mark">
        no unsafe code
      </Note>
      <Box x={470} y={84} w={212} h={76} title="CPU kernels" lines={["scalar, NEON, AVX2", "for resampling"]} />

      <Note x={740} y={234}>
        Safe bridges: the unsafe C ABI
      </Note>
      <Note x={740} y={252}>
        of a port stays inside them
      </Note>
      <Wire points={[[704, 237], [732, 237]]} dashed />
      {PORTS.map((port, index) => (
        <g key={port.format}>
          <Wire points={[[xs[index] + col / 2, 178], [xs[index] + col / 2, 216]]} />
          <Box x={xs[index]} y={216} w={col} h={42} title={BRIDGES[index]} kind="quiet" center />
          <Wire points={[[xs[index] + col / 2, 258], [xs[index] + col / 2, 284]]} />
          <Box
            x={xs[index]}
            y={284}
            w={col}
            h={118}
            serif
            title={port.format}
            lines={["translated from", port.original, port.pinned]}
          />
        </g>
      ))}
      <path className="fx-d-wire" d={`M${xs[1] + col / 2} 402V420M${xs[3] + col / 2} 402V420`} />
      <rect className="fx-d-quiet" x={xs[1]} y={420} width={xs[3] + col - xs[1]} height={34} rx={2} />
      <Note x={(xs[1] + xs[3] + col) / 2} y={442} anchor="middle">
        one SharpYUV core, shared by WebP and AVIF
      </Note>

      <rect className="fx-d-ghost" x={740} y={66} width={300} height={140} rx={2} />
      <Note x={758} y={96} kind="title">
        Absent by design
      </Note>
      {ABSENT.map((item, index) => (
        <Absent key={item} x={758} y={130 + index * 30} text={item} />
      ))}

      <rect className="fx-d-ghost" x={740} y={284} width={300} height={170} rx={2} />
      <Note x={758} y={312} kind="title">
        The originals
      </Note>
      <Note x={758} y={334}>
        built only as references for the gate
      </Note>
      {PORTS.map((port, index) => (
        <Note key={port.format} x={758} y={362 + index * 22} kind="code">
          {port.reference}
        </Note>
      ))}
      <Wire points={[[700, 343], [738, 343]]} dashed arrow marker={marker} />
    </>
  );
}

function Narrow() {
  const marker = useMarker();
  const half = 172;
  const xs = [0, 188, 0, 188];
  const bridgeTop = 284;
  const portTop = 424;
  return (
    <>
      <Markers id={marker} />
      <Box x={0} y={0} w={360} h={44} kind="quiet" title="Your Rust code, or the CLI" center />
      <Wire points={[[180, 44], [180, 62]]} arrow marker={marker} />
      <rect className="fx-d-accent" x={0} y={64} width={360} height={176} rx={2} />
      <Note x={16} y={94} kind="title">
        The engine
      </Note>
      <Note x={16} y={116}>
        decode, color, geometry, quantize,
      </Note>
      <Note x={16} y={133}>
        encode, the report, the memory budget
      </Note>
      <Note x={16} y={154} kind="mark">
        no unsafe code
      </Note>
      <Box x={16} y={168} w={328} h={56} title="CPU kernels" lines={["scalar, NEON, AVX2, for resampling"]} />

      <Note x={192} y={260}>
        safe bridges hold
      </Note>
      <Note x={192} y={277}>
        the unsafe C ABI
      </Note>
      {PORTS.map((port, index) => {
        const x = xs[index];
        const row = Math.floor(index / 2);
        const by = bridgeTop + 8 + row * 56;
        return (
          <g key={port.format}>
            <Box x={x} y={by} w={half} h={44} kind="quiet" title={BRIDGES[index]} center />
            <Box
              x={x}
              y={portTop + row * 132}
              w={half}
              h={118}
              serif
              title={port.format}
              lines={["translated from", port.original, port.pinned]}
            />
          </g>
        );
      })}
      <Wire points={[[180, 240], [180, bridgeTop + 8]]} />
      <Wire points={[[180, bridgeTop + 120], [180, portTop]]} />
      <rect className="fx-d-quiet" x={0} y={portTop + 268} width={360} height={34} rx={2} />
      <Note x={180} y={portTop + 290} anchor="middle">
        one SharpYUV core, shared by WebP and AVIF
      </Note>

      <rect className="fx-d-ghost" x={0} y={portTop + 320} width={360} height={150} rx={2} />
      <Note x={16} y={portTop + 348} kind="title">
        The originals
      </Note>
      <Note x={16} y={portTop + 368}>
        built only as references for the gate
      </Note>
      {PORTS.map((port, index) => (
        <Note key={port.format} x={16} y={portTop + 394 + index * 20} kind="code">
          {port.reference}
        </Note>
      ))}

      <rect className="fx-d-ghost" x={0} y={portTop + 486} width={360} height={136} rx={2} />
      <Note x={16} y={portTop + 514} kind="title">
        Absent by design
      </Note>
      {ABSENT.map((item, index) => (
        <Absent key={item} x={16} y={portTop + 548 + index * 28} text={item} />
      ))}
    </>
  );
}

export function AnatomyDrawing() {
  return (
    <Drawing
      label="The anatomy of Ferrisma. Your Rust code or the command line calls the engine, which has no unsafe code and carries decode, color, geometry, quantization, encoding, the report and the memory budget, with CPU kernels for resampling in scalar, NEON and AVX2. Safe bridges connect it to four ports: JPEG, translated from libjpeg-turbo 3.2.1; WebP, from libwebp 1.6.0; JPEG XL, from libjxl at revision a16d32d0; AVIF, from libavif 1.4.2 with libaom 3.15.2. WebP and AVIF share one SharpYUV core. The originals are built only as references for the gate. Absent by design: a C compiler in the build, assembly kernels, and the originals' code in a package."
      caption={
        <>
          Everything that ships is Rust. The originals stand beside the project, not inside it:
          they are compiled only to be compared against.
        </>
      }
      wideBox="0 0 1040 456"
      narrowBox="0 0 360 1054"
      wide={<Wide />}
      narrow={<Narrow />}
    />
  );
}
