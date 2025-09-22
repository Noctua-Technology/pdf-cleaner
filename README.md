# clean-pdf

Module for easily removing text and other content from a pdf.

```bash
npm i @noctuatech/pdf-cleaner
```

## API

This package exposes a small set of functions through the `cleaner()` initializer which prepares the library and returns the available methods.

All methods operate on a PDF provided as a `Uint8Array` (or a Node.js `Buffer` which is compatible) and return a `Uint8Array` containing the modified PDF bytes.

### cleaner()

Initializes the library and returns the available operations.

Signature:

```ts
async function cleaner(): Promise<Cleaner>
```

Returns an object with the following methods:

- `removeText(bytes: Uint8Array): Uint8Array` — removes text drawing operations from the PDF and returns the cleaned PDF bytes.

- `filterOperations(bytes: Uint8Array, operators: string[], mode: Mode): Uint8Array` — filters content stream operators according to the provided list and mode (see `Mode` enum below).

- `leaveOnlyText(bytes: Uint8Array): Uint8Array` — keeps only text drawing operators and removes other content.

Example:

```ts
import fs from "node:fs/promises";
import { cleaner, Mode } from "@nocutatech/pdf-cleaner";

const { removeText, filterOperations } = await cleaner();

const pdf = await fs.readFile("./test.pdf");

// Remove text from the PDF
const withoutText = await removeText(pdf);
await fs.writeFile("./no-text.pdf", withoutText);

// Remove specific operators (example operators are PDF content stream operators like 'Tj', 'TJ', 'Do', etc.)
const cleaned = await filterOperations(pdf, ["Tj", "TJ"], Mode.Remove);
await fs.writeFile("./filtered.pdf", cleaned);
```

### Types / enums

The `Mode` enum has two values:

```ts
enum Mode {
  Keep = 0,
  Remove = 1,
}
```

- `Mode.Keep` — when used with `filterOperations` will keep the listed operators and remove others.
- `Mode.Remove` — will remove the listed operators.
