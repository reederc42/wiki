import { describe, beforeEach, after, test } from "node:test";
import assert from "node:assert";
import { waitFor } from "@testing-library/dom";
import fs from "fs";
import { DOM } from "../test-helpers/dom.js";

describe("View Subject component", () => {
    let dom, window, document;

    beforeEach(() => {
        dom = new DOM();
        window = dom.window;
        document = dom.window.document;

        dom.addScript(`
            import { subjects, signal } from "../src/store/subjects";
            import { inject } from "../src/store/inject";
            import "../src/components/ViewSubject";

            window.subjects = subjects;
            window.subjectsSignal = signal;
            window.inject = inject;
        `);
    });

    after(async () => {
        await fs.rm("testdata", {
            recursive: true,
            force: true,
        });
    });

    test("content is not empty if subject not empty", async () => {
        let subjectName = "Pro in antistite ferinos";
        let eventFired = false;
        window.addEventListener("wiki:signal-" + window.subjectsSignal, () => {
            eventFired = true;
        });

        window.subjects.fetchContent(subjectName);

        function assertion() {
            return eventFired;
        }
        await waitFor(
            () => {
                if (!assertion()) {
                    throw new Error("waiting");
                }
            },
            { container: document },
        );
        assert(assertion());

        let subject = window.subjects.get(subjectName);
        window.inject("viewer", subject);

        let wikiViewSubject = document.createElement("wiki-view-subject");
        wikiViewSubject.id = "viewer";
        document.body.appendChild(wikiViewSubject);

        // "not empty" means a top-level header with subject name
        assert(wikiViewSubject.querySelector("h1").textContent == subjectName);
    });
});
