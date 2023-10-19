import { JSDOM } from "jsdom";
import fs from "fs";
import { buildSync } from "./build.js";

export class DOM {
    constructor(url = "http://localhost/") {
        this.dom = new JSDOM(``, {
            runScripts: "dangerously",
            url: url,
        });
        this.window = this.dom.window;

        this.window.requestAnimationFrame = (cb) => {
            cb();
        };

        this.addScript(`import "core-js";`);
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
