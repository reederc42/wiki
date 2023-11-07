import { describe, beforeEach, after, test } from "node:test";
import assert from "node:assert";
import fs from "fs";
import { waitFor } from "@testing-library/dom";
import { DOM } from "../test-helpers/dom.js";

describe("Subject component", () => {
    let dom, window, document;

    beforeEach(() => {
        dom = new DOM();
        window = dom.window;
        document = dom.window.document;

        dom.addScript(`
            import { subjects, signal } from "../src/store/subjects";
            import "../src/components/Subject";

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

    test("subject inits to view and switches to edit and back to view", async () => {
        let subjectName = "Pro in antistite ferinos";
        let eventFired = false;
        window.addEventListener("wiki:signal-" + window.subjectsSignal, () => {
            eventFired = true;
        });
        let wikiSubject = document.createElement("wiki-subject");
        wikiSubject.setAttribute("subj", encodeURIComponent(subjectName));
        document.body.appendChild(wikiSubject);

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

        let buttons = wikiSubject.querySelectorAll("button");
        let viewButton = buttons[0];
        let editButton = buttons[1];

        let viewTab = wikiSubject.querySelector("#view");
        let editTab = wikiSubject.querySelector("#edit");

        assert(window.getComputedStyle(editTab).display == "none");

        editButton.click();

        assert(window.getComputedStyle(viewTab).display == "none");

        viewButton.click();

        assert(window.getComputedStyle(editTab).display == "none");
    });

    test("title contains subject name", async () => {
        let subjectName = "Pro in antistite ferinos";
        let eventFired = false;
        window.addEventListener("wiki:signal-" + window.subjectsSignal, () => {
            eventFired = true;
        });

        let wikiSubject = document.createElement("wiki-subject");
        wikiSubject.setAttribute("subj", encodeURIComponent(subjectName));
        document.body.appendChild(wikiSubject);

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

        assert(document.title.includes(subjectName));
    });

    test("content not found shows error", async () => {
        let subjectName = "not a real subject";

        let wikiSubject = document.createElement("wiki-subject");
        wikiSubject.setAttribute("subj", encodeURIComponent(subjectName));
        document.body.appendChild(wikiSubject);

        function assertion() {
            let p = wikiSubject.querySelector("p");
            return p && p.textContent.includes("not found");
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
    });

    test("shows fetching before content available", async () => {
        let subjectName = "Pro in antistite ferinos";

        let wikiSubject = document.createElement("wiki-subject");
        wikiSubject.setAttribute("subj", encodeURIComponent(subjectName));
        document.body.appendChild(wikiSubject);

        assert(wikiSubject.textContent.includes("Fetching"));

        function assertion() {
            return !wikiSubject.querySelector("p");
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
    });
});
