import { JSDOM } from "jsdom";
import * as fs from "fs";
import { buildSync } from "../../tools/build.js";

export class DOM {
    constructor() {
        this.dom = new JSDOM(``, { runScripts: "dangerously" });
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

        this.window.requestAnimationFrame = (cb) => {
            cb();
        };

        this.addScript("./src/test-helpers/core-js.js");
    }

    addScript(src) {
        let outfile;
        let entryPoint = src;
        if (fs.existsSync(src)) {
            let base = src.split("/").pop().split(".")[0];
            outfile = `testdata/${base}-${crypto.randomUUID()}.js`;
        } else {
            outfile = `testdata/anon-${crypto.randomUUID()}.js`;
            entryPoint = `testdata/anon-src-${crypto.randomUUID()}.js`;
            fs.mkdirSync("testdata", {
                recursive: true,
            });
            fs.writeFileSync(entryPoint, src);
        }

        buildSync(
            {
                entryPoints: [entryPoint],
                outfile: outfile,
            },
            "test",
        );

        let script = this.window.document.createElement("script");
        script.textContent = fs.readFileSync(outfile);
        this.window.document.body.appendChild(script);

        return outfile;
    }
}
