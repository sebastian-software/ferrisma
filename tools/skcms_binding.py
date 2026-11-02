#!/usr/bin/env python3
"""Resolve actual executable code addresses to live source object ranges."""
from pathlib import Path
import platform
import re
import subprocess


def live_bindings(binary, mapping, expected):
    """expected maps a logical name to (exact nm-name regex, owner regex).

    Mach-O uses only the live Symbols region, never dead symbols or FDEs.
    GNU local functions may have no individual map row; their exact nm code
    address must belong to the expected input object's actual .text extent.
    """
    mapped = Path(mapping).read_text()
    symbols = subprocess.check_output(["nm", str(binary)], text=True)
    code = [(int(address, 16), symbol) for address, symbol in
            re.findall(r"^([0-9a-fA-F]+) [Tt] (\S+)\s*$", symbols, re.M)]
    if platform.system() == "Darwin":
        objects_region = mapped.split("# Object files:\n", 1)[1].split("# Sections:", 1)[0]
        objects = dict(re.findall(r"^\[\s*(\d+)\]\s+(.+)$", objects_region, re.M))
        live = mapped.split("# Symbols:\n", 1)[1].split("# Dead Stripped Symbols:", 1)[0]
        rows = [(int(address, 16), int(size, 16), objects[index], symbol) for
                address, size, index, symbol in re.findall(
                    r"^(0x[0-9a-fA-F]+)\s+(0x[0-9a-fA-F]+)\s+\[\s*(\d+)\]\s+(\S+)\s*$", live, re.M)]
    else:
        region = mapped.split("Linker script and memory map\n", 1)[1]
        rows, section = [], None
        for line in region.splitlines():
            start = re.match(r"^ (\.\S+)(?:\s+.*)?$", line)
            if start:
                section = start[1]
            extent = re.match(r"^\s+(?:\.\S+\s+)?(0x[0-9a-fA-F]+)\s+(0x[0-9a-fA-F]+)\s+(\S+(?:\.o|\.a\([^()]+\)))\s*$", line)
            if extent and section and section.startswith(".text"):
                rows.append((int(extent[1], 16), int(extent[2], 16), extent[3], None))
            if line and not line[0].isspace():
                section = None
    result = {}
    for logical, (symbol_pattern, owner_pattern) in expected.items():
        selected = [(address, symbol) for address, symbol in code if re.fullmatch(symbol_pattern, symbol)]
        if len(selected) != 1:
            raise RuntimeError(f"Expected one actual code symbol for {logical}: {selected}")
        address, symbol = selected[0]
        owners = [(base, size, owner) for base, size, owner, mapped_symbol in rows
                  if size > 0 and base <= address < base + size
                  and (mapped_symbol == symbol if platform.system() == "Darwin" else True)]
        if len(owners) != 1 or not re.search(owner_pattern, owners[0][2]):
            raise RuntimeError(f"Live source ownership differs for {logical}: {owners}")
        result[logical] = {"symbol": symbol, "address": address, "owner": owners[0][2],
                           "input_text_start": owners[0][0], "input_text_size": owners[0][1]}
    return result
