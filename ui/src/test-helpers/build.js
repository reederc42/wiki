import * as esbuild from "esbuild";

export function buildSync(options = {}, env = "test") {
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
