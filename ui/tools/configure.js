/* eslint-env node */

import fs from "node:fs";

import yargs from "yargs";
import { hideBin } from "yargs/helpers";

const configFile = "./src/config.json";
export const defaults = JSON.parse(fs.readFileSync("config.default.json"));

export function configure(config) {
    let mergedConfig = defaults;
    for (const v in defaults) {
        if (v in config) {
            mergedConfig[v] = config[v];
        }
    }
    fs.writeFileSync(configFile, JSON.stringify(mergedConfig));
}

function main() {
    const argv = yargs(hideBin(process.argv)).default(defaults).argv;
    configure(argv);
}

if (process.argv.includes(import.meta.url.substring("file://".length))) main();
