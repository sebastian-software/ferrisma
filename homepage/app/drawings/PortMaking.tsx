/*
 * How one port is made, with the JPEG XL decoder as the example: two lanes,
 * the original above and the port below, and six steps from left to right.
 * The comparisons join the lanes; the last step leaves the development
 * machine for x86-64 hardware.
 * Sources: codecs/libjxl-rs/upstream.json and docs/PROVENANCE.md (the pin),
 * docs/adr/0001-mechanical-port.md (the translation), docs/adr/0004-simd-paths.md
 * (the Highway operations layer, the oracle variants, the default selection),
 * docs/quality-gate.md (the recorded results and run 43656118559).
 */
import { Box, Drawing, Markers, Note, Wire, useMarker } from "./parts";

type Step = {
  title: string;
  lines: [string, string, string];
  /** Which lane the step works in. */
  lane: "original" | "port" | "both";
};

const STEPS: Step[] = [
  { title: "Pin", lines: ["libjxl at commit", "a16d32d0, held as", "a Git submodule"], lane: "original" },
  { title: "Translate", lines: ["the scalar path:", "same modules, flow,", "tables and widths"], lane: "port" },
  { title: "Compare", lines: ["with libjxl built", "scalar only, word", "for word"], lane: "both" },
  { title: "Rebuild SIMD", lines: ["Highway's operations", "mapped one by one to", "Rust intrinsics"], lane: "port" },
  { title: "Compare", lines: ["per target: NEON with", "libjxl's NEON build,", "AVX2 with its AVX2"], lane: "both" },
  { title: "Run hosted", lines: ["on x86-64 hardware;", "then the SIMD path", "becomes the default"], lane: "both" },
];

/** The comparator, as in the gate drawing. */
function Same({ cx, cy, r = 18 }: { cx: number; cy: number; r?: number }) {
  const h = r * 0.45;
  const g = r * 0.28;
  return (
    <g>
      <circle className="fx-d-ring" cx={cx} cy={cy} r={r} />
      <path className="fx-d-equal" d={`M${cx - h} ${cy - g}h${2 * h}M${cx - h} ${cy}h${2 * h}M${cx - h} ${cy + g}h${2 * h}`} />
    </g>
  );
}

function Wide() {
  const marker = useMarker();
  const col = 136;
  const gap = 16;
  const x = (index: number) => 144 + index * (col + gap);
  const top = 26;
  const bottom = 176;
  const laneH = 64;
  return (
    <>
      <Markers id={marker} />
      <Note x={0} y={top + 30} kind="label">
        libjxl
      </Note>
      <Note x={0} y={top + 48}>
        C++, the original
      </Note>
      <Note x={0} y={bottom + 30} kind="label">
        libjxl-rs
      </Note>
      <Note x={0} y={bottom + 48}>
        Rust, the port
      </Note>

      {/* Original lane */}
      <Box x={x(0)} y={top} w={col} h={laneH} title="a16d32d0" lines={["pinned source"]} kind="frame" />
      <Box x={x(2)} y={top} w={col} h={laneH} title="Scalar build" lines={["scalar only"]} kind="quiet" />
      <Box x={x(4)} y={top} w={col} h={laneH} title="SIMD builds" lines={["NEON and AVX2"]} kind="quiet" />
      <Wire points={[[x(0) + col, top + 32], [x(2) - 4, top + 32]]} arrow marker={marker} />
      <Wire points={[[x(2) + col, top + 32], [x(4) - 4, top + 32]]} arrow marker={marker} />

      {/* Port lane */}
      <Box x={x(1)} y={bottom} w={col} h={laneH} title="Scalar Rust" lines={["the translation"]} kind="frame" />
      <Box x={x(3)} y={bottom} w={col} h={laneH} title="SIMD Rust" lines={["NEON and AVX2"]} kind="frame" />
      <Wire points={[[x(0) + col / 2, top + laneH], [x(0) + col / 2, bottom + 32], [x(1) - 4, bottom + 32]]} arrow marker={marker} />
      <Wire points={[[x(1) + col, bottom + 32], [x(3) - 4, bottom + 32]]} arrow marker={marker} />

      {/* Comparisons join the lanes */}
      <Wire points={[[x(2) + col / 2, top + laneH], [x(2) + col / 2, 120]]} accent />
      <Wire points={[[x(1) + col, bottom + 14], [x(2) + col / 2, bottom + 14], [x(2) + col / 2, 140]]} accent />
      <Same cx={x(2) + col / 2} cy={130} />
      <Wire points={[[x(4) + col / 2, top + laneH], [x(4) + col / 2, 120]]} accent />
      <Wire points={[[x(3) + col, bottom + 14], [x(4) + col / 2, bottom + 14], [x(4) + col / 2, 140]]} accent />
      <Same cx={x(4) + col / 2} cy={130} />

      {/* Hosted run */}
      <rect className="fx-d-accent" x={x(5)} y={top} width={col} height={bottom + laneH - top} rx={2} />
      <Note x={x(5) + 14} y={top + 26} kind="label">
        Hosted Linux
      </Note>
      <Note x={x(5) + 14} y={top + 44}>
        x86-64 hardware
      </Note>
      <Same cx={x(5) + col / 2} cy={130} />
      <Note x={x(5) + 14} y={bottom + 30}>
        every job passed,
      </Note>
      <Note x={x(5) + 14} y={bottom + 48} kind="code">
        run 43656118559
      </Note>
      <Wire points={[[x(4) + col / 2 + 18, 130], [x(5) + col / 2 - 22, 130]]} arrow marker={`${marker}-accent`} accent dashed />

      {/* Steps */}
      {STEPS.map((step, index) => (
        <g key={index}>
          <Note x={x(index)} y={bottom + laneH + 46} kind="mark">
            {String(index + 1)}
          </Note>
          <Note x={x(index) + 16} y={bottom + laneH + 46} kind="title">
            {step.title}
          </Note>
          {step.lines.map((line, row) => (
            <Note key={row} x={x(index)} y={bottom + laneH + 70 + row * 17}>
              {line}
            </Note>
          ))}
        </g>
      ))}
    </>
  );
}

