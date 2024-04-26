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
};

export const apiOptions = {
    mock: {
        plugins: [],
    },

    server: {
        plugins: [
            resolve({
                "../api/mock/subject": "../api/server/subject",
            })
        ]
    }
}

export const authOptions = {
    mock: {
        plugins: [],
    },

    server: {
        plugins: [
            resolve({
                "../auth/mock/user": "../auth/server/user",
            })
        ]
    },
}

export function merge(objs) {
    if (!Array.isArray(objs)) {
        throw "objs must be array";
    }

    var out = objs[0];
    for (var i = 1; i < objs.length; i++) {
        for (var k in objs[i]) {
            if (Array.isArray(objs[i][k])) {
                if (!(k in out)) {
                    out[k] = [];
                }

                out[k] = out[k].concat(objs[i][k]);
            } else {
                out[k] = objs[i][k];
            }
        }
    }

    return out;
}
