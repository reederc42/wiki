/* eslint-env node */

import os from "node:os";

import express from "express";
import yargs from "yargs";
import { hideBin } from "yargs/helpers";

import { build } from "./build.js";
import { options } from "./build-options.js";
import { configure } from "./configure.js";
import { isMain } from "./is-main.js";

function getAddress(iface = "", family = "IPv4") {
    let i = os.networkInterfaces()[iface];
    if (i) {
        for (const j of i) {
            if (j.family == family) {
                return j.address;
            }
        }
    }
    return "";
}

async function main() {
    const argv = yargs(hideBin(process.argv))
        .default("build", "dev")
        .default("api", "mock")
        .default("port", 8080)
        .default("interface", ["lo0", "en0", "lo", "eth0"]).argv;

    configure(argv, argv.api);

    await build(
        {
            ...options[argv.build],
            ...options[argv.api],
        },
        argv.build,
    );

    const app = express();
    const dist = process.cwd() + "/dist";
    app.use(express.static(dist));

    function sendIndex(_, res) {
        res.sendFile(`${dist}/index.html`);
    }
    app.get("/*", sendIndex);

    const listener = app.listen(argv.port).address();
    if (listener == null) {
        throw new Error(`could not listen on ${argv.port}`);
    }

    let addresses = [];
    let interfaces =
        argv.interface instanceof Array ? argv.interface : [argv.interface];
    for (const i of interfaces) {
        let addr = getAddress(i);
        if (addr != "") {
            addresses.push({
                interface: i,
                address: addr,
            });
        }
    }
    if (addresses.length > 0) {
        for (const a of addresses) {
            console.log(
                `${a.interface} listening at ` +
                    `http://${a.address}:${listener.port}/`,
            );
        }
    } else {
        console.log(`listening at http://localhost:${listener.port}/`);
        console.log("could not find network addresses");
    }
}

if (isMain(import.meta.url)) await main();
