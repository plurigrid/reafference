import { copyFile, mkdir, writeFile } from "node:fs/promises";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

import {
  buildHealthFrameBytes,
  buildHealthSnapshot,
  decodeFrameBytes,
  frameHashHex,
  stripAnsi,
  toHexDump,
} from "../shared/health-frame.mjs";

const scriptDir = dirname(fileURLToPath(import.meta.url));
const repoRoot = resolve(scriptDir, "..");
const outDir = resolve(repoRoot, "build/health");

await mkdir(outDir, { recursive: true });
await mkdir(resolve(outDir, "shared"), { recursive: true });

const snapshot = buildHealthSnapshot();
const frameBytes = buildHealthFrameBytes(snapshot);
const frameHash = await frameHashHex(frameBytes);
const frameText = decodeFrameBytes(frameBytes);
const plainText = stripAnsi(frameText);
const hexText = toHexDump(frameBytes);

const manifest = {
  stamp: snapshot.stamp,
  bytes: frameBytes.length,
  sha256: frameHash,
  files: {
    ansi: "frame.ansi",
    plain: "frame.txt",
    hex: "frame.hex.txt",
  },
};

await writeFile(resolve(outDir, "frame.ansi"), Buffer.from(frameBytes));
await writeFile(resolve(outDir, "frame.txt"), plainText);
await writeFile(resolve(outDir, "frame.hex.txt"), `${hexText}\n`);
await writeFile(resolve(outDir, "frame.json"), `${JSON.stringify(manifest, null, 2)}\n`);
await copyFile(resolve(repoRoot, "reafferance.health/index.html"), resolve(outDir, "index.html"));
await copyFile(resolve(repoRoot, "shared/health-frame.mjs"), resolve(outDir, "shared/health-frame.mjs"));

process.stdout.write(`built ${frameBytes.length} bytes :: ${frameHash}\n`);
