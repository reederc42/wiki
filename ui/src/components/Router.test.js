import { describe, beforeEach, after, test } from "node:test";
import assert from "node:assert";
import { DOM } from "../test-helpers/dom.js";
import fs from "fs";

describe("Router component", () => {
    let dom;
    let window;
    let document;

    beforeEach(() => {
        dom = new DOM();
        window = dom.window;
        document = dom.window.document;

        dom.addScript(`
            import "../src/components/Router";
            import { router } from "../src/store/router";

            window.router = router;
        `);
    });

    after(async () => {
        await fs.rm("testdata", {
            recursive: true,
            force: true,
        });
    });

    test("root shows list of subjects", () => {
        let wikiRouter = document.createElement("wiki-router");
        document.body.appendChild(wikiRouter);

        assert(wikiRouter.querySelector("p"));
    });

    test("wiki/ shows subject", () => {
        window.router.navigate("/wiki/someSubject");

        let wikiRouter = document.createElement("wiki-router");
        document.body.appendChild(wikiRouter);

        assert(wikiRouter.querySelector("wiki-subject"));
    });

    test("navigate to and from subject changes view", () => {
        let wikiRouter = document.createElement("wiki-router");
        document.body.appendChild(wikiRouter);

        assert(wikiRouter.querySelector("p"));

        window.router.navigate("/wiki/someSubject");

        assert(wikiRouter.querySelector("wiki-subject"));

        window.router.navigate("/badpath/");

        assert(wikiRouter.querySelector("p"));
    });
});
