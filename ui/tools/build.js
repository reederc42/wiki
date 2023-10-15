/*eslint-env node */

import fs from "fs";
import esbuild from "esbuild";

const outdir = "dist";

export async function build(options = {}, env = "") {
    console.log(`building for ${env}...`);

    fs.rmSync(outdir, {
        recursive: true,
        force: true,
    });

    fs.mkdirSync(outdir);
    fs.cpSync("public", outdir, {
        recursive: true,
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

    console.log(`finishing building for ${env}`);
}

export function buildSync(options = {}, env = "") {
    console.log(`building for ${env}...`);

    let baseOptions = {
        bundle: true,
        logLevel: "info",
    };
    esbuild.buildSync({
        ...baseOptions,
        ...options,
    });

    console.log(`finished building for ${env}`);
}
