/* eslint-env node */

import fs from "node:fs";

import esbuild from "esbuild";
import yargs from "yargs";
import { hideBin } from "yargs/helpers";

import { prod, dev } from "./build-options.js";
import { defaults, configure } from "./configure.js";

const outdir = "dist";

export async function build(options = {}, env = "") {
    console.log(`building for ${env}...`);

    fs.rmSync(outdir, {
        recursive: true,
        force: true,
    });

    let baseOptions = {
        entryPoints: ["src/main.js"],
        bundle: true,
        outfile: outdir + "/index.js",
        logLevel: "info",
    };
    await esbuild.build({
        ...baseOptions,
        ...options,
    });

    fs.cpSync("public", outdir, {
        recursive: true,
    });

    console.log(`finishing building for ${env}`);
}

async function main() {
    const argv = yargs(hideBin(process.argv))
        .default("build", "prod")
        .default(defaults).argv;

    configure(argv);

    let options;
    switch (argv.build) {
        case "prod":
            options = prod;
            break;
        case "dev":
            options = dev;
            break;
    }

    await build(options, argv.build);
}

if (process.argv.includes(import.meta.url.substring("file://".length)))
    await main();
