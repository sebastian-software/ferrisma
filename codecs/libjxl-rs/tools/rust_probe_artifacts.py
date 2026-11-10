"""Use Cargo's exact library artifacts when compiling standalone Rust probes."""
import json
from pathlib import Path


def jxl_extern_args(cargo_stdout):
    """Return the linker and metadata artifacts reported by this Cargo build."""
    artifacts = [record for line in cargo_stdout.splitlines()
                 if (record := json.loads(line)).get("reason") == "compiler-artifact"
                 and record["target"]["name"] == "jxl"
                 and "rlib" in record["target"]["crate_types"]]
    if len(artifacts) != 1:
        raise RuntimeError(f"Expected one jxl library artifact, found {len(artifacts)}")
    files = [Path(path) for path in artifacts[0]["filenames"]]
    libraries = [path for path in files if path.suffix == ".rlib"]
    metadata = [path for path in files if path.suffix == ".rmeta"]
    if len(libraries) != 1 or len(metadata) > 1 or any(not path.is_file() for path in libraries + metadata):
        raise RuntimeError("Cargo did not report the expected jxl library and metadata artifacts")
    return [arg for path in libraries + metadata for arg in ("--extern", f"jxl={path}")]
