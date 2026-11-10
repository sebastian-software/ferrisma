#!/usr/bin/env python3
"""Print the Rust name of C identifiers under the naming rule of ADR 0001.

Usage: python3 tools/rust_name.py [--constant] NAME...

Without --constant, prints the function name (snake case): VP8GetHeaders ->
vp8_get_headers. With --constant, prints the name of a constant, table or
global: kZigzag -> ZIGZAG. Type names, enum values and upper-snake constants
keep their C names and need no conversion.
"""

import argparse
import keyword
import re

RUST_KEYWORDS = {
    "as", "async", "await", "break", "const", "continue", "crate", "dyn", "else", "enum",
    "extern", "false", "fn", "for", "gen", "if", "impl", "in", "let", "loop", "match", "mod",
    "move", "mut", "pub", "ref", "return", "self", "static", "struct", "super", "trait", "true",
    "type", "unsafe", "use", "where", "while", "abstract", "become", "box", "do", "final",
    "macro", "override", "priv", "try", "typeof", "unsized", "virtual", "yield",
}


def words(name):
    """Split a C identifier into words as ADR 0001 defines."""
    name = name.replace("WebP", "Webp")
    result = []
    for part in name.split("_"):
        current = ""
        for index, char in enumerate(part):
            previous = part[index - 1] if index else ""
            following = part[index + 1] if index + 1 < len(part) else ""
            boundary = char.isupper() and current and (
                previous.islower()
                or ((previous.isupper() or previous.isdigit()) and following.islower()))
            if boundary:
                result.append(current)
                current = ""
            current += char
        if current:
            result.append(current)
    return [word for word in result if word]


def function_name(name):
    snake = "_".join(word.lower() for word in words(name))
    return f"r#{snake}" if snake in RUST_KEYWORDS else snake


def constant_name(name):
    if re.match(r"k[A-Z0-9]", name):
        name = name[1:]
    return "_".join(word.upper() for word in words(name))


def main():
    parser = argparse.ArgumentParser(description=__doc__,
                                     formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("--constant", action="store_true", help="convert constants and tables")
    parser.add_argument("names", nargs="+")
    args = parser.parse_args()
    for name in args.names:
        print(f"{name} -> {constant_name(name) if args.constant else function_name(name)}")


EXAMPLES = {
    "VP8GetHeaders": "vp8_get_headers",
    "VP8LDecodeImage": "vp8l_decode_image",
    "WebPSafeMalloc": "webp_safe_malloc",
    "WebPIDecGetRGB": "webp_i_dec_get_rgb",
    "VP8GetCPUInfo": "vp8_get_cpu_info",
    "TransformWHT_C": "transform_wht_c",
    "GetLE16": "get_le16",
    "SharpYuvConvert": "sharp_yuv_convert",
    "VP8LColorCacheInit": "vp8l_color_cache_init",
    "WebPRescalerImportRow": "webp_rescaler_import_row",
    "VP8InitBitReader": "vp8_init_bit_reader",
    "load_bits": "load_bits",
    "Type": "r#type",
}
CONSTANT_EXAMPLES = {"kZigzag": "ZIGZAG", "kYModesIntra4": "Y_MODES_INTRA4", "kBands": "BANDS"}


def self_test():
    for name, expected in EXAMPLES.items():
        assert function_name(name) == expected, (name, function_name(name), expected)
    for name, expected in CONSTANT_EXAMPLES.items():
        assert constant_name(name) == expected, (name, constant_name(name), expected)


self_test()
assert not keyword.iskeyword("vp8")

if __name__ == "__main__":
    main()
