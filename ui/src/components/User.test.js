import test from "node:test";
import assert from "node:assert";
import {JSDOM} from "jsdom";
import * as fs from "fs";
import {build} from "../../tools/build.js";
import {waitFor} from "@testing-library/dom";

test("test the thing", async () => {
    let outfile = `testdata/User-${crypto.randomUUID()}.js`;
    await build({
        entryPoints: ['./src/components/User.js'],
        bundle: true,
        outfile: outfile
    }, "test");
    const User = fs.readFileSync(outfile);
    let dom = new JSDOM(``, {runScripts: "dangerously"});
    let window = dom.window;
    window.addEventListener("log", (event) => {
        console.log(event);
    });
    window.addEventListener("info", (event) => {
        console.info(event);
    });
    window.addEventListener("error", (event) => {
        console.error(event);
    });
    window.requestAnimationFrame = (timestamp) => {};

    let userScript = window.document.createElement("script");
    userScript.textContent = User;
    window.document.body.appendChild(userScript);

    let wikiUser = window.document.createElement("wiki-user");
    window.document.body.appendChild(wikiUser);

    await waitFor(() => {
        assert(wikiUser.querySelector("wiki-signed-out-user"))
    }, {container: window.document});
});
