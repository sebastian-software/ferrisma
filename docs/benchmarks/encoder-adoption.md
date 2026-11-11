# Current AVIF encoder comparison

This procedure asks whether the workspace libavif/AOM encoder should replace
`rav1e` for a specified image, quality, geometry, thread configuration, and
host. It is not a release-readiness or production-safety assessment. The
historical measurements in [M2-libavif-backend](M2-libavif-backend.md) were
made on earlier revisions and remain historical. No current performance result
is asserted by this procedure.

## Workload and quality

The existing [corpus preparation](corpus-and-comparison.md) supplies the same
JPEG files and independent linear-light Lanczos3/SSIMULACRA2 references. The
default keeps the four historical cases: dog, flower, kodim03, and kodim23.
`COMPARE_IMAGE_SET=dog` or `flower` selects a bounded first run. `beach`,
`beach-24`, and `beach-48` use the separately downloaded and hash-verified
Joseph Gage CC BY-SA 2.0 photograph described in the corpus guide. The latter
two use native-pixel regions of the **same full 60.2 MP input**, not independent
photos or evidence of partial decoding. They may need substantially more RAM
for reference creation and are optional.

For each image, anchor quality (default 80), and thread setting
(one and all available cores), encode a 10-bit 4:4:4 `rav1e` speed-6 anchor.
Match libavif/AOM speeds 6 and 7 **separately for each thread setting** using
integer quality 0–100 and the existing bisection scorer. Byte interpolation
between bracketing scores is an estimate; the selected nearest integer quality
is the actual timed setting. Missing or non-bracketing targets, or a nearest
integer quality more than 0.5 SSIMULACRA2 from its target, fail before any
warmup or timing. Additional anchor qualities can be selected explicitly.
Every timed output is independently rescored: it must be within 0.1
SSIMULACRA2 of the matched candidate and within 0.5 of its own anchor. The
scorer is RGB-only and does not validate alpha or arbitrary profiles. Both
encoders use Ferrixel's same decode/transform path and output color policy;
the independent reference has the known JPEG decoder and ICC limitations in
the corpus guide. This comparison does not isolate encoding from decoding.

## Run

Build prerequisites include the pinned root Rust toolchain, NASM on x86-64,
Node 22+, a compatible Sharp installation for preparation, and `/usr/bin/time`.
Use a fresh output directory for each match. Select the same image set and
anchor qualities in every phase:

```sh
export BENCH_SHARP_NODE_MODULES=/path/to/node_modules
export COMPARE_IMAGE_SET=dog COMPARE_ENCODER_LEVELS=80
python3 -I scripts/bench/compare-sharp.py prepare /tmp/ferrixel-encoders
scripts/bench/compare-encoders.sh /tmp/ferrixel-encoders match
COMPARE_RUNS=5 COMPARE_ENCODER_RUN_ID=simd \
  scripts/bench/compare-encoders.sh /tmp/ferrixel-encoders time
COMPARE_RUNS=5 COMPARE_ENCODER_RUN_ID=scalar AOM_SIMD_CAPS_MASK=0 \
  scripts/bench/compare-encoders.sh /tmp/ferrixel-encoders time
```

`all` on the shell entry point runs match then time; it does not prepare the
corpus. Matching builds both encoder CLIs and the independent scorer together
with their locked dependencies, checks the source receipt before and after,
and records their hashes. `COMPARE_REPO` must name the checkout containing
the harness; a different checkout is rejected for this comparison. A relative
`CARGO_TARGET_DIR` is resolved against that checkout. Cargo-reported executable
paths must match the selected target directories; cross-target or other Cargo
configuration that changes those paths is rejected. The `time` phase never
rebuilds a binary. It refuses changed inputs,
references, binaries, tracked source diff, configuration, or matching output.
It also refuses a reused run ID. `COMPARE_MAX_LOAD` defaults to 4; an idle
window must arrive within `COMPARE_IDLE_TIMEOUT` seconds (default 900). Use
`COMPARE_RUNS` from 1 to 100. Timing requires a quiet host; the load average is
only a crude gate, not proof of isolation. The manual Linux workflow defaults to
dog, quality 80, and five samples. Its optional large lanes fetch and
verify the documented source before preparation.

## Evidence and interpretation

`encoders/matched.json` and `match-provenance.json` record each anchor and
bracket, input/reference/tool and output SHA-256 values, source commit and
tracked diff, scorer source and manifest, Cargo lockfiles, Rust toolchain, build flags, Sharp
versions, and the AOM SIMD mask. The matching directories retain the encoded
outputs and the tracked source patch for inspection. Untracked source files
are not captured; use a clean checkout for decision evidence. A match with
unbracketed cases stays visible but cannot be used for timing.

The first hosted run, [43073087629](https://github.com/sebastian-software/ferrixel/actions/runs/43073087629),
matched the dog image at anchor qualities 60 and 80. The quality-60 target was
bracketed, but its nearest integer AOM settings were 1.379–1.619 SSIMULACRA2
away for both speeds and thread settings. The four quality-80 settings were
within 0.5. Timing failed its quality check, so that run is **not timing
evidence**. This integer-quality gap does not establish an encoder or runtime
defect; other images and levels must be checked independently.

Each `encoders/time-<run-id>/` contains a provenance record, raw `samples.json`,
summary, and the final output for each lane. Each sample records its exact
command, output size/hash/score, process wall time including startup and I/O,
peak RSS, engine stage report, and before/after load. One warmup precedes each
lane; lane order reverses for alternating measured samples. Median, nearest-rank
p95, range and maximum observed RSS summarize the retained samples. Five
samples are too few to characterize tail latency. Each RSS observation is
from the same process as its wall-time sample; the scorer runs afterward.
`provenance.json` remains incomplete on interruption and hashes the completed
samples, summary, and retained final outputs. Timing captures host, toolchain,
flags, binary hashes and sizes, affinity count, and SIMD setting. The binary
sizes compare full CLI builds with different linked feature sets, not isolated
encoder libraries. Compare SIMD and scalar timings only after checking both
runs' source, flags, quality, and scores; they use the same matched targets.

Keep the raw artifact and exact workflow head with any report. One hosted runner
or one ARM machine cannot establish portable performance. Selection still
requires the two-architecture quality/resource/binary-size corpus criteria in
[ADR-0025](../adr/0025-codec-selection-reference-conformance-and-image-scope.md),
the nightly/native and safe-boundary decisions in
[ADR-0024](../adr/0024-pluggable-encoders-and-libavif-rs-backend.md), and the
existing codec reference and license gates.
