import express from 'express';
import {build} from './build.js';
import * as os from 'os';
import * as fs from 'fs';

function getAddress(iface='', family='IPv4') {
    let i = os.networkInterfaces()[iface]
    if (i) {
        for (const j of i) {
            if (j.family == family) {
                return j.address;
            }
        }
    }
    return '';
}

const app = express();
const port = 8080;

await build({
    sourcemap: true
}, 'dev');

let dist = process.cwd() + "/dist";

let distFiles = fs.readdirSync(dist);
for (const d of distFiles) {
    app.get(`/${d}`, (_, res) => {
        res.sendFile(`${dist}/${d}`);
    })
}

app.get('/*', (_, res) => {
    res.sendFile(`${dist}/index.html`);
})

let listener = app.listen(port).address();
let ifaces = ['lo0', 'en0'];
let addresses = [];
for (const i of ifaces) {
    let addr = getAddress(i);
    addresses.push(addr);
}
for (const a of addresses) {
    console.log(`listening at http://${a}:${listener.port}/`);
}
