import esbuild from "esbuild";

export function buildSync(options = {}) {
    let baseOptions = {
        bundle: true,
        logLevel: "info",
    };
    let result = esbuild.buildSync({
        ...baseOptions,
        ...options,
    });

    return result;
}
