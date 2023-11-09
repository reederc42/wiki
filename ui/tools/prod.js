import { build } from "./build.js";
import { prod } from "./build-options.js";

await build(prod, "prod");
