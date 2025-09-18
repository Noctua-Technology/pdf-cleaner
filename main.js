import fs from "node:fs/promises";
import path from "node:path";

import sourceInit, { removeText, filterOperations } from "./pkg/pdf_cleaner.js";

const wasm = await fs.readFile(
  path.join(import.meta.dirname, "pkg/pdf_cleaner_bg.wasm")
);

export async function cleaner() {
  await sourceInit({ module_or_path: wasm });

  return { removeText, filterOperations };
}
