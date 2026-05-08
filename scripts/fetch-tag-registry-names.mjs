#!/usr/bin/env node
import { readFileSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";

const API_BASE = "https://monster-siren.hypergryph.com/api";
const registryPath = resolve(import.meta.dirname, "../data/tag_registry.json");

const raw = readFileSync(registryPath, "utf8");
const registry = JSON.parse(raw);

console.log("Fetching albums from Monster Siren API...");
const res = await fetch(`${API_BASE}/albums`);
if (!res.ok) {
  console.error(`API request failed: ${res.status} ${res.statusText}`);
  process.exit(1);
}

const body = await res.json();
const remoteAlbums = body.data ?? body;

const nameMap = new Map();
for (const album of remoteAlbums) {
  nameMap.set(String(album.cid), album.name);
}

let updated = 0;
for (const entry of registry.albums) {
  const remoteName = nameMap.get(entry.cid);
  if (remoteName && entry.name !== remoteName) {
    entry.name = remoteName;
    updated++;
  }
}

if (updated === 0) {
  console.log("All album names are already up to date.");
  process.exit(0);
}

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

writeFileSync(registryPath, JSON.stringify(registry, null, 2) + "\n", "utf8");
console.log(`Updated ${updated} album name(s).`);
