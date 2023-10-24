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
            import "../src/components/ViewSubject";

            window.subjects = subjects;
            window.subjectsSignal = signal;
        `);
    });

    after(async () => {
        await fs.rm("testdata", {
            recursive: true,
            force: true,
        });
    });

    test("content is not empty after update", async () => {
        let subjectName = "Pro in antistite ferinos";
        let eventFired = false;
        window.addEventListener("reef:signal-" + window.subjectsSignal, () => {
            eventFired = true;
        });

        window.subjects.updateContent(subjectName);

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

        let wikiViewSubject = document.createElement("wiki-view-subject");
        wikiViewSubject.setAttribute("subj", encodeURIComponent(subjectName));
        document.body.appendChild(wikiViewSubject);

        // "not empty" means a top-level header with subject name
        console.log(wikiViewSubject.querySelector("h1").textContent);
        assert(wikiViewSubject.querySelector("h1").textContent == subjectName);
    });

    test("content not found shows error", async () => {
        let subjectName = "not a real subject";
        let eventFired = false;
        window.addEventListener("reef:signal-" + window.subjectsSignal, () => {
            eventFired = true;
        });

        window.subjects.updateContent(subjectName);

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

        let wikiViewSubject = document.createElement("wiki-view-subject");
        wikiViewSubject.setAttribute("subj", encodeURIComponent(subjectName));
        document.body.appendChild(wikiViewSubject);

        assert(wikiViewSubject.textContent.includes("Error"));
    });
});
