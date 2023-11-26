import { build } from "./build.js";
import { prod } from "./build-options.js";
import fs from "fs";

let config = JSON.parse(fs.readFileSync("./config.default.json"));
fs.writeFileSync("./src/config.json", JSON.stringify(config));

await build(prod, "prod");
