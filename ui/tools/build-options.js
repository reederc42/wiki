/*eslint-env node */

export const prod = {
    minify: true,
    drop: ["console"],
    alias: {
        "reefjs": "./node_modules/reefjs/src/reef.js",
    }
};

export const dev = {
    sourcemap: true,
};
