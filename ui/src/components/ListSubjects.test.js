import assert from "node:assert";
import { describe, beforeEach, test } from "node:test";

import { DOM } from "../test-helpers/dom.js";
import { waitFor } from "../test-helpers/waitFor.js";

describe("List Subjects component", () => {
    let dom, window, document;

    const columns = 3;

    beforeEach(() => {
        dom = new DOM();
        window = dom.window;
        document = dom.window.document;

        dom.addScript(`
            import { subjects, signal } from "./src/store/subjects";
            import "./src/components/ListSubjects";

            window.subjects = subjects;
            window.subjectsSignal = signal;
        `);
    });

    test("creating element updates subjects", async () => {
        let eventFired = false;
        window.addEventListener("wiki:signal-" + window.subjectsSignal, () => {
            eventFired = true;
        });

        let wikiListSubjects = document.createElement("wiki-list-subjects");
        document.body.appendChild(wikiListSubjects);

        await waitFor(() => eventFired, document);
    });

    test("shows all subjects in links", async () => {
        let eventFired = false;
        window.addEventListener("wiki:signal-" + window.subjectsSignal, () => {
            eventFired = true;
        });

        let wikiListSubjects = document.createElement("wiki-list-subjects");
        document.body.appendChild(wikiListSubjects);

        await waitFor(() => eventFired, document);

        assert(
            wikiListSubjects.querySelector("table").querySelectorAll("a")
                .length == window.subjects.list().length,
        );
    });

    test("subjects are split into n columns", async () => {
        let eventFired = false;
        window.addEventListener("wiki:signal-" + window.subjectsSignal, () => {
            eventFired = true;
        });

        let wikiListSubjects = document.createElement("wiki-list-subjects");
        document.body.appendChild(wikiListSubjects);

        await waitFor(() => eventFired, document);

        let rows = wikiListSubjects.querySelectorAll("tr");
        for (const row of rows) {
            assert(row.querySelectorAll("td").length == columns);
        }
    });

    test("subjects are sorted alphabetically", async () => {
        let eventFired = false;
        window.addEventListener("wiki:signal-" + window.subjectsSignal, () => {
            eventFired = true;
        });

        let wikiListSubjects = document.createElement("wiki-list-subjects");
        document.body.appendChild(wikiListSubjects);

        await waitFor(() => eventFired, document);

        let a = [];
        let rows = wikiListSubjects.querySelectorAll("tr");
        for (let i = 0; i < rows.length; i++) {
            a[i] = [];
            let cols = rows[i].querySelectorAll("td");
            for (let j = 0; j < cols.length; j++) {
                a[i][j] = cols[j].textContent;
            }
        }
        let actual = [];
        for (let j = 0; j < columns; j++) {
            for (let i = 0; i < a.length; i++) {
                let v = a[i][j];
                if (v != "") {
                    actual.push(v);
                }
            }
        }

        assertArraysEqual(window.subjects.list().sort(), actual);
    });
});

function assertArraysEqual(a1, a2) {
    assert(a1.length == a2.length);
    for (let i = 0; i < a1.length; i++) {
        assert(a1[i] == a2[i]);
    }
}
