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

    // test("updateList emits event on success", async () => {
    //     let eventFired = false;
    //     document.addEventListener("reef:store-" + window.subjectsSignal, () => {
    //         eventFired = true;
    //     });

    //     window.subjects.updateList();

    //     await waitFor(
    //         () => {
    //             if (!eventFired) {
    //                 throw new Error("waiting");
    //             }
    //         },
    //         { container: document },
    //     );
    //     assert(eventFired);
    // });

    // test("list returns empty array", async () => {
    //     assert(window.subjects.list().length == 0);
    // });

    test("list after update returns array", async () => {
        let eventFired = false;
        document.addEventListener("reef:store-" + window.subjectsSignal, () => {
            eventFired = true;
        });

        window.subjects.updateList();

        // await waitFor(
        //     () => {
        //         if (!eventFired) {
        //             throw new Error("waiting");
        //         }
        //     },
        //     { container: document },
        // );
        // assert(eventFired);

        assert(window.subjects.list().length > 0);
    });
});
