import { describe, beforeEach, after, test } from "node:test";
import assert from "node:assert";
import { waitFor } from "@testing-library/dom";
import fs from "fs";
import { DOM } from "../test-helpers/dom.js";

describe("List Subjects component", () => {
    let dom, window, document;

    beforeEach(() => {
        dom = new DOM();
        window = dom.window;
        document = dom.window.document;

        dom.addScript(`
            import { subjects, signal } from "../src/store/subjects";
            import "../src/components/ListSubjects";

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

    test("creating element updates subjects", async () => {
        let eventFired = false;
        window.addEventListener("reef:signal-" + window.subjectsSignal, () => {
            eventFired = true;
        });

        let wikiListSubjects = document.createElement("wiki-list-subjects");
        document.body.appendChild(wikiListSubjects);

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
    });

    test("shows all subjects in links", async () => {
        let eventFired = false;
        window.addEventListener("reef:signal-" + window.subjectsSignal, () => {
            eventFired = true;
        });

        let wikiListSubjects = document.createElement("wiki-list-subjects");
        document.body.appendChild(wikiListSubjects);

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

        assert(
            wikiListSubjects.querySelectorAll("a").length ==
                window.subjects.list().length,
        );
    });

    test("subjects are split into n columns", async () => {
        const columns = 3;
        let eventFired = false;
        window.addEventListener("reef:signal-" + window.subjectsSignal, () => {
            eventFired = true;
        });

        let wikiListSubjects = document.createElement("wiki-list-subjects");
        document.body.appendChild(wikiListSubjects);

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

        let rows = wikiListSubjects.querySelectorAll("tr");
        for (const row of rows) {
            assert(row.querySelectorAll("td").length == columns);
        }
    });
});
