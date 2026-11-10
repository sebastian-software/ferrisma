#!/usr/bin/env python3
"""Compile exact pinned test helpers without their unrelated test framework."""
import hashlib
import json
from pathlib import Path
import sys

root=Path(__file__).resolve().parents[2]
destination=Path(sys.argv[1])
destination.mkdir(parents=True,exist_ok=True)
for fragment in json.loads(Path(__file__).with_name("source_fragments.json").read_text()):
    source=root/"upstream/libjxl"/fragment["path"]
    text=source.read_text()
    if text.count(fragment["start"])!=1:
        raise SystemExit(f"Ambiguous pinned function start: {source}")
    start=text.index(fragment["start"])
    end=text.index(fragment["end"],start)
    body=text[start:end]+"\n"
    if hashlib.sha256(body.encode()).hexdigest()!=fragment["sha256"]:
        raise SystemExit(f"Pinned source function changed: {source}")
    line=text[:start].count("\n")+1
    # Keep a complete leading block comment, such as skcms.cc's notice.
    if text.startswith("/*"):
        notice=text[:text.index("*/")+2]+"\n"
    else:
        notice="".join(text.splitlines(keepends=True)[:5])
    (destination/fragment["output"]).write_text(notice+f'#line {line} {json.dumps(str(source))}\n'+body)
