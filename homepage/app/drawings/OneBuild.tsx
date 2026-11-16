/*
 * One build: a Rust application that binds the C and C++ originals, beside
 * one that uses Ferrisma. What the originals' builds use is what this
 * repository needs to build them as references: C and C++ compilers and
 * CMake (the ports' tools directories, docs/quality-gate.md), and NASM for the x86
 * assembly of libjpeg-turbo and libaom (ADR-0044). Ferrisma's side: no `cc`
 * build dependency in Cargo.lock, the ports are Rust (README.md).
 */
import { Box, Drawing, Markers, Note, Wire, useMarker } from "./parts";

const ORIGINALS = ["libjpeg-turbo", "libwebp", "libjxl", "libavif, libaom"];
const PORTS = ["JPEG port", "WebP port", "JPEG XL port", "AVIF port"];

type Side = {
  title: string;
  middle: string;
  middleNote: string;
  leaves: string[];
  leafNotes: string[];
  needs: string[];
  accent: boolean;
};

const BIND: Side = {
  title: "Binding the originals",
  middle: "Binding crates",
  middleNote: "one per library",
  leaves: ORIGINALS,
  leafNotes: ["C", "C", "C++", "C"],
  needs: ["C and C++ compilers, CMake,", "NASM for the x86 assembly"],
  accent: false,
};
const OURS: Side = {
  title: "With Ferrisma",
  middle: "ferrisma",
  middleNote: "one API, one CLI",
  leaves: PORTS,
  leafNotes: ["Rust", "Rust", "Rust", "Rust"],
  needs: ["Cargo", ""],
  accent: true,
};

function Panel({ side, x, w, marker }: { side: Side; x: number; w: number; marker: string }) {
  const leafW = (w - 40 - 3 * 10) / 4;
  const cx = x + w / 2;
  return (
    <g>
      <rect className={side.accent ? "fx-d-accent" : "fx-d-ghost"} x={x} y={0} width={w} height={330} rx={2} />
      <Note x={x + 20} y={32} kind="title">
        {side.title}
      </Note>
      <Box x={cx - 110} y={52} w={220} h={40} title="Your Rust application" kind="quiet" center />
      <Wire points={[[cx, 92], [cx, 112]]} arrow marker={marker} />
      <Box x={cx - 110} y={116} w={220} h={52} title={side.middle} lines={[side.middleNote]} center />
      {side.leaves.map((leaf, index) => {
        const lx = x + 20 + index * (leafW + 10);
        return (
          <g key={leaf}>
            <Wire points={[[cx, 168], [cx, 180], [lx + leafW / 2, 180], [lx + leafW / 2, 196]]} arrow marker={marker} />
            <Box x={lx} y={200} w={leafW} h={52} title={leaf} lines={[side.leafNotes[index]]} kind={side.accent ? "frame" : "quiet"} center top={22} />
          </g>
        );
      })}
      <Note x={x + 20} y={284} kind="mark">
        The build needs
      </Note>
      {side.needs.map((line, index) => (
        <Note key={index} x={x + 20} y={304 + index * 17}>
          {line}
        </Note>
      ))}
    </g>
  );
}

export function OneBuildDrawing() {
  const wideMarker = useMarker();
  const narrowMarker = useMarker();
  return (
    <Drawing
      label="Two ways to get the four codecs into a Rust application. Binding the originals: the application uses one binding crate per library, libjpeg-turbo, libwebp, libjxl and libavif with libaom, written in C and C++; the build needs C and C++ compilers, CMake, and NASM for the x86 assembly. With Ferrisma: the application uses one crate, ferrisma, with one API and one CLI, over four ports written in Rust; the build needs Cargo."
      caption={
        <>
          The originals' builds are what this repository uses to compile them as references for
          the gate. In Ferrisma the codecs themselves are Rust, so Cargo is the whole build.
        </>
      }
      wideBox="0 0 1040 332"
      narrowBox="0 0 360 676"
      wide={
        <>
          <Markers id={wideMarker} />
          <Panel side={BIND} x={0} w={510} marker={wideMarker} />
          <Panel side={OURS} x={530} w={510} marker={wideMarker} />
        </>
      }
      narrow={
        <>
          <Markers id={narrowMarker} />
          <g>
            <Panel side={{ ...BIND, leaves: ["JPEG", "WebP", "JPEG XL", "AVIF"] }} x={0} w={360} marker={narrowMarker} />
          </g>
          <g transform="translate(0 344)">
            <Panel side={{ ...OURS, leaves: ["JPEG", "WebP", "JPEG XL", "AVIF"] }} x={0} w={360} marker={narrowMarker} />
          </g>
        </>
      }
    />
  );
}
