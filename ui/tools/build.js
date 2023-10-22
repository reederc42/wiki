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
