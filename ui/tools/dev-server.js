/*eslint-env node */

import express from "express";
import { build } from "./build.js";
import os from "os";

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
const port = 8080;

await build(
    {
        sourcemap: true,
    },
    "dev",
);

let dist = process.cwd() + "/dist";

app.use(express.static(dist));

app.get("/wiki/*", (_, res) => {
    res.sendFile(`${dist}/index.html`);
});

let listener = app.listen(port).address();
let ifaces = ["lo0", "en0", "lo", "eth0"];
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
