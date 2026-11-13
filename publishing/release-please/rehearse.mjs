// Offline Release Please rehearsal for the inert drafts in this directory.
//
// Runs the real Release Please Manifest, strategies, plugins and updaters
// against this checkout's manifests and a synthetic commit history. It builds
// no GitHub client, opens no pull request, creates no tag and publishes
// nothing; the "repository" is an in-memory map of local files. Adapted from
// sebastian-software/ferromark `scripts/lib/release-rehearsal.mjs` (ADR-0020).
//
// Usage (from the repository root, with release-please 17.6.0 installed in a
// directory outside the repository, see docs/release-automation.md):
//
//   node publishing/release-please/rehearse.mjs <node_modules> shared
//   node publishing/release-please/rehearse.mjs <node_modules> per-port
//
// Exit status 0 means every scenario produced the expected versions and no
// first-party requirement was left stale; 1 lists each finding.

import { readFileSync, existsSync } from "node:fs";
import { createRequire } from "node:module";
import { join, resolve } from "node:path";

const [nodeModules, shape, option] = process.argv.slice(2);
if (!nodeModules || !["shared", "per-port"].includes(shape) || ![undefined, "--key-by-package"].includes(option)) {
  console.error("usage: rehearse.mjs <node_modules dir> shared|per-port [--key-by-package]");
  process.exit(2);
}
// `--key-by-package` rehearses the proposed manifest fix in memory: every
// renamed first-party dependency (`webp = { package = "libwebp-rs", … }`) is
// keyed by its package name instead. The library target names (`webp`, `jxl`,
// `sharpyuv`, `dav1d`, `aom`) are unchanged, so Rust sources would not change;
// only `dep:` feature references would.
const keyByPackage = option === "--key-by-package";
const require = createRequire(join(resolve(nodeModules), "rehearsal.js"));
const { Manifest, setLogger } = require("release-please");
const TOML = require("@iarna/toml");
const rpVersion = require("release-please/package.json").version;

const root = process.cwd();
const draft = "publishing/release-please";
const configFile = `${draft}/config-${shape === "shared" ? "shared-version" : "per-port"}.json`;
const manifestFile = `${draft}/manifest-${shape === "shared" ? "shared-version" : "per-port"}.json`;
const quiet = Object.fromEntries(
  ["info", "debug", "trace", "warn", "error"].map((n) => [n, () => {}]),
);
const warnings = [];
const capture = { ...quiet, warn: (...a) => warnings.push(a.join(" ")) };

