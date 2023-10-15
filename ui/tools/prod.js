/*eslint-env node */

import { build } from "./build.js";

await build(
    {
        minify: true,
    },
    "prod",
);
