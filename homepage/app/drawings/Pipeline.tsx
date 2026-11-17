/*
 * The pipeline as a cross-section: four formats enter, one channel carries
 * the picture through five stations under one memory budget, and the image
 * and its report leave side by side. Stages and their order follow
 * docs/status.md ("What works through the engine"); depths follow
 * docs/supported-matrix.md ("Outputs"); the report's lines are fields and
 * classes named in docs/supported-matrix.md ("Discarded report") and
 * docs/status.md; the budget is the 2 GiB default of the supported matrix.
 */
import { Box, Drawing, Markers, Note, Tile, Wire, useMarker } from "./parts";

const FORMATS = ["JPEG", "WebP", "JPEG XL", "AVIF"];

const STATIONS = [
  { title: "Decode", lines: ["the original's", "decoder, ported"], icon: "decode" },
  { title: "Linear light", lines: ["color made", "explicit, in f32"], icon: "linear" },
  { title: "Geometry", lines: ["orient, then crop,", "then resize"], icon: "geometry" },
  { title: "Quantize", lines: ["at the output", "depth, 8 to 16 bit"], icon: "quantize" },
  { title: "Encode", lines: ["AVIF, WebP", "or JPEG XL"], icon: "encode" },
] as const;

type Icon = (typeof STATIONS)[number]["icon"];

/** The picture as each station leaves it, drawn in a 44 × 44 box centered on (cx, cy). */
function StationIcon({ icon, cx, cy }: { icon: Icon; cx: number; cy: number }) {
  const x = cx - 22;
  const y = cy - 22;
  switch (icon) {
    case "decode":
      return (
        <g>
          {[0, 1, 2].flatMap((row) =>
            [0, 1, 2].map((col) => (
              <rect
                key={`${row}-${col}`}
                className={(row + col) % 2 === 0 ? "fx-d-cell" : "fx-d-cell-light"}
                x={x + 2 + col * 14}
                y={y + 2 + row * 14}
                width={12}
                height={12}
              />
            )),
          )}
        </g>
      );
    case "linear":
      return (
        <g>
          <rect className="fx-d-quiet" x={x} y={y} width={44} height={44} />
          <path className="fx-d-wire fx-d-dashed" d={`M${x + 4} ${y + 40}L${x + 40} ${y + 4}`} />
          <path
            className="fx-d-wire"
            d={`M${x + 4} ${y + 40}C${x + 22} ${y + 39} ${x + 33} ${y + 28} ${x + 40} ${y + 4}`}
          />
        </g>
      );
    case "geometry":
      return (
        <g>
          <rect className="fx-d-quiet" x={x} y={y + 6} width={44} height={32} />
          <rect className="fx-d-crop" x={x + 9} y={y + 13} width={22} height={17} />
          <path className="fx-d-wire" d={`M${x + 31} ${y + 30}L${x + 41} ${y + 40}`} />
          <path className="fx-d-wire" d={`M${x + 35} ${y + 40}H${x + 41}V${y + 34}`} />
        </g>
      );
    case "quantize":
      return (
        <path
          className="fx-d-wire"
          d={`M${x + 2} ${y + 42}V${y + 33}H${x + 11}V${y + 24}H${x + 20}V${y + 16}H${x + 29}V${y + 9}H${x + 38}V${y + 2}H${x + 44}`}
        />
      );
    case "encode":
      return (
        <g>
          <rect className="fx-d-quiet" x={x + 4} y={y} width={36} height={44} />
          {[30, 22, 26, 14, 20].map((length, index) => (
            <path
              key={index}
              className="fx-d-wire"
              d={`M${x + 9} ${y + 9 + index * 7}h${length - 4}`}
            />
          ))}
        </g>
      );
  }
}

const REPORT = ["kernels and threads", "stage timings", "peak managed bytes"];