/** One step per row: which lane it works in on the left, the words beside it. */
function Narrow() {
  const height = 96;
  const step = 108;
  return (
    <>
      <Note x={8} y={16} kind="label">
        libjxl (C++)
      </Note>
      <Note x={8} y={34} kind="label">
        libjxl-rs (Rust)
      </Note>
      <rect className="fx-d-lane-on" x={150} y={6} width={14} height={14} />
      <rect className="fx-d-lane-on-port" x={150} y={24} width={14} height={14} />
      <Note x={172} y={34}>
        the lane each step works in
      </Note>
      {STEPS.map((item, index) => {
        const y = 52 + index * step;
        const last = index === STEPS.length - 1;
        return (
          <g key={index}>
            <rect className={last ? "fx-d-accent" : "fx-d-frame"} x={0} y={y} width={360} height={height} rx={2} />
            <rect
              className={item.lane === "port" ? "fx-d-hair-box" : "fx-d-lane-on"}
              x={12}
              y={y + 14}
              width={22}
              height={30}
            />
            <rect
              className={item.lane === "original" ? "fx-d-hair-box" : "fx-d-lane-on-port"}
              x={12}
              y={y + 52}
              width={22}
              height={30}
            />
            {item.lane === "both" && <Same cx={23} cy={y + 48} r={9} />}
            <Note x={52} y={y + 30} kind="mark">
              {String(index + 1)}
            </Note>
            <Note x={70} y={y + 30} kind="title">
              {item.title}
            </Note>
            {item.lines.map((line, row) => (
              <Note key={row} x={52} y={y + 52 + row * 17}>
                {line}
              </Note>
            ))}
          </g>
        );
      })}
    </>
  );
}

export function PortMakingDrawing() {
  return (
    <Drawing
      label="How one port is made, with the JPEG XL decoder. Six steps across two lanes, libjxl in C++ above and libjxl-rs in Rust below. One: pin libjxl at commit a16d32d0. Two: translate its scalar path into Rust, keeping modules, control flow, tables and widths. Three: compare the Rust scalar path with libjxl built scalar only, word for word. Four: rebuild the SIMD paths, mapping Highway's operations one by one to Rust intrinsics for NEON and AVX2. Five: compare per target, NEON with libjxl's NEON build and AVX2 with its AVX2 build. Six: run the comparisons hosted on x86-64 hardware, where every job passed in run 43656118559; then the SIMD path becomes the default."
      caption={
        <>
          The original is never changed and never shipped; it is built three ways only to be
          compared against. Steps three and five are the quality gate.
        </>
      }
      wideBox="0 0 1040 356"
      narrowBox={`0 0 360 ${52 + 6 * 108}`}
      wide={<Wide />}
      narrow={<Narrow />}
    />
  );
}
