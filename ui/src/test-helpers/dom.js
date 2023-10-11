import { JSDOM } from "jsdom";
import * as fs from "fs";
import { buildSync } from "../../tools/build.js";

export class DOM {
    constructor() {
        this.dom = new JSDOM(``, {runScripts: "dangerously"});
        this.window = this.dom.window;

        this.window.addEventListener("log", (message) => {
            console.log(message);
        });
        this.window.addEventListener("info", (message) => {
            console.info(message);
        });
        this.window.addEventListener("error", (message) => {
            console.error(message);
        });

        this.window.requestAnimationFrame = (cb) => {cb()};

        this.addScript("./src/test-helpers/core-js.js");
    }

    addScript(src) {
        let options = {};
        if (fs.statSync(src)) {
            let base = src.split("/").pop().split(".")[0];
            options.entryPoints = [src];
            options.outfile = `testdata/${base}-${crypto.randomUUID()}.js`;
        }
        buildSync(options, "test");
        let script = this.window.document.createElement("script");
        script.textContent = fs.readFileSync(outfile);
        this.window.document.body.appendChild(script);

        return outfile;
    }
}
