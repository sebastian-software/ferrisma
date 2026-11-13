"""Rewrite the NEON islow pass-2 helpers of jidctint_neon.rs as macro_rules!
(C's `static INLINE`, which upstream defines as always_inline).
Usage: python3 -I to_macro.py <in> <out>"""
import re
import sys

src = open(sys.argv[1]).read()

SIG = {
    "jsimd_idct_islow_pass2_regular": None,
    "jsimd_idct_islow_pass2_sparse": None,
}
ATTRS_RE = (r"#\[expect\(clippy::similar_names, reason = \"C variable names\"\)\]\n"
            r"#\[expect\(\n    clippy::identity_op,\n    clippy::erasing_op,\n"
            r"    reason = \"C writes k \* DCTSIZE / 2 and buf_offset \+ k offsets with k = 0 and 1\"\n\)\]\n"
            r"#\[expect\(\n    clippy::cast_possible_truncation,\n"
            r"    reason = \"C's conversion of CENTERJSAMPLE to uint8_t\"\n\)\]\n"
            r"#\[target_feature\(enable = \"neon\"\)\]\n#\[inline\]\n")
for name in SIG:
    head = re.compile(
        r"(// upstream: simd/arm/jidctint-neon.c " + name + r"\n)" + ATTRS_RE +
        r"fn " + name + r"\(\n    workspace: &\[i16\],\n    mut output_buf: JSAMPROWS_MUT<'_, '_>,\n"
        r"    output_col: JDIMENSION,\n    buf_offset: u32,\n\) \{\n")
    m = head.search(src)
    assert m, name
    # The body ends at the first line that is exactly "}" after the head.
    end = src.index("\n}\n", m.end())
    body = src[m.end():end + 1]
    body = "".join("    " + line if line.strip() else line
                   for line in body.splitlines(keepends=True))
    macro = (
        m[1] +
        "// C defines the helper `static INLINE`, which upstream's build makes\n"
        "// `__attribute__((always_inline))`. Rust rejects `#[inline(always)]` on a\n"
        "// `#[target_feature]` function, and LLVM keeps the `#[inline]` function out\n"
        "// of line, so the helper is a macro that expands in its caller's Neon\n"
        "// context, with the C parameters as `let` bindings (ADR 0004, 2026-11-13\n"
        "// amendment).\n"
        "macro_rules! " + name + " {\n"
        "    ($workspace:expr, $output_buf:expr, $output_col:expr, $buf_offset:expr) => {{\n"
        "        let workspace: &[i16] = $workspace;\n"
        "        let mut output_buf: JSAMPROWS_MUT<'_, '_> = $output_buf;\n"
        "        let output_col: JDIMENSION = $output_col;\n"
        "        let buf_offset: u32 = $buf_offset;\n"
        "\n" + body +
        "    }};\n"
        "}\n"
        "use " + name + ";\n")
    src = src[:m.start()] + macro + src[end + 3:]
    # Calls become macro invocations.
    src = re.sub(r"(\s)" + name + r"\(&", r"\1" + name + "!(&", src)
open(sys.argv[2], "w").write(src)
