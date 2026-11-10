// Generate Ardo routes from the repository's canonical Markdown.
import { readFileSync, mkdirSync, writeFileSync, statSync } from "node:fs";
import { dirname, relative, resolve, sep } from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";
import { unified } from "unified";
import remarkParse from "remark-parse";
import remarkGfm from "remark-gfm";

const homepage = resolve(dirname(fileURLToPath(import.meta.url)), "..");
const root = resolve(homepage, "..");
const pages = [
  ["docs/getting-started.md", "/guide/getting-started"],
  ["docs/status.md", "/guide/status"],
  ["docs/architecture.md", "/guide/architecture"],
  ["CONTRIBUTING.md", "/guide/development"],
  ["docs/validation.md", "/guide/validation"],
  ["codecs/libjpeg-turbo-rs/README.md", "/codecs/jpeg"],
  ["codecs/libwebp-rs/README.md", "/codecs/webp"],
  ["codecs/libjxl-rs/README.md", "/codecs/jpeg-xl"],
  ["codecs/libavif-rs/README.md", "/codecs/avif"],
] as const;
const routes = new Map<string, string>(pages.map(([source, route]) => [resolve(root, source), route]));

export function rewriteTarget(source: string, target: string): string {
  if (/^(?:[a-z][a-z\d+.-]*:|\/\/|#|\/)/i.test(target)) return target;
  const match = target.match(/^([^?#]*)(.*)$/);
  if (!match || !match[1]) return target;
  const path = resolve(dirname(source), decodeURI(match[1]));
  const repoPath = relative(root, path);
  if (repoPath === ".." || repoPath.startsWith(`..${sep}`)) throw Error(`Link escapes repository: ${target}`);
  const stats = statSync(path); // Missing targets fail the build, including mapped pages.
  const route = routes.get(path);
  if (route) return route + match[2];
  const kind = stats.isDirectory() ? "tree" : "blob";
  return `https://github.com/sebastian-software/ferrixel/${kind}/main/${repoPath.split(sep).map(encodeURIComponent).join("/")}${match[2]}`;
}

type Node = { type: string; url?: string; children?: Node[]; position?: { start: { offset?: number }; end: { offset?: number } } };
const parser = unified().use(remarkParse).use(remarkGfm);

export function render(source: string, markdown: string): string {
  const changes: { start: number; end: number; text: string }[] = [];
  function walk(node: Node): void {
    if ((node.type === "link" || node.type === "image" || node.type === "definition") && node.url && node.position) {
      const start = node.position.start.offset;
      const end = node.position.end.offset;
      if (start === undefined || end === undefined) throw Error("Markdown node has no source position");
      const replacement = rewriteTarget(source, node.url);
      if (replacement !== node.url) {
        // Rewrite the destination within its own parsed node, preserving all surrounding Markdown.
        const raw = markdown.slice(start, end);
        const at = raw.lastIndexOf(node.url);
        if (at < 0) throw Error(`Cannot locate Markdown destination: ${node.url}`);
        changes.push({ start: start + at, end: start + at + node.url.length, text: replacement });
      }
    }
    for (const child of node.children ?? []) walk(child);
  }
  walk(parser.parse(markdown) as Node);
  for (const change of changes.sort((a, b) => b.start - a.start)) {
    markdown = markdown.slice(0, change.start) + change.text + markdown.slice(change.end);
  }
  return markdown;
}

export function sync(): void {
  for (const [sourceName, route] of pages) {
    const source = resolve(root, sourceName);
    const markdown = readFileSync(source, "utf8");
    const title = /^# (.+)$/m.exec(markdown)?.[1].replaceAll("`", "");
    if (!title) throw Error(`Canonical guide has no title: ${sourceName}`);
    const destination = resolve(homepage, "app/routes", `${route.slice(1)}.md`);
    mkdirSync(dirname(destination), { recursive: true });
    writeFileSync(destination, `---\ntitle: ${JSON.stringify(title)}\n---\n\n${render(source, markdown)}`);
  }
}
if (process.argv[1] && import.meta.url === pathToFileURL(resolve(process.argv[1])).href) sync();
