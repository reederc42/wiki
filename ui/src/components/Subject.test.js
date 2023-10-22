import { describe, beforeEach, after, test } from "node:test";
import assert from "node:assert";
import fs from "fs";
import { DOM } from "../test-helpers/dom.js";

describe("Subject component", () => {
    let dom;
    let window;
    let document;

    beforeEach(() => {
        dom = new DOM();
        window = dom.window;
        document = dom.window.document;

        dom.addScript(`
            import "../src/components/Subject";
        `);
    });

    after(async () => {
        await fs.rm("testdata", {
            recursive: true,
            force: true,
        });
    });

    test("subject inits to view and switches to edit and back to view", () => {
        let wikiSubject = document.createElement("wiki-subject");
        document.body.appendChild(wikiSubject);

        let buttons = wikiSubject.querySelectorAll("button");
        let viewButton = buttons[0];
        let editButton = buttons[1];

        let viewDiv = wikiSubject.querySelector("#view");
        let editDiv = wikiSubject.querySelector("#edit");

        assert(window.getComputedStyle(editDiv).display == "none");

        editButton.click();

        assert(window.getComputedStyle(viewDiv).display == "none");

        viewButton.click();

        assert(window.getComputedStyle(editDiv).display == "none");
    });
});