"""The crate's declared minimum Rust version, read from Cargo.toml (`rust-version`).

The audit tools build their "msrv" configuration with this toolchain and write
it into the `clippy.toml` they generate, so a change of the declaration needs
no edit here (the same source `check.py` uses for `--msrv`).
"""
from pathlib import Path
import re


def rust_version():
    text = (Path(__file__).resolve().parent.parent / "Cargo.toml").read_text()
    return re.search(r'^rust-version = "([^"]+)"$', text, re.M)[1]


MSRV = rust_version()
