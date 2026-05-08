#!/usr/bin/env node
import { readFileSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";

const filePath = process.argv[2]
  ?? resolve(import.meta.dirname, "../data/tag_registry.json");

const raw = readFileSync(filePath, "utf8");
const registry = JSON.parse(raw);

if (Array.isArray(registry.albums)) {
  registry.albums.sort((a, b) => {
    const dateA = a.releaseDate;
    const dateB = b.releaseDate;
    if (dateA && !dateB) return -1;
    if (!dateA && dateB) return 1;
    if (dateA && dateB && dateA !== dateB) return dateA.localeCompare(dateB);
    const numA = parseInt(a.cid, 10);
    const numB = parseInt(b.cid, 10);
    if (numA !== numB) return numA - numB;
    return a.cid.localeCompare(b.cid);
  });
}

const sorted = JSON.stringify(registry, null, 2) + "\n";

if (sorted === raw) {
  console.log("tag_registry.json: already sorted by cid");
  process.exit(0);
}

writeFileSync(filePath, sorted, "utf8");
console.log(`tag_registry.json: sorted ${registry.albums.length} albums by cid`);