function readRepoFiles() {
  const read = (f) => readFileSync(resolve(root, f), "utf8");
  const cargo = TOML.parse(read("Cargo.toml"));
  const config = JSON.parse(read(configFile));
  const paths = new Set(["Cargo.toml", "Cargo.lock"]);
  for (const member of cargo.workspace.members) {
    if (member !== ".") paths.add(`${member}/Cargo.toml`);
  }
  for (const [pkgPath, pkg] of Object.entries(config.packages)) {
    for (const entry of pkg["extra-files"] ?? []) {
      const p = entry.path.startsWith("/")
        ? entry.path.slice(1)
        : pkgPath === "."
          ? entry.path
          : `${pkgPath}/${entry.path}`;
      paths.add(p);
    }
  }
  const files = new Map([...paths].filter((p) => existsSync(resolve(root, p))).map((p) => [p, read(p)]));
  if (shape === "per-port") moveEngineOutOfRoot(files);
  if (keyByPackage) {
    for (const [p, text] of files) {
      if (!p.endsWith("Cargo.toml")) continue;
      files.set(p, text.replace(/^(\s*)[A-Za-z0-9_-]+ = \{ package = "([^"]+)", (path = )/gm, "$1$2 = { $3"));
    }
  }
  files.set("release-please-config.json", read(configFile));
  files.set(".release-please-manifest.json", read(manifestFile));
  return files;
}

// Configuration B needs a virtual workspace root (the ferrugo layout): with a
// root `[package]`, the `rust` strategy for "." rewrites every member to the
// engine's version. This applies the move in memory only: the `[package]` half
// of the root manifest becomes `crates/ferrisma/Cargo.toml` with its path
// dependencies re-anchored, and the root keeps `[workspace]` and the profiles.
function moveEngineOutOfRoot(files) {
  const text = files.get("Cargo.toml");
  const at = text.indexOf("\n[package]\n");
  const workspace = text
    .slice(0, at + 1)
    .replace(/^(\s*)"\.",$/m, '$1"crates/ferrisma",')
    .replace('default-members = [".",', 'default-members = ["crates/ferrisma",');
  const engine = text
    .slice(at + 1)
    .replaceAll('path = "crates/', 'path = "../')
    .replaceAll('path = "codecs/', 'path = "../../codecs/');
  files.set("Cargo.toml", workspace);
  files.set("crates/ferrisma/Cargo.toml", engine);
  const adapter = "crates/ferrisma-libavif/Cargo.toml";
  files.set(adapter, files.get(adapter).replaceAll('path = "../.."', 'path = "../ferrisma"'));
}

// The first-party packages and their paths, from the workspace itself.
function workspacePackages(files) {
  const cargo = TOML.parse(files.get("Cargo.toml"));
  const out = new Map(cargo.package ? [[cargo.package.name, "."]] : []);
  for (const member of cargo.workspace.members) {
    if (member === ".") continue;
    out.set(TOML.parse(files.get(`${member}/Cargo.toml`)).package.name, member);
  }
  return out;
}

function tagFor(config, pkgPath, version) {
  const pkg = config.packages[pkgPath];
  const component = pkg.component;
  const withComponent = pkg["include-component-in-tag"] ?? config["include-component-in-tag"] ?? true;
  return withComponent ? `${component}-v${version}` : `v${version}`;
}

// history: [{ message, files }] newest first; released: whether the manifest
// versions exist as tags (false for the very first release).
async function propose(files, history, released) {
  setLogger(capture);
  const config = JSON.parse(files.get("release-please-config.json"));
  const manifest = JSON.parse(files.get(".release-please-manifest.json"));
  const github = {
    repository: { owner: "sebastian-software", repo: "ferrisma", defaultBranch: "main" },
    async getFileJson(file) {
      return JSON.parse(files.get(file));
    },
    async getFileContentsOnBranch(file) {
      if (!files.has(file)) {
        const err = new Error(`not found: ${file}`);
        err.name = "FileNotFoundError";
        throw err;
      }
      return {
        parsedContent: files.get(file),
        content: Buffer.from(files.get(file)).toString("base64"),
        sha: "a".repeat(40),
      };
    },
    async findFilesByGlobAndRef(glob, _ref, prefix) {
      const base = prefix === undefined || prefix === "." ? "" : `${prefix}/`;
      const keys = [...files.keys()].filter((f) => f.startsWith(base)).map((f) => f.slice(base.length));
      if (!glob.includes("*")) {
        return keys.some((f) => f === glob || f.startsWith(`${glob}/`)) ? [glob] : [];
      }
      const re = new RegExp(`^${glob.split("*").map((s) => s.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")).join("[^/]*")}$`);
      return keys.filter((f) => re.test(f));
    },
    async *releaseIterator() {
      if (!released) return;
      for (const [pkgPath, version] of Object.entries(manifest)) {
        yield { tagName: tagFor(config, pkgPath, version), sha: "a".repeat(40), notes: "baseline" };
      }
    },
    // No tag exists before the first release; afterwards releaseIterator
    // already finds every component, so the tag fallback stays empty.
    async *tagIterator() {},
    async *mergeCommitIterator() {
      for (const [i, c] of history.entries()) {
        yield { sha: `${i + 1}`.padStart(40, "b"), message: c.message, files: c.files };
      }
      yield {
        sha: released ? "a".repeat(40) : config["bootstrap-sha"],
        message: "chore: preceding commit",
        files: [],
      };
    },
  };
  const m = await Manifest.fromManifest(github, "main", "release-please-config.json", ".release-please-manifest.json", {
    logger: capture,
  });
  const candidates = await m.buildPullRequests();
  if (candidates.length !== 1) {
    throw new Error(`expected exactly one release pull request, got ${candidates.length}`);
  }
  const pr = candidates[0];
  const updated = new Map(files);
  for (const u of pr.updates) {
    if (!updated.has(u.path) && !u.createIfMissing) {
      warnings.push(`update for missing file skipped: ${u.path}`);
      continue;
    }
    updated.set(u.path, u.updater.updateContent(updated.get(u.path), quiet));
  }
  return { files: updated, title: pr.title.toString(), paths: pr.updates.map((u) => u.path) };
}

// Every first-party requirement must name the version its target now carries,
// whether the dependency is keyed by the package name or renamed.
function audit(files, packages, expected) {
  const findings = [];
  const versions = new Map();
  for (const [name, path] of packages) {
    const file = path === "." ? "Cargo.toml" : `${path}/Cargo.toml`;
    versions.set(name, TOML.parse(files.get(file)).package.version);
  }
  for (const [name, want] of Object.entries(expected)) {
    if (versions.get(name) !== want) findings.push(`${name}: package version ${versions.get(name)}, expected ${want}`);
  }
  for (const [name, path] of packages) {
    const file = path === "." ? "Cargo.toml" : `${path}/Cargo.toml`;
    const parsed = TOML.parse(files.get(file));
    if (parsed.package.publish !== false) findings.push(`${name}: publish flag changed`);
    for (const kind of ["dependencies", "dev-dependencies", "build-dependencies"]) {
      for (const [key, dep] of Object.entries(parsed[kind] ?? {})) {
        if (typeof dep !== "object" || dep.path === undefined) continue;
        const target = dep.package ?? key;
        if (!versions.has(target) || dep.version === undefined) continue;
        const req = dep.version.replace(/^[=^]/, "");
        if (req !== versions.get(target)) {
          findings.push(
            `${name}: ${kind}.${key}${dep.package ? ` (package ${dep.package})` : ""} requires ${dep.version}, but ${target} is ${versions.get(target)}`,
          );
        }
      }
    }
  }
  for (const lock of [...files.keys()].filter((f) => f.endsWith("Cargo.lock"))) {
    for (const p of TOML.parse(files.get(lock)).package ?? []) {
      if (p.source || !versions.has(p.name)) continue;
      if (p.version !== versions.get(p.name)) findings.push(`${lock}: ${p.name} ${p.version}, manifest says ${versions.get(p.name)}`);
    }
  }
  return { findings, versions };
}

function setManifestAfterRelease(files, packages, config) {
  const versions = {};
  for (const pkgPath of Object.keys(config.packages)) {
    const file = pkgPath === "." ? "Cargo.toml" : `${pkgPath}/Cargo.toml`;
    versions[pkgPath] = TOML.parse(files.get(file)).package.version;
  }
  const next = new Map(files);
  next.set(".release-please-manifest.json", `${JSON.stringify(versions, null, 2)}\n`);
  return next;
}

const pathOf = (packages, name) => packages.get(name);
const touch = (packages, ...names) => names.map((n) => `${pathOf(packages, n) === "." ? "" : `${pathOf(packages, n)}/`}src/lib.rs`);

const base = readRepoFiles();
const packages = workspacePackages(base);
const config = JSON.parse(base.get("release-please-config.json"));
const published = Object.keys(config.packages).map((p) => [...packages].find(([, path]) => path === p)[0]);
const all = [...packages.keys()];
const everyPublished = [...published];
let failed = false;

function report(label, result, expected, allow = []) {
  const { findings, versions } = audit(result.files, packages, expected);
  const unexpected = findings.filter((f) => !allow.some((a) => f.includes(a)));
  console.log(`\n## ${label}\n${result.title}`);
  console.log([...versions].map(([n, v]) => `  ${n} ${v}`).join("\n"));
  for (const f of findings) console.log(`  ${unexpected.includes(f) ? "FINDING" : "known"}: ${f}`);
  if (unexpected.length) failed = true;
}

console.log(`release-please ${rpVersion}, draft ${configFile}`);

const sharedAll = (v) => Object.fromEntries(all.map((n) => [n, v]));

if (shape === "shared") {
  const first = await propose(
    base,
    [{ message: "feat: first functional pre-release\n\nRelease-As: 0.1.0", files: ["src/lib.rs"] }],
    false,
  );
  report("first release (Release-As: 0.1.0)", first, sharedAll("0.1.0"));
  const released = setManifestAfterRelease(first.files, packages, config);
  for (const [label, message, file, want] of [
    ["fix in a port", "fix(webp): reject a truncated VP8L header", "codecs/libwebp-rs/src/lib.rs", "0.1.1"],
    ["feature", "feat(jxl): admit the lossless effort 2 profile", "codecs/libjxl-rs/src/lib.rs", "0.2.0"],
    ["breaking change below 1.0", "feat(pipeline)!: make TransformOptions non-exhaustive", "src/pipeline.rs", "0.2.0"],
  ]) {
    report(label, await propose(released, [{ message, files: [file] }], true), sharedAll(want));
  }
  const noPreMajor = new Map(released);
  const cfg = JSON.parse(released.get("release-please-config.json"));
  cfg["bump-minor-pre-major"] = false;
  noPreMajor.set("release-please-config.json", JSON.stringify(cfg));
  const drift = await propose(noPreMajor, [{ message: "feat(pipeline)!: remove an option", files: ["src/pipeline.rs"] }], true);
  const drifted = drift.title.includes("1.0.0");
  console.log(`\n## negative control: without bump-minor-pre-major a breaking change proposes 1.0.0: ${drifted ? "confirmed" : "NOT confirmed"}`);
  if (!drifted) failed = true;
} else {
  const first = await propose(
    base,
    [{ message: "feat: first functional pre-release\n\nRelease-As: 0.1.0", files: touch(packages, ...everyPublished) }],
    false,
  );
  report("first release (Release-As: 0.1.0 on a commit touching every published package)", first, Object.fromEntries(everyPublished.map((n) => [n, "0.1.0"])));
  const released = setManifestAfterRelease(first.files, packages, config);
  const fix = await propose(released, [{ message: "fix(webp): reject a truncated VP8L header", files: touch(packages, "libwebp-rs") }], true);
  report("fix in libwebp-rs only", fix, { "libwebp-rs": "0.1.1", "libjxl-rs": "0.1.0", "libjpeg-turbo-rs": "0.1.0" });
  const feat = await propose(released, [{ message: "feat(jxl): admit the lossless effort 2 profile", files: touch(packages, "libjxl-rs") }], true);
  report("feature in libjxl-rs only", feat, { "libjxl-rs": "0.2.0", "libwebp-rs": "0.1.0", "libjpeg-turbo-rs": "0.1.0" });
  const engine = await propose(released, [{ message: "feat(pipeline)!: make TransformOptions non-exhaustive", files: touch(packages, "ferrisma") }], true);
  report("breaking engine change", engine, { ferrisma: "0.2.0", "ferrisma-kernels": "0.2.0", "libwebp-rs": "0.1.0", "libjxl-rs": "0.1.0" });
}

if (warnings.length) {
  console.log("\n## library warnings");
  for (const w of [...new Set(warnings)]) console.log(`  ${w}`);
}
console.log(failed ? "\nRESULT: findings above" : "\nRESULT: clean");
process.exit(failed ? 1 : 0);
