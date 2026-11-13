#!/usr/bin/env python3
"""Record the SIMD features of the benchmark host (decision D2/D6 columns).

Usage: python3 -I cpu_features.py OUT.json

Writes the instruction-set extensions the CPU offers that matter for the
v1.0 SIMD scope (D6: NEON on AArch64, up to AVX2 on x86-64) and the level
above it, so a three-column record can say which SIMD level column (b) ran
at and whether a higher level had to be disabled. Linux reads
/proc/cpuinfo, macOS sysctl hw.optional.*.
"""

import json
import platform
import subprocess
import sys
from pathlib import Path

X86 = ["sse2", "ssse3", "sse4_1", "sse4_2", "avx", "avx2", "fma", "bmi2", "f16c",
       "avx512f", "avx512bw", "avx512vl", "avx512vnni", "avx512vbmi2"]
ARM = ["asimd", "asimddp", "i8mm", "bf16", "sve", "sve2"]
MAC = {"neon": "hw.optional.neon", "asimddp": "hw.optional.arm.FEAT_DotProd",
       "i8mm": "hw.optional.arm.FEAT_I8MM", "bf16": "hw.optional.arm.FEAT_BF16",
       "sve": "hw.optional.arm.FEAT_SVE", "avx2": "hw.optional.avx2_0",
       "avx512f": "hw.optional.avx512f"}


def linux():
    flags = set()
    for line in Path("/proc/cpuinfo").read_text(errors="replace").splitlines():
        key, _, value = line.partition(":")
        if key.strip() in ("flags", "Features"):
            flags.update(value.split())
            break
    return {name: name in flags for name in (X86 if platform.machine() == "x86_64" else ARM)}


def mac():
    out = {}
    for name, key in MAC.items():
        result = subprocess.run(["sysctl", "-n", key], capture_output=True, text=True)
        out[name] = result.stdout.strip() == "1"
    return out


def main():
    machine = platform.machine()
    features = linux() if sys.platform.startswith("linux") else mac()
    x86 = machine in ("x86_64", "AMD64")
    if x86:
        v1 = "AVX2" if features.get("avx2") else "SSE4.1" if features.get("sse4_1") else "SSE2"
        above = [f for f in ("avx512f",) if features.get(f)]
    else:
        v1 = "NEON"
        above = [f for f in ("asimddp", "i8mm", "bf16", "sve", "sve2") if features.get(f)]
    record = {"machine": machine, "features": features,
              "v1_scope_level": v1,
              "above_v1_scope": above,
              "notes": "v1_scope_level: the highest D6 level the CPU offers; "
                       "above_v1_scope: extensions D6 excludes that column (b) "
                       "must not use."}
    Path(sys.argv[1]).write_text(json.dumps(record, indent=2) + "\n")
    print(json.dumps(record))


if __name__ == "__main__":
    main()
