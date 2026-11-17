/*
 * Refuse or report, as a decision: the three outcomes a part of the input can
 * have, and no fourth. Source: docs/adr/0042-no-silent-loss-refuse-or-report.md
 * (D1 refuse what would make the picture wrong, D2 report what is left out of
 * a correct picture) and docs/adr/0043-formats-behave-alike.md (D1, the same
 * outcome class in every format).
 */
import { Box, Drawing, Markers, Note, Wire, useMarker } from "./parts";

function Wide() {
  const marker = useMarker();
  const accent = `${marker}-accent`;
  return (
    <>
      <Markers id={marker} />
      <Box x={0} y={112} w={190} h={72} title="A part of the input" lines={["color, alpha, metadata…"]} />
      <Wire points={[[190, 148], [226, 148]]} arrow marker={marker} />
      <Box x={230} y={112} w={230} h={72} kind="quiet" title="Can the pipeline" lines={["carry it into the output?"]} />

      <Wire points={[[345, 112], [345, 40], [536, 40]]} arrow marker={marker} />
      <Note x={356} y={92}>
        yes
      </Note>
      <Box x={540} y={6} w={240} h={68} title="Carried" lines={["into the output, converted"]} serif />

      <Wire points={[[460, 148], [536, 148]]} arrow marker={marker} />
      <Note x={474} y={140}>
        no
      </Note>
      <Box x={540} y={112} w={240} h={72} kind="quiet" title="Would the picture" lines={["be wrong without it?"]} />

      <Wire points={[[780, 130], [800, 130], [800, 60], [826, 60]]} arrow marker={marker} />
      <Note x={808} y={116}>
        yes
      </Note>
      <Box x={830} y={22} w={210} h={76} title="Refused" lines={["an error with its reason"]} serif />

      <Wire points={[[780, 166], [800, 166], [800, 236], [826, 236]]} arrow marker={accent} accent />
      <Note x={808} y={190}>
        no
      </Note>
      <Box
        x={830}
        y={198}
        w={210}
        h={76}
        kind="accent"
        title="Left out, and named"
        lines={["in the report"]}
        serif
      />
      <Note x={0} y={250} kind="mark">
        There is no fourth outcome.
      </Note>
      <Note x={0} y={272}>
        The same property gets the same outcome in every format.
      </Note>
    </>
  );
}

function Narrow() {
  const marker = useMarker();
  const accent = `${marker}-accent`;
  return (
    <>
      <Markers id={marker} />
      <Box x={60} y={0} w={240} h={64} title="A part of the input" lines={["color, alpha, metadata…"]} center />
      <Wire points={[[180, 64], [180, 88]]} arrow marker={marker} />
      <Box x={60} y={92} w={240} h={64} kind="quiet" title="Can the pipeline carry it" lines={["into the output?"]} center />
      <Wire points={[[300, 124], [328, 124], [328, 186]]} arrow marker={marker} />
      <Note x={316} y={116}>
        yes
      </Note>
      <Wire points={[[180, 156], [180, 266]]} arrow marker={marker} />
      <Note x={190} y={180}>
        no
      </Note>
      <Box x={196} y={190} w={164} h={64} title="Carried" lines={["converted if needed"]} serif center />
      <Box x={60} y={270} w={240} h={64} kind="quiet" title="Would the picture be" lines={["wrong without it?"]} center />
      <Wire points={[[120, 334], [120, 370]]} arrow marker={marker} />
      <Note x={128} y={356}>
        yes
      </Note>
      <Wire points={[[240, 334], [240, 370]]} arrow marker={accent} accent />
      <Note x={248} y={356}>
        no
      </Note>
      <Box x={0} y={374} w={170} h={76} title="Refused" lines={["an error with", "its reason"]} serif />
      <Box x={190} y={374} w={170} h={76} kind="accent" title="Left out, named" lines={["in the report"]} serif />
      <Note x={0} y={480} kind="mark">
        There is no fourth outcome.
      </Note>
      <Note x={0} y={500}>
        Same property, same outcome, in every format.
      </Note>
    </>
  );
}

export function RefuseOrReportDrawing() {
  return (
    <Drawing
      className="fx-drawing-compact"
      label="Refuse or report. For each part of the input, such as color, alpha or metadata: if the pipeline can carry it into the output, it does, converted if needed. If not, and the picture would be wrong without it, the input is refused with an error and its reason. If the picture stays correct without it, the part is left out and named in the report. There is no fourth outcome, and the same property gets the same outcome in every format."
      caption={
        <>
          A caller can always tell what happened to their image: it was carried, it was refused
          with a reason, or the report says what was left out.
        </>
      }
      wideBox="0 0 1040 284"
      narrowBox="0 0 360 512"
      wide={<Wide />}
      narrow={<Narrow />}
    />
  );
}
