#!/usr/bin/env python3
"""Compare libaom's four native-VMAF tuning modes with a pinned model."""
import argparse
import hashlib
import json
from pathlib import Path
import subprocess

from build_aom_vmaf import REVISION
from check_aom_cli import pixels

ROOT = Path(__file__).resolve().parents[1]
MODELS = {
    'float': ('vmaf_float_v0.6.1.json', '26573aa333346f1109226e7283969e8b3581119a1ab7f18897767351dd963ff8'),
    'integer': ('vmaf_v0.6.1.json', '5950d61fa1f861bd45d8149d80539ed9f3376cfc2495b8f0fa8e9f57cb131ee3'),
}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--debug', action='store_true')
    parser.add_argument('--model', choices=MODELS, default='integer')
    args = parser.parse_args()
    profile = 'debug' if args.debug else 'release'
    source = ROOT / 'build/deps/vmaf'
    revision = subprocess.check_output(['git', '-C', source, 'rev-parse', 'HEAD'], text=True).strip()
    model_name, model_sha256 = MODELS[args.model]
    model = source / 'model' / model_name
    if revision != REVISION or hashlib.sha256(model.read_bytes()).hexdigest() != model_sha256:
        raise SystemExit('The original VMAF model does not match the pinned source')
    work = ROOT / f'build/aom-vmaf-modes-{profile}-{args.model}'
    work.mkdir(parents=True, exist_ok=True)
    modes = ['vmaf_with_preprocessing', 'vmaf_without_preprocessing', 'vmaf', 'vmaf_neg']
    records = []
    for depth in [8, 10, 12]:
        raw = work / f'{depth}.yuv'
        raw.write_bytes(pixels(128, 96, depth, '420', 1))
        for tune in modes:
            outputs, decoded = [], []
            for label, directory in [('c', f'aom-native-{profile}'), ('rust', f'aom-rust-{profile}')]:
                name = f'{depth}-{tune}-{label}'
                target = work / f'{name}.ivf'
                command = [str(ROOT / f'build/{directory}/aomenc'), '--quiet', '--width=128', '--height=96',
                           '--limit=1', '--passes=1', '--usage=0', '--cpu-used=6', '--threads=1',
                           '--end-usage=q', '--cq-level=28', '--ivf', '--lag-in-frames=0',
                           f'--bit-depth={depth}', f'--input-bit-depth={depth}',
                           f'--profile={2 if depth == 12 else 0}', '--i420', f'--tune={tune}',
                           f'--vmaf-model-path={model}', '-o', str(target), str(raw)]
                with (work / f'{name}.log').open('w') as log:
                    result = subprocess.run(command, stdout=log, stderr=subprocess.STDOUT)
                if result.returncode:
                    raise SystemExit(f'VMAF mode failed: {name}: inspect {work / (name + ".log")}')
                outputs.append(target.read_bytes())
                output = work / f'{name}.yuv'
                with (work / f'{name}-decode.log').open('w') as log:
                    subprocess.run([str(ROOT / f'build/{directory}/aomdec'), '--rawvideo', '-o', output,
                                    work / f'{depth}-{tune}-c.ivf'], stdout=log, stderr=subprocess.STDOUT, check=True)
                decoded.append(output.read_bytes())
            if outputs[0] != outputs[1] or decoded[0] != decoded[1] or len(decoded[0]) != raw.stat().st_size:
                raise SystemExit(f'VMAF tuning bitstream/pixel mismatch: {depth}/{tune}')
            records.append(dict(bitdepth=depth, tune=tune, frames=1, exact=True,
                                input_sha256=hashlib.sha256(raw.read_bytes()).hexdigest(),
                                encoded_bytes=len(outputs[0]), encoded_sha256=hashlib.sha256(outputs[0]).hexdigest(),
                                decoded_sha256=hashlib.sha256(decoded[0]).hexdigest()))
            print(f'Exact native-VMAF mode: {depth}/{tune}', flush=True)
    # Preserve the original process-level failure for an unusable model path.
    errors = []
    for label, directory in [('c', f'aom-native-{profile}'), ('rust', f'aom-rust-{profile}')]:
        command = [str(ROOT / f'build/{directory}/aomenc'), '--quiet', '--width=128', '--height=96',
                   '--limit=1', '--passes=1', '--cpu-used=6', '--threads=1', '--ivf', '--lag-in-frames=0',
                   '--tune=vmaf', f'--vmaf-model-path={work / "absent-model.json"}',
                   '-o', str(work / f'absent-model-{label}.ivf'), str(work / '8.yuv')]
        value = subprocess.run(command, stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
        (work / f'absent-model-{label}.log').write_bytes(value.stdout)
        errors.append((value.returncode, value.stdout))
    if errors[0] != errors[1] or errors[0][0] != 1 or b'Fatal error: Failed to load VMAF model.' not in errors[0][1]:
        raise SystemExit('Missing VMAF model failure differs from the original')
    report = dict(profile=profile, model=f'model/{model_name}', model_sha256=model_sha256,
                  vmaf_tag='v3.2.1', vmaf_revision=REVISION,
                  pkg_config_version=subprocess.check_output(['pkg-config', '--modversion', 'libvmaf'], text=True).strip(),
                  native_vmaf_retained=True, vmaf_rust_port=False,
                  missing_model_exact_failure=True, missing_model_exit_code=errors[0][0], cases=records)
    (work / 'summary.json').write_text(json.dumps(report, indent=2) + '\n')


if __name__ == '__main__':
    main()
