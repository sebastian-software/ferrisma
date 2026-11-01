#!/usr/bin/env python3
"""Record native scalar instructions when comparing floating-point payloads."""

from pathlib import Path
import os
import subprocess

ROOT = Path(__file__).resolve().parent.parent


def run(*args, **kwargs):
    print("+ " + " ".join(map(str, args)), flush=True)
    return subprocess.run(list(map(str, args)), cwd=ROOT, check=True, **kwargs)


def instructions(path, names):
    if not path.is_file():
        print(f"No compiled object: {path}", flush=True)
        return
    symbols = subprocess.check_output(
        ["nm", "--defined-only", str(path)], text=True
    )
    for line in symbols.splitlines():
        fields = line.split()
        if len(fields) < 3 or fields[-2] not in ("t", "T", "w", "W"):
            continue
        symbol = fields[-1]
        if any(name in symbol for name in names):
            run("objdump", "-d", "--disassemble=" + symbol, path)


def main():
    run("rustc", "-vV")
    compiler = os.environ.get("CXX", "clang++")
    run(compiler, "--version")
    cpu = Path("/proc/cpuinfo")
    if cpu.is_file():
        seen = set()
        for line in cpu.read_text().splitlines():
            key = line.split(":", 1)[0].strip()
            if key in ("vendor_id", "model name", "flags") and key not in seen:
                print(line, flush=True)
                seen.add(key)
    probe = ROOT / "target/scalar-diagnostics"
    run(compiler, "-std=c++17", "-O3", "-fno-vectorize", "-fno-slp-vectorize",
        "-stdlib=libc++", "-DHWY_COMPILE_ONLY_SCALAR=1",
        "-Iupstream/libjxl", "-Iupstream/libjxl/third_party/highway",
        "-Iupstream/libjxl/lib/include", "tools/oracle/scalar_diagnostics.cc",
        "-o", probe)
    run(probe, "2147483648", "2141266757", "2143913829", "1046066128",
        "1060575065", "1033100696", "0")
    instructions(probe, ("AddProbe", "MulProbe", "OriginalGamutProbe"))
    for build in ("oracle", "oracle-asan"):
        directory = ROOT / "target" / build
        original = directory / "jxl_oracle"
        if original.is_file():
            run(original, input=(
                "tm-color 4 0 1148846080 0 1132396544 1046066128 1060575065 "
                "1033100696 2147483648 2141266757 2143913829 0\n"
            ), text=True)
            if (ROOT / "src/jxl/render_pipeline/stage_gaborish.rs").is_file():
                request = (
                    "gab-row 1 1038867968 1031462862 1038867968 1031462862 "
                    "1038867968 1031462862 1 2 0 4 2141266757 4289020705 "
                    "2143913829 4291047952 2139095040 4286578688 1056964608 "
                    "1065353216 2147483648\n"
                )
                result = run(original, input=request, text=True, capture_output=True)
                words = result.stdout.split()
                if len(words) != 821 or words[0] != "0":
                    raise RuntimeError("Unexpected original Gaborish observation: " + result.stdout)
                indices = (118, 119, 120, 282, 283, 284, 446, 447, 448)
                print("original native Gaborish words:",
                      " ".join(words[index] for index in indices), flush=True)
                for first, second in (("2141266757", "4289020705"),
                                      ("2143913829", "4291047952")):
                    request = (
                        f"gab-row 1 {first} {second} {second} {first} {first} "
                        f"{second} 1 2 3 3 " + " ".join(["1065353216"] * 9) + "\n"
                    )
                    result = run(original, input=request, text=True, capture_output=True)
                    words = result.stdout.split()
                    if len(words) != 861 or words[0] != "0":
                        raise RuntimeError("Unexpected original normalization observation: " + result.stdout)
                    indices = (*range(121, 127), *range(293, 299), *range(465, 471))
                    print("original native normalization words:",
                          " ".join(words[index] for index in indices), flush=True)
            if (ROOT / "src/jxl/dec_xyb_scalar.rs").is_file():
                rows = (
                    "1132396544 0 0 0",
                    "1132396544 1014350479 1056964608 1061158912",
                    "1132396544 3196059648 3212836864 1073741824",
                    "1132396544 2143363909 4291397477 1061158912",
                    "1065353216 1545563933 1838757507 4286825518",
                )
                for row in rows:
                    result = run(original, input="xyb-helper 0 " + row + "\n",
                                 text=True, capture_output=True)
                    if len(result.stdout.split()) != 3:
                        raise RuntimeError("Unexpected original XYB observation: " + result.stdout)
                    print("original native XYB words:", result.stdout.strip(), flush=True)
        instructions(directory / "CMakeFiles/jxl_oracle.dir/sample_conversion.cc.o",
                     ("GamutMap",))
    for profile in ("debug", "release"):
        for candidate in sorted((ROOT / "target" / profile / "deps").glob("parity-*")):
            if candidate.is_file() and candidate.open("rb").read(4) == b"\x7fELF":
                instructions(candidate, ("add_gamut", "gamut_map"))


if __name__ == "__main__":
    main()
