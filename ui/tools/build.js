/* eslint-env node */

import fs from "node:fs";

import esbuild from "esbuild";
import yargs from "yargs";
import { hideBin } from "yargs/helpers";

import { options, apiOptions, authOptions, merge } from "./build-options.js";
import { configure } from "./configure.js";
import { isMain } from "./is-main.js";

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
        .default("api", "mock")
        .default("auth", "mock")
        .argv;

    configure(argv, argv.api, argv.auth);

    await build(
        merge([
            options[argv.build],
            apiOptions[argv.api],
            authOptions[argv.auth],
        ]),
        argv.build,
    );
}

if (isMain(import.meta.url)) await main();
