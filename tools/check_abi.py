#!/usr/bin/env python3
"""Compare named translated record sizes, alignment, and field offsets with Clang."""
import argparse
import json
import os
from pathlib import Path
import re
import shlex
import subprocess
import platform

ROOT = Path(__file__).resolve().parents[1]


def walk(node):
    yield node
    for child in node.get("inner", []):
        yield from walk(child)


def compiler_flags(entry):
    flags = shlex.split(entry["command"])
    result = [flags[0]]
    index = 1
    while index < len(flags):
        flag = flags[index]
        if flag in ("-o", "-c"):
            index += 2 if flag == "-o" else 1
            continue
        if flag != entry["file"]:
            result.append(flag)
        index += 1
    return result


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--build", type=Path, default=ROOT / "build/baseline")
    parser.add_argument("--features", default="", help="Comma-separated Cargo features matching the C reference configuration")
    parser.add_argument('--root-profile', type=Path, help='Native generated profile used by the isolated consumer build')
    parser.add_argument('--production-profile', type=Path,
                        help='Probe the production package, which compiles this generated native profile')
    args = parser.parse_args()
    entries = [entry for entry in json.loads((args.build / "compile_commands.json").read_text())
               if "avif_obj.dir" in entry["command"]]
    manifest = {unit["source"]: unit for unit in json.loads((ROOT / "TRANSLATION.json").read_text())["units"]}
    features = [feature for feature in args.features.split(",") if feature]
    rust_package, rust_root, rust_profile = 'libavif-rs', ROOT, 'release'
    profile = "-".join(feature for feature in ["mini", "extended-pixi", "avm"] if feature in features)
    if profile:
        manifest.update({unit["source"]: unit for unit in json.loads((ROOT / "translations" / f"{profile}.json").read_text())["units"]})
    if any(feature in features for feature in ["libgav1", "rav1e", "svt", "libyuv"]):
        for unit in json.loads((ROOT / "translations/optional-av1.json").read_text())["units"]:
            if Path(unit["source"]).stem.startswith("codec_") or "libyuv" in features:
                manifest[unit["source"]] = unit
    work = ROOT / "build/abi"
    if args.root_profile and args.production_profile:
        parser.error('Probe either an isolated native profile or the production package')
    production_target = None
    if args.production_profile:
        from check_root_profile import validate_profile
        from root_production import cargo_target, native_target
        target = native_target()
        generated = validate_profile(args.production_profile.resolve(), target)
        if generated['variant'] != ('-'.join(features) or 'default'):
            parser.error('ABI probe features must match the production native profile')
        manifest = {unit['source']: unit for unit in generated['units']}
        # The probe consumes the production package; on Linux its Cargo
        # profile selects the matching Debug or Release native translation.
        rust_profile = generated['preprocessing_profile']
        work = ROOT / f'build/abi-production-{target}-{rust_profile}-{generated["variant"]}'
        production_target = cargo_target(features)
        os.environ.setdefault('LIBAVIF_RS_NATIVE_DEPENDENCY_ROOT', str(ROOT))
    if args.root_profile:
        from check_root_profile import validate_profile, profile_work, profile_target
        target = ('aarch64-apple-darwin' if platform.system() == 'Darwin' and platform.machine() == 'arm64'
                  else 'x86_64-unknown-linux-gnu' if platform.system() == 'Linux' and platform.machine() == 'x86_64'
                  else None)
        generated = validate_profile(args.root_profile.resolve(), target)
        if generated['variant'] != ('-'.join(features) or 'default'):
            parser.error('ABI probe features must match the generated native root profile')
        manifest = {unit['source']: unit for unit in generated['units']}
        rust_package, rust_root = 'libavif-native-root-profile', profile_work(generated)
        if not (rust_root / 'Cargo.toml').is_file():
            parser.error('Build the isolated native root profile before probing its ABI')
        rust_profile = generated['preprocessing_profile']
        work = ROOT / f'build/abi-{target}-{rust_profile}-{generated["variant"]}'
    work.mkdir(parents=True, exist_ok=True)
    expected, rust = [], ["fn main() {"]
    checked_records, skipped = 0, []
    for index, entry in enumerate(entries):
        source = Path(entry["file"])
        unit = manifest[str(source.relative_to(ROOT))]
        translation = (ROOT / unit["rust"]).read_text()
        records = re.findall(r"pub (struct|union) (\w+) \{(.*?)\n\}", translation, re.S)
        if not records:
            continue
        flags = compiler_flags(entry)
        ast = json.loads(subprocess.check_output(
            flags + ["-Xclang", "-ast-dump=json", "-fsyntax-only", str(source)],
            cwd=entry["directory"], stderr=subprocess.PIPE,
        ))
        nodes = list(walk(ast))
        tags = {node["name"]: node for node in nodes
                if node.get("kind") == "RecordDecl" and node.get("completeDefinition") and node.get("name")}
        aliases = {node["name"]: node for node in nodes if node.get("kind") == "TypedefDecl" and node.get("name")}
        by_id = {node["id"]: node for node in nodes if "id" in node and node.get("completeDefinition")}
        c_values = []
        for kind, name, body in records:
            if name.startswith("C2RustUnnamed"):
                skipped.append(f"{unit['rust']}::{name}: anonymous C record")
                continue
            record = tags.get(name)
            c_type = f"{kind} {name}"
            if name in aliases:
                alias = aliases[name]
                if alias.get("type", {}).get("qualType", "").startswith(("struct ", "union ")):
                    c_type = name
                    if record is None:
                        for node in walk(alias):
                            reference = node.get("decl", {}).get("id") or node.get("ownedTagDecl", {}).get("id")
                            if reference in by_id:
                                record = by_id[reference]
                                break
            if record is None and name == '__va_list_tag':
                # Clang exposes the builtin array's real element layout through
                # typeof even though the implicit tag has no source declaration.
                c_type = '__typeof__(((__builtin_va_list){0})[0])'
                record = {'inner': [{'kind': 'FieldDecl', 'name': field} for field in
                                    re.findall(r'^\s*pub (\w+):', body, re.M)]}
            if record is None:
                raise SystemExit(f"Cannot resolve original C record: {unit['source']}::{name}")
            c_fields = [node for node in record.get("inner", []) if node.get("kind") == "FieldDecl"]
            rust_fields = re.findall(r"^\s*pub (\w+):", body, re.M)
            if len(c_fields) != len(rust_fields) or any(field.get("isBitfield") or not field.get("name") for field in c_fields):
                skipped.append(f"{unit['rust']}::{name}: anonymous fields or bitfields")
                continue
            module = "avif::" + ("libyuv::" if '/third_party/libyuv/' in unit['source'] else "") + Path(unit["rust"]).stem
            pairs = [(f"sizeof({c_type})", f"core::mem::size_of::<{module}::{name}>()", "size"),
                     (f"_Alignof({c_type})", f"core::mem::align_of::<{module}::{name}>()", "alignment")]
            pairs += [(f"__builtin_offsetof({c_type}, {c_field['name']})",
                       f"core::mem::offset_of!({module}::{name}, {rust_field})", c_field["name"])
                      for c_field, rust_field in zip(c_fields, rust_fields)]
            for c_expression, rust_expression, label in pairs:
                c_values.append(c_expression)
                rust.append(f'    println!("{{}}", {rust_expression});')
                expected.append((f"{module}::{name}.{label}", None))
            checked_records += 1
        if c_values:
            probe = work / f"probe-{index}.c"
            ir = work / f"probe-{index}.ll"
            probe.write_text(f'#include "{source}"\nconst unsigned long long libavif_rs_abi_values[] = {{\n'
                             + ",\n".join(c_values) + "\n};\n")
            subprocess.run(flags + ["-S", "-emit-llvm", "-o", str(ir), str(probe)],
                           cwd=entry["directory"], check=True, stderr=subprocess.PIPE)
            line = next(line for line in ir.read_text().splitlines() if line.startswith("@libavif_rs_abi_values ="))
            values = [int(value) for value in re.findall(r"i64 (\d+)", line)]
            if len(values) != len(c_values):
                raise SystemExit(f"Unexpected Clang layout probe output for {source}")
            start = len(expected) - len(values)
            for offset, value in enumerate(values):
                label, _ = expected[start + offset]
                expected[start + offset] = (label, value)
    rust.append("}")
    probe = work / "probe.rs"
    probe.write_text("\n".join(rust) + "\n")
    # Use Cargo rather than assuming a particular compiler metadata/deps layout.
    (work / "Cargo.toml").write_text(
        '[package]\nname = "libavif-rs-abi-probe"\nversion = "0.0.0"\nedition = "2021"\n'
        '[workspace]\n[[bin]]\nname = "probe"\npath = "probe.rs"\n'
        f'[dependencies]\navif = {{ package = {json.dumps(rust_package)}, path = {json.dumps(str(rust_root))}, features = {json.dumps(features)} }}\n'
        '[profile.dev]\npanic = "unwind"\noverflow-checks = false\n'
        '[profile.release]\npanic = "unwind"\noverflow-checks = false\n'
    )
    # The probe is a Rust consumer of an rlib, so it owns the runtime search
    # path for the same locally built AVM dylib that CMake consumers link.
    local_avm = ROOT / "build/deps/avm-build"
    links = [f"cargo:rustc-link-arg=-Wl,-rpath,{local_avm}"] if "avm" in features and local_avm.exists() else []
    if args.root_profile or args.production_profile:
        # Selected shared optional engines likewise resolve through the probe.
        from root_variants import native_links
        links += [f"cargo:rustc-link-arg=-Wl,-rpath,{path}" for path in native_links(generated['variant'])[1]]
    (work / "build.rs").write_text(
        "fn main() {" + "".join(f" println!({json.dumps(link)});" for link in links) + " }\n"
    )
    command = ["cargo", "run", "--offline", "--quiet", "--manifest-path", str(work / "Cargo.toml"),
               "--target-dir", str(profile_target(generated) if args.root_profile
                                   else production_target or ROOT / 'target')]
    if rust_profile == 'release':
        command.append('--release')
    actual = [int(line) for line in subprocess.check_output(command, cwd=ROOT, text=True).splitlines()]
    if len(actual) != len(expected):
        raise SystemExit("Layout probes returned different numbers of checks")
    differences = [f"{name}: C={c}, Rust={value}" for (name, c), value in zip(expected, actual) if c != value]
    if differences:
        raise SystemExit("ABI layout mismatch:\n" + "\n".join(differences))
    report = {"records": checked_records, "checks": len(expected), "skipped": skipped}
    (work / "summary.json").write_text(json.dumps(report, indent=2) + "\n")
    print(f"ABI layouts match: {checked_records} named record instances, {len(expected)} size/alignment/field-offset checks")
    print(f"Explicitly excluded anonymous/bitfield records: {len(skipped)} (see {work / 'summary.json'})")


if __name__ == "__main__":
    main()
