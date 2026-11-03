#!/usr/bin/env python3
"""Live object/address and actual compile-option proof for decoder observers.

This does not alter or rebuild a graph. The frozen state verifier owns common
strict flag and archive-map checks. The private helper observer additionally
needs GNU text-section attribution to its directly linked pristine-include TU.
"""
import json
from pathlib import Path
import re
import shlex
import subprocess

from brotli_state_reference import PIN, ROOT, compile_options, live_map_bindings, sha


def direct_gnu_bindings(mapped):
    """Supplement archive attribution with live, directly linked .o sections."""
    assert "Linker script and memory map\n" in mapped
    found, section, owner = {}, None, None
    for line in mapped.split("Linker script and memory map\n", 1)[1].splitlines():
        start = re.match(r"^ (\.\S+)(?:\s+(.*))?$", line)
        if start:
            section, owner = start[1], None
        extent = re.match(r"^\s+(?:\.\S+\s+)?(0x[0-9a-fA-F]+)\s+(0x[0-9a-fA-F]+)\s+(\S+\.o)\s*$", line)
        if extent and section and section.startswith(".text"):
            owner = (int(extent[1], 16), int(extent[2], 16), extent[3])
        # An archive input is handled by the frozen parser, never attributed
        # to an earlier directly linked object in the same output section.
        if re.match(r"^\s+\.\S+\s+0x", line) and not extent:
            owner = None
        symbol = re.match(r"^\s+(0x[0-9a-fA-F]+)\s+(Brotli\w+)\s*$", line)
        if symbol and owner:
            address = int(symbol[1], 16)
            assert owner[0] <= address < owner[0] + owner[1], (symbol[2], owner)
            assert symbol[2] not in found, symbol[2]
            found[symbol[2]] = {"address": address, "object": owner[2]}
        if line and not line[0].isspace():
            section, owner = None, None
    return found


def requested_bindings(mapped, expected):
    if "# Object files:" not in mapped:
        live = live_map_bindings(mapped)
        direct = direct_gnu_bindings(mapped)
        assert not set(live).intersection(direct)
        live.update(direct)
        return live
    # Encoder maps can also name unrequested zero-size aliases. Resolve the
    # requested concrete functions from their own live rows; no other symbol
    # can establish an observed function's owner or address.
    objects_text = mapped.split("# Object files:\n", 1)[1].split("# Sections:", 1)[0]
    objects = dict(re.findall(r"^\[\s*(\d+)\]\s+(.+)$", objects_text, re.M))
    rows = mapped.split("# Symbols:\n", 1)[1].split("# Dead Stripped Symbols:", 1)[0]
    live = {}
    for address, size, index, symbol in re.findall(
            r"^(0x[0-9A-Fa-f]+)\s+(0x[0-9A-Fa-f]+)\s+\[\s*(\d+)\]\s+_?(Brotli\w+)\s*$", rows, re.M):
        if symbol in expected:
            assert symbol not in live and int(size, 16) > 0, symbol
            live[symbol] = {"address": int(address, 16), "object": objects[index]}
    return live


def verify(binary, sanitized, map_name, observers, expected):
    commands_path = binary.parent / "compile_commands.json"
    commands = json.loads(commands_path.read_text())
    decoded = [c for c in commands if c["file"].endswith("/c/dec/decode.c")]
    assert len(decoded) == 1
    source = Path(decoded[0]["file"]).resolve().parents[2]
    git = lambda *a: subprocess.check_output(["git", "-C", source, *a], text=True).strip()
    assert git("rev-parse", "HEAD") == PIN
    assert not git("status", "--porcelain", "--untracked-files=no")
    units = [c for c in commands if Path(c["file"]).resolve().is_relative_to(source)]
    assert len(units) == 35, len(units)
    adapters = []
    for filename in observers:
        matches = [c for c in commands if Path(c["file"]).resolve() == ROOT / "tools/oracle" / filename
                   and any(f"CMakeFiles/{binary.name}.dir/" in f for f in
                           (c.get("arguments") or shlex.split(c["command"])))]
        assert len(matches) == 1, (filename, len(matches))
        adapters.extend(matches)
    for command in [*units, *adapters]:
        compile_options(command, sanitized)
    map_path = binary.parent / map_name
    mapped = map_path.read_text()
    live = requested_bindings(mapped, expected)
    symbols = subprocess.check_output(["nm", str(binary)], text=True)
    bindings = {}
    for symbol, expected_object in expected.items():
        addresses = re.findall(rf"^([0-9a-fA-F]+) [Tt] _?{symbol}$", symbols, re.M)
        assert len(addresses) == 1, symbol
        actual = live[symbol]
        assert Path(actual["object"]).name == expected_object, (symbol, actual, expected_object)
        assert actual["address"] == int(addresses[0], 16), (symbol, actual, addresses)
        bindings[symbol] = actual
    return {"configured_pristine_c_units": len(units), "checked_observer_units": len(adapters),
            "actual_link_map_sha256": sha(map_path), "live_symbol_bindings": bindings,
            "binary_sha256": sha(binary), "compile_commands_sha256": sha(commands_path)}
