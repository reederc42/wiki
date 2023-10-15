import { describe, beforeEach, after, test } from "node:test";
import assert from "node:assert";
import { DOM } from "../test-helpers/dom.js";
import fs from "fs";

describe("Router component", () => {
    let dom;
    let document;

    beforeEach(() => {
        dom = new DOM();
        document = dom.window.document;

        dom.addScript(`
            import "../src/components/Router";
            import { route } from "../src/store/route";

            window.route = route;
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

    test("path not found redirects to root");

    test("wiki/ shows subject");
});
