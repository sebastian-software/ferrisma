/*
 * The quality gate as a picture: one input goes to the original and to the
 * port, build kind against the same build kind, per target, and the outputs
 * must be identical. Rules from docs/adr/0041-v1-quality-gate-equivalence-
 * with-the-ported-original.md and docs/quality-gate.md ("The gate in one
 * paragraph"): scalar against scalar, NEON against NEON on AArch64, AVX2 and
 * the lower levels the original selects against the same on x86-64; integers
 * exact, floats exact down to signed zeros, NaN payloads exempt; no
 * performance gate.
 */
import { Box, Drawing, Markers, Note, Tile, Wire, useMarker } from "./parts";

const TARGETS = [
  {
    name: "AArch64",
    note: "NEON is the SIMD level in scope",
    rows: [
      { kind: "scalar", original: "scalar C/C++ build", port: "scalar Rust" },
      { kind: "NEON", original: "its NEON build", port: "Rust intrinsics" },
    ],
  },
  {
    name: "x86-64",
    note: "AVX2, and the SSE levels the original picks",
    rows: [
      { kind: "scalar", original: "scalar C/C++ build", port: "scalar Rust" },
      { kind: "AVX2", original: "its AVX2 build", port: "Rust intrinsics" },
    ],
  },
];

/** The comparator: two outputs meet in one ring. */
function Same({ cx, cy, r }: { cx: number; cy: number; r: number }) {
  return (
    <g>
      <circle className="fx-d-ring" cx={cx} cy={cy} r={r} />
      <path className="fx-d-equal" d={`M${cx - r / 2.4} ${cy - 5}h${r / 1.2}M${cx - r / 2.4} ${cy}h${r / 1.2}M${cx - r / 2.4} ${cy + 5}h${r / 1.2}`} />
    </g>
  );
}

function Wide() {
  const marker = useMarker();
  const panelTop = 112;
  const rowHeight = 96;
  return (
    <>
      <Markers id={marker} />
      <Tile x={400} y={0} w={240} h={52} label="One input" note="the same file, the same options" />
      <Wire points={[[520, 52], [520, 76]]} />
      <Wire points={[[255, 76], [785, 76]]} />
      <Wire points={[[255, 76], [255, panelTop]]} arrow marker={marker} />
      <Wire points={[[785, 76], [785, panelTop]]} arrow marker={marker} />
      {TARGETS.map((target, t) => {
        const x = t * 530;
        return (
          <g key={target.name}>
            <rect className="fx-d-quiet" x={x} y={panelTop} width={510} height={262} rx={2} />
            <Note x={x + 20} y={panelTop + 34} kind="title">
              {target.name}
            </Note>
            <Note x={x + 490} y={panelTop + 34} anchor="end">
              {target.note}
            </Note>
            {target.rows.map((row, r) => {
              const y = panelTop + 58 + r * rowHeight;
              return (
                <g key={row.kind}>
                  <Box
                    x={x + 20}
                    y={y}
                    w={176}
                    h={70}
                    title={`Original, ${row.kind}`}
                    lines={[row.original]}
                  />
                  <Box
                    x={x + 314}
                    y={y}
                    w={176}
                    h={70}
                    title={`Port, ${row.kind}`}
                    lines={[row.port]}
                  />
                  <Wire points={[[x + 196, y + 35], [x + 228, y + 35]]} accent />
                  <Wire points={[[x + 314, y + 35], [x + 282, y + 35]]} accent />
                  <Same cx={x + 255} cy={y + 35} r={26} />
                </g>
              );
            })}
          </g>
        );
      })}
      <Note x={520} y={406} anchor="middle">
        Bytes and integers match exactly; so does every floating-point word, down to the sign of
        a zero. A NaN only has to meet a NaN.
      </Note>
      <Note x={520} y={428} anchor="middle">
        There is no speed gate. A failed comparison is a finding: no expected output is refreshed
        to make it pass.
      </Note>
    </>
  );
}

function Narrow() {
  const marker = useMarker();
  const panelHeight = 252;
  return (
    <>
      <Markers id={marker} />
      <Tile x={80} y={0} w={200} h={52} label="One input" note="same file, same options" />
      <Wire points={[[180, 52], [180, 80]]} arrow marker={marker} />
      {TARGETS.map((target, t) => {
        const top = 84 + t * (panelHeight + 16);
        return (
          <g key={target.name}>
            <rect className="fx-d-quiet" x={0} y={top} width={360} height={panelHeight} rx={2} />
            <Note x={14} y={top + 30} kind="title">
              {target.name}
            </Note>
            <Note x={14} y={top + 50}>
              {target.note}
            </Note>
            {target.rows.map((row, r) => {
              const y = top + 70 + r * 88;
              return (
                <g key={row.kind}>
                  <Box x={12} y={y} w={128} h={68} title="Original" lines={[row.kind, "C/C++ build"]} top={22} />
                  <Box x={220} y={y} w={128} h={68} title="Port" lines={[row.kind, "Rust"]} top={22} />
                  <Wire points={[[140, y + 34], [160, y + 34]]} accent />
                  <Wire points={[[220, y + 34], [200, y + 34]]} accent />
                  <Same cx={180} cy={y + 34} r={20} />
                </g>
              );
            })}
            {t === 0 && <Wire points={[[180, top + panelHeight], [180, top + panelHeight + 16]]} />}
          </g>
        );
      })}
      {[
        "Bytes and integers match exactly; so does",
        "every float, down to the sign of a zero.",
        "A NaN only has to meet a NaN.",
        "No speed gate. A failed comparison is a",
        "finding, never a reason to refresh the",
        "expected output.",
      ].map((line, index) => (
        <Note key={index} x={0} y={84 + 2 * panelHeight + 16 + 30 + index * 19 + (index > 2 ? 10 : 0)}>
          {line}
        </Note>
      ))}
    </>
  );
}

export function GateDrawing() {
  return (
    <Drawing
      label="The quality gate. One input, with the same options, goes to the original and to the port. On AArch64, the original's scalar build is compared with the port's scalar path, and the original's NEON build with the port's NEON path. On x86-64, scalar against scalar, and the original's AVX2 build, with the SSE levels it selects, against the port's AVX2 path. Each pair must produce identical output, bit for bit."
      caption={
        <>
          Build kind against the same build kind, on each target. The port's SIMD is written with
          Rust intrinsics and has to reproduce the original's SIMD build, not only its scalar
          one.
        </>
      }
      wideBox="0 0 1040 436"
      narrowBox={`0 0 360 ${84 + 2 * 252 + 16 + 30 + 5 * 19 + 24}`}
      wide={<Wide />}
      narrow={<Narrow />}
    />
  );
}
