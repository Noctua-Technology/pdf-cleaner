# clean-pdf

Module for easily removing text and other content from a pdf.

```bash
npm i @noctuatech/pdf-cleaner
```

## API

This package exposes a small set of functions through the `cleaner()` initializer which prepares the library and returns the available methods.

All methods operate on a PDF provided as a `Uint8Array` (or a Node.js `Buffer` which is compatible) and return a `Uint8Array` containing the modified PDF bytes.

### cleaner()

Initializes the library and returns the PDFDocument class.

```ts
import { cleaner } from "@nocutatech/pdf-cleaner";

const { PDFDocument } = await cleaner();
```

### PDFDocument.filterOperations

Filters content stream operators according to the provided list and mode (see `Mode` enum below).

```ts
import { cleaner, Mode } from "@nocutatech/pdf-cleaner";
import fs from "node:fs/promises";

const { PDFDocument } = await cleaner();

const doc = await PDFDocument.fromBytes(
  await fs.readFile("./test.pdf")
);

const embeddedImagesRemoved = await doc.filterOperations(
  ["BI", "ID", "EI"],
  Mode.Remove
);
```

### Cleaner.removeText

Removes text drawing operations from the PDF and returns the cleaned PDF bytes.

```ts
import { cleaner, Mode } from "@nocutatech/pdf-cleaner";
import fs from "node:fs/promises";

const { PDFDocument } = await cleaner();

const doc = await PDFDocument.fromBytes(
  await fs.readFile("./test.pdf")
);

const embeddedImagesRemoved = await doc.removeText();
```

### Cleaner.leaveOnlyText

Keeps only text drawing operators and removes other content.

```ts
import { cleaner, Mode } from "@nocutatech/pdf-cleaner";
import fs from "node:fs/promises";

const { PDFDocument } = await cleaner();

const doc = await PDFDocument.fromBytes(
  await fs.readFile("./test.pdf")
);

const embeddedImagesRemoved = await doc.leaveOnlyText();
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
