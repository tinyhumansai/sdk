#!/usr/bin/env node

import { readFile, writeFile } from "node:fs/promises";

const manifestPath = "Cargo.toml";
const bump = process.argv[2];
if (!["patch", "minor", "major"].includes(bump)) {
  console.error("usage: node scripts/bump-versions.mjs <patch|minor|major>");
  process.exit(1);
}

const manifest = await readFile(manifestPath, "utf8");
const match = manifest.match(/^version = "(\d+)\.(\d+)\.(\d+)"$/m);
if (!match) throw new Error(`Could not find package version in ${manifestPath}`);

let [, major, minor, patch] = match.map(Number);
if (bump === "major") [major, minor, patch] = [major + 1, 0, 0];
if (bump === "minor") [minor, patch] = [minor + 1, 0];
if (bump === "patch") patch += 1;

const version = `${major}.${minor}.${patch}`;
await writeFile(
  manifestPath,
  manifest.replace(match[0], `version = "${version}"`),
);
console.log(version);