function Wide() {
  const marker = useMarker();
  const left = 160;
  const width = 132;
  const top = 150;
  const height = 124;
  const right = left + width * STATIONS.length;
  return (
    <>
      <Markers id={marker} />
      {FORMATS.map((format, index) => {
        const y = 128 + index * 40;
        return (
          <g key={format}>
            <Tile x={0} y={y} w={112} h={32} label={format} />
            <Wire points={[[112, y + 16], [136, y + 16], [left - 6, 212]]} />
          </g>
        );
      })}
      <Note x={0} y={112} kind="mark">
        Read
      </Note>

      <path className="fx-d-wire" d={`M${left} 92V84H${right}V92`} />
      <Note x={(left + right) / 2} y={70} anchor="middle">
        One managed memory budget, 2 GiB by default: every engine buffer is charged and reported
      </Note>

      <rect className="fx-d-frame" x={left} y={top} width={right - left} height={height} />
      {STATIONS.map((station, index) => {
        const x = left + index * width;
        const cx = x + width / 2;
        return (
          <g key={station.title}>
            {index > 0 && (
              <path className="fx-d-hair" d={`M${x} ${top + 12}V${top + height - 12}`} />
            )}
            <Note x={cx} y={top + 30} anchor="middle" kind="title">
              {station.title}
            </Note>
            <StationIcon icon={station.icon} cx={cx} cy={top + 78} />
            {station.lines.map((line, row) => (
              <Note key={row} x={cx} y={top + height + 24 + row * 17} anchor="middle">
                {line}
              </Note>
            ))}
          </g>
        );
      })}

      <Wire points={[[right, 212], [836, 212]]} />
      <Wire points={[[836, 212], [836, 176], [856, 176]]} arrow marker={marker} />
      <Wire points={[[836, 212], [836, 266], [856, 266]]} arrow marker={`${marker}-accent`} accent />
      <Tile x={864} y={150} w={176} h={52} label="The image" note="in the format you chose" />
      <rect className="fx-d-accent" x={864} y={226} width={176} height={128} rx={2} />
      <Note x={878} y={252} kind="title">
        The report
      </Note>
      {REPORT.map((line, index) => (
        <Note key={line} x={878} y={276 + index * 18}>
          {line}
        </Note>
      ))}
      <Note x={878} y={276 + REPORT.length * 18} kind="code">
        discarded=exif,xmp
      </Note>
    </>
  );
}

function Narrow() {
  const marker = useMarker();
  const left = 44;
  const right = 352;
  const step = 104;
  const height = 92;
  const first = 136;
  const last = first + (STATIONS.length - 1) * step + height;
  return (
    <>
      <Markers id={marker} />
      {FORMATS.map((format, index) => {
        const x = index % 2 === 0 ? 8 : 184;
        const y = 8 + Math.floor(index / 2) * 46;
        return (
          <g key={format}>
            <Tile x={x} y={y} w={168} h={36} label={format} />
          </g>
        );
      })}
      <Wire points={[[92, 98], [92, 112], [198, 112]]} />
      <Wire points={[[268, 98], [268, 112], [198, 112]]} />
      <Wire points={[[198, 112], [198, first]]} arrow marker={marker} />

      <path className="fx-d-wire" d={`M30 ${first}H22V${last}H30`} />
      <Note x={14} y={(first + last) / 2} anchor="middle" rotate={-90}>
        One memory budget, 2 GiB by default
      </Note>

      {STATIONS.map((station, index) => {
        const y = first + index * step;
        return (
          <g key={station.title}>
            <rect className="fx-d-frame" x={left} y={y} width={right - left} height={height} rx={2} />
            <StationIcon icon={station.icon} cx={left + 40} cy={y + height / 2} />
            <Note x={left + 84} y={y + 32} kind="title">
              {station.title}
            </Note>
            {station.lines.map((line, row) => (
              <Note key={row} x={left + 84} y={y + 54 + row * 17}>
                {line}
              </Note>
            ))}
            {index < STATIONS.length - 1 && (
              <path className="fx-d-wire" d={`M198 ${y + height}V${y + step}`} />
            )}
          </g>
        );
      })}

      <Wire points={[[198, last], [198, last + 16]]} />
      <Wire points={[[198, last + 16], [112, last + 16], [112, last + 34]]} arrow marker={marker} />
      <Wire
        points={[[198, last + 16], [276, last + 16], [276, last + 34]]}
        arrow
        marker={`${marker}-accent`}
        accent
      />
      <Tile x={44} y={last + 40} w={136} h={56} label="The image" note="as you chose" />
      <rect className="fx-d-accent" x={196} y={last + 40} width={156} height={124} rx={2} />
      <Note x={208} y={last + 64} kind="title">
        The report
      </Note>
      {REPORT.map((line, index) => (
        <Note key={line} x={208} y={last + 86 + index * 17}>
          {line}
        </Note>
      ))}
      <Note x={208} y={last + 86 + REPORT.length * 17} kind="code">
        discarded=exif,xmp
      </Note>
    </>
  );
}

const LAST_NARROW = 136 + 4 * 104 + 92;

export function PipelineDrawing() {
  return (
    <Drawing
      label="The pipeline in cross-section. A JPEG, WebP, JPEG XL or AVIF file enters one channel with five stations: decode with the original's decoder, ported to Rust; move into linear light with explicit color; orient, then crop, then resize; quantize at the output depth, 8 to 16 bits; encode as AVIF, WebP or JPEG XL. One managed memory budget spans every station. The image leaves together with a processing report that names kernels, threads, stage timings, the peak of managed memory and what was discarded."
      caption={
        <>
          One pass, whatever the format. The report travels with the image: it names the work
          done, the memory the engine charged to its budget, and every class of information it
          left out (here: EXIF and XMP).
        </>
      }
      wideBox="0 40 1040 324"
      narrowBox={`0 0 360 ${LAST_NARROW + 172}`}
      wide={<Wide />}
      narrow={<Narrow />}
    />
  );
}
