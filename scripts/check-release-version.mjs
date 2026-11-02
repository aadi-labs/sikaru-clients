// Checks that every client in this repository carries one release version.
// Usage: node scripts/check-release-version.mjs [tag]   (run from the repository root)
import { readFileSync } from "node:fs";
import { resolve } from "node:path";

function tomlSections(path) {
  const sections = new Map();
  let current = "";
  for (const line of readFileSync(resolve(path), "utf8").split(/\r?\n/)) {
    const header = line.match(/^\s*\[([^\[\]]+)\]\s*(#.*)?$/);
    if (header) {
      current = header[1].trim();
      continue;
    }
    if (/^\s*\[\[/.test(line)) {
      current = null; // array-of-tables entries never hold a release version
      continue;
    }
    const version = line.match(/^\s*version\s*=\s*"([^"]+)"/);
    if (version && current !== null && !sections.has(current)) sections.set(current, version[1]);
  }
  return sections;
}

function pythonVersion(path) {
  const sections = tomlSections(path);
  return sections.get("project") ?? sections.get("tool.poetry");
}

const versions = {
  "cli/Cargo.toml": tomlSections("cli/Cargo.toml").get("package"),
  "sdk/python/pyproject.toml": pythonVersion("sdk/python/pyproject.toml"),
  "sdk/typescript/package.json": JSON.parse(readFileSync(resolve("sdk/typescript/package.json"), "utf8")).version,
};
const tag = process.argv[2];
if (tag !== undefined) versions[`tag ${tag}`] = tag.replace(/^v/, "");

const distinct = new Set(Object.values(versions));
for (const [source, version] of Object.entries(versions)) console.log(`${source}: ${version ?? "(missing)"}`);
if (distinct.size !== 1 || distinct.has(undefined)) {
  console.error("Release versions differ; set every client to the same version.");
  process.exit(1);
}
console.log(`Release version ${[...distinct][0]} is consistent.`);
