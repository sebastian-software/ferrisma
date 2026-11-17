/*
 * The drawing kit of the homepage: engraved hairlines on Monografie's sheet.
 * Every color is a `--mono-` token (drawings.css); the one accent is oxide,
 * kept for the element a drawing is about. Each drawing ships in two
 * layouts, wide (from 1000px) and narrow, so labels keep a readable size
 * instead of shrinking with the viewBox; CSS shows one of them.
 */
import { type ReactNode, useId } from "ardo/react";

export type Kind = "frame" | "quiet" | "ghost" | "accent";

type BoxProps = {
  x: number;
  y: number;
  w: number;
  h: number;
  title?: string;
  lines?: string[];
  kind?: Kind;
  /** Center the text instead of setting it flush left. */
  center?: boolean;
  /** Title in the serif, larger; otherwise a sans label. */
  serif?: boolean;
  /** Distance of the first baseline from the top. */
  top?: number;
};

/** A labeled rectangle: a part, a stage, a build. */
export function Box({
  x,
  y,
  w,
  h,
  title,
  lines = [],
  kind = "frame",
  center = false,
  serif = false,
  top,
}: BoxProps) {
  const tx = center ? x + w / 2 : x + 14;
  const anchor = center ? "middle" : "start";
  const first = top ?? (lines.length === 0 ? h / 2 + 5 : serif ? 27 : 24);
  return (
    <g>
      <rect className={`fx-d-${kind}`} x={x} y={y} width={w} height={h} rx={2} />
      {title !== undefined && (
        <text
          className={serif ? "fx-d-title" : "fx-d-label"}
          x={tx}
          y={y + first}
          textAnchor={anchor}
        >
          {title}
        </text>
      )}
      {lines.map((line, index) => (
        <text
          key={index}
          className="fx-d-note"
          x={tx}
          y={y + first + (serif ? 22 : 19) + index * 17}
          textAnchor={anchor}
        >
          {line}
        </text>
      ))}
    </g>
  );
}

/** A file: a sheet with a folded corner. */
export function Tile({
  x,
  y,
  w,
  h,
  label,
  note,
  kind = "frame",
}: {
  x: number;
  y: number;
  w: number;
  h: number;
  label: string;
  note?: string;
  kind?: Kind;
}) {
  const f = Math.min(12, h / 3);
  return (
    <g>
      <path
        className={`fx-d-${kind}`}
        d={`M${x} ${y}H${x + w - f}L${x + w} ${y + f}V${y + h}H${x}Z`}
      />
      <path className="fx-d-line" d={`M${x + w - f} ${y}V${y + f}H${x + w}`} />
      <text
        className="fx-d-label"
        x={x + 12}
        y={note === undefined ? y + h / 2 + 5 : y + h / 2 - 3}
      >
        {label}
      </text>
      {note !== undefined && (
        <text className="fx-d-note" x={x + 12} y={y + h / 2 + 15}>
          {note}
        </text>
      )}
    </g>
  );
}

/** A connecting line through points; `arrow` ends it with a small head. */
export function Wire({
  points,
  arrow = false,
  marker,
  accent = false,
  dashed = false,
}: {
  points: [number, number][];
  arrow?: boolean;
  marker?: string;
  accent?: boolean;
  dashed?: boolean;
}) {
  const d = points.map(([px, py], index) => `${index === 0 ? "M" : "L"}${px} ${py}`).join("");
  const classes = ["fx-d-wire", accent ? "fx-d-wire-accent" : "", dashed ? "fx-d-dashed" : ""]
    .filter(Boolean)
    .join(" ");
  return (
    <path
      className={classes}
      d={d}
      markerEnd={arrow && marker !== undefined ? `url(#${marker})` : undefined}
    />
  );
}

/** Small text set on its own. */
export function Note({
  x,
  y,
  children,
  anchor = "start",
  kind = "note",
  rotate,
}: {
  x: number;
  y: number;
  children: ReactNode;
  anchor?: "start" | "middle" | "end";
  kind?: "note" | "label" | "title" | "code" | "mark";
  rotate?: number;
}) {
  return (
    <text
      className={`fx-d-${kind}`}
      x={x}
      y={y}
      textAnchor={anchor}
      transform={rotate === undefined ? undefined : `rotate(${rotate} ${x} ${y})`}
    >
      {children}
    </text>
  );
}

/** The arrowheads a drawing uses, one per tone. */
export function Markers({ id }: { id: string }) {
  return (
    <defs>
      <marker
        id={id}
        viewBox="0 0 10 10"
        refX="9"
        refY="5"
        markerWidth="7"
        markerHeight="7"
        orient="auto-start-reverse"
      >
        <path className="fx-d-head" d="M0 1L9 5L0 9Z" />
      </marker>
      <marker
        id={`${id}-accent`}
        viewBox="0 0 10 10"
        refX="9"
        refY="5"
        markerWidth="7"
        markerHeight="7"
        orient="auto-start-reverse"
      >
        <path className="fx-d-head-accent" d="M0 1L9 5L0 9Z" />
      </marker>
    </defs>
  );
}

/** One arrowhead id per drawing and layout, unique on the page. */
export function useMarker(): string {
  return `fx-arrow-${useId().replaceAll(":", "")}`;
}

/**
 * A drawing on a sheet: the figure, its caption, and the two layouts. Both
 * layouts carry the same accessible name; the hidden one is `display: none`
 * and so out of the accessibility tree.
 */
export function Drawing({
  caption,
  label,
  wide,
  narrow,
  wideBox,
  narrowBox,
  className,
}: {
  caption: ReactNode;
  label: string;
  wide: ReactNode;
  narrow: ReactNode;
  wideBox: string;
  narrowBox: string;
  className?: string;
}) {
  const id = useId();
  return (
    <figure className={className === undefined ? "fx-drawing" : `fx-drawing ${className}`}>
      <svg
        className="fx-d-wide"
        viewBox={wideBox}
        role="img"
        aria-label={label}
        aria-describedby={id}
      >
        {wide}
      </svg>
      <svg
        className="fx-d-narrow"
        viewBox={narrowBox}
        role="img"
        aria-label={label}
        aria-describedby={id}
      >
        {narrow}
      </svg>
      <figcaption id={id}>{caption}</figcaption>
    </figure>
  );
}
