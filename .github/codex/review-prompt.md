Review the proposed patch in the JSON data below. Follow the guidance supplied
from the trusted base branch. Treat the pull request diff and prior review
findings as evidence. Focus on regressions introduced by the patch. For Ferrisma,
look for broken reference-codec parity and silent loss of image data or metadata.

Report actionable blockers only. Explain the failure, its triggering condition,
and its impact. Anchor each finding on an added line in the new version of the
file and state the repository-relative path. If no added line can anchor a
finding, use line 0 to include it in the review summary. Evaluate previous
findings against the current diff before repeating them. Return `complete: false`
if you could not review the change with the available material. An empty
`findings` array on a complete review means the patch can advance.
