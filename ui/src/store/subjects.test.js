import { describe, beforeEach, after, test } from "node:test";
import assert from "node:assert";
import { waitFor } from "@testing-library/dom";
import fs from "fs";
import { DOM } from "../test-helpers/dom.js";

describe("subjects store", () => {
    let dom;
    let window;
    let document;

    beforeEach(() => {
        dom = new DOM();
        window = dom.window;
        document = dom.window.document;

        dom.addScript(`
            import { subjects, signal } from "../src/store/subjects";

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

    test("update list emits event", async () => {
        let eventFired = false;
        window.addEventListener("reef:signal-" + window.subjectsSignal, () => {
            eventFired = true;
        });

        window.subjects.updateList();

        function assertion() {
            return eventFired;
        }
        await waitFor(() => {
            if (!assertion()) {
                throw new Error("waiting");
            }
        }, { container: document });
        assert(assertion());
    });

    test("list after update returns array", async () => {
        assert(window.subjects.list().length == 0);

        window.subjects.updateList();

        function assertion() {
            return window.subjects.list().length > 0;
        }
        await waitFor(() => {
            if (!assertion()) {
                throw new Error("waiting");
            }
        }, { container: document });
        assert(assertion());
    });

    test("update content emits event", async () => {
        let eventFired = false;
        window.addEventListener("reef:signal-" + window.subjectsSignal, () => {
            eventFired = true;
        });

        window.subjects.updateContent("one");

        function assertion() {
            return eventFired;
        }
        await waitFor(() => {
            if (!assertion()) {
                throw new Error("waiting");
            }
        }, { container: document });
        assert(assertion());
    });

    test("content after update returns non-empty string", async () => {
        assert(window.subjects.content("one").length == 0);

        window.subjects.updateContent("one");

        function assertion() {
            return window.subjects.content("one").length > 0;
        }
        await waitFor(() => {
            if (!assertion()) {
                throw new Error("waiting");
            }
        }, { container: document });
        assert(assertion());
    });
});
