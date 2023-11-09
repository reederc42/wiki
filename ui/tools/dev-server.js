/*eslint-env node */

import express from "express";
import yargs from "yargs";
import { hideBin } from "yargs/helpers";
import { build } from "./build.js";
import { prod as prodBuild, dev as devBuild } from "./build-options.js";
import os from "os";

const argv = yargs(hideBin(process.argv)).argv;

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

const app = express();
let port = 8080;
if (argv.port) {
    port = argv.port;
}

if (argv.build == "prod") {
    await build(prodBuild, "prod");
} else {
    await build(devBuild, "dev");
}

let dist = process.cwd() + "/dist";

app.use(express.static(dist));

function sendIndex(_, res) {
    res.sendFile(`${dist}/index.html`);
}

app.get("/wiki/*", sendIndex);
app.get("/wiki-new", sendIndex);
app.get("/wiki-new/*", sendIndex);

let listener = app.listen(port).address();
let ifaces = ["lo0", "en0", "lo", "eth0"];
if (argv.interface) {
    if (argv.interface instanceof Array) {
        ifaces.concat(argv.interface);
    } else {
        ifaces.push(argv.interface);
    }
}
let addresses = [];
for (const i of ifaces) {
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
            `${a.interface} listening at http://${a.address}:${listener.port}/`,
        );
    }
} else {
    console.log(`listening at http://localhost:${listener.port}/`);
    console.log("could not find network addresses");
}
