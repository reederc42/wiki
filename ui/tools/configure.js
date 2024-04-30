/* eslint-env node */

import fs from "node:fs";

import yargs from "yargs";
import { hideBin } from "yargs/helpers";

import { isMain } from "./is-main.js";

const configFile = "./src/config.json";
export const defaults = JSON.parse(fs.readFileSync("config.default.json"));

export function configure(config, api, auth) {
    let mergedConfig = {
        ...defaults["api"][api],
        ...defaults["auth"][auth],
    };
    for (const v in mergedConfig) {
        if (v in config) {
            mergedConfig[v] = config[v];
        }
    }
    fs.writeFileSync(configFile, JSON.stringify(mergedConfig));
}

function main() {
    const argv = yargs(hideBin(process.argv))
        .default("api", "mock")
        .default("auth", "mock")
        .argv;
    configure(argv, argv.api, argv.auth);
}

if (isMain(import.meta.url)) main();
