import { JSDOM } from "jsdom";

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
        let buildOptions = {
            stdin: {
                contents: src,
                resolveDir: ".",
            },
            write: false,
        };
        let result = buildSync(buildOptions);

        let script = this.window.document.createElement("script");
        script.textContent = new TextDecoder().decode(
            result.outputFiles[0].contents,
        );
        this.window.document.head.appendChild(script);
    }
}
