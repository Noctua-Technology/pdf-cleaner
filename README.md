# clean-pdf

Module for easily removing text and other content from a pdf.

```bash
npm i @noctuatech/pdf-cleaner
```

```ts
import { cleaner } from "@nocutatech/pdf-cleaner";
import fs from "node:fs/promises";

const { removeText } = await cleaner();

const fileToBeCleaned = await fs.readFile("./test.pdf");

const cleanedFile = await removeText(fileToBeCleaned);

await fs.writeFile("./cleaned.pdf", cleanedFile);
```