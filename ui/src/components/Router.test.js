import { describe, beforeEach, afterEach, test } from "node:test";
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
        `);
    });

    test("")
});
