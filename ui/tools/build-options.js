/* eslint-env node */

import resolve from "esbuild-plugin-resolve";

export const options = {
    prod: {
        minify: true,
        drop: ["console"],
        alias: {
            reefjs: "./node_modules/reefjs/src/reef.js",
        },
    },

    dev: {
        sourcemap: true,
    },

    mock: {
        plugins: [],
    },

    server: {
        plugins: [
            resolve({
                "../api/mock/subject": "../api/server/subject",
                "../auth/mock/user": "../auth/server/user",
            }),
        ],
    },
};
