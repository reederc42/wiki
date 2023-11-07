import { describe, beforeEach, after, test } from "node:test";
import assert from "node:assert";
import { waitFor } from "@testing-library/dom";
import fs from "fs";
import { DOM } from "../test-helpers/dom.js";

describe("subjects store", () => {
    let dom, window, document;

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
        window.addEventListener("wiki:signal-" + window.subjectsSignal, () => {
            eventFired = true;
        });

        window.subjects.updateList();

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

    test("list after update returns array", async () => {
        assert(window.subjects.list().length == 0);

        window.subjects.updateList();

        function assertion() {
            return window.subjects.list().length > 0;
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

    test("update content emits event", async () => {
        let eventFired = false;
        window.addEventListener("wiki:signal-" + window.subjectsSignal, () => {
            eventFired = true;
        });

        window.subjects.fetchContent("Pro in antistite ferinos");

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

    test("content after update returns non-empty string", async () => {
        let subjectName = "Pro in antistite ferinos";
        assert(window.subjects.get(subjectName) === undefined);

        window.subjects.fetchContent(subjectName);

        function assertion() {
            return window.subjects.get(subjectName).content.length > 0;
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

    test("not found content rejects promise", async () => {
        let subjectName = "not a subject";

        let err = new Error();
        window.subjects.fetchContent(subjectName).catch((e) => {
            err = e;
        });

        function assertion() {
            return err.message.includes("not found");
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

    test("get returns mutable reference", async () => {
        let subjectName = "Pro in antistite ferinos";

        window.subjects.fetchContent(subjectName);

        function assertion() {
            return window.subjects.get(subjectName).content.length > 0;
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
            window.subjects.get(subjectName) ===
                window.subjects.get(subjectName),
        );

        let subject = window.subjects.get(subjectName);
        subject.content = "some test value";

        assert(subject.content == window.subjects.get(subjectName).content);
    });

    test("pushing nonexistent subject fails", async () => {
        let err;
        window.subjects.pushContent("some subject").catch((e) => {
            err = e;
        });

        function assertion() {
            return err !== undefined;
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

    test("pushing new content sets synced", async () => {
        let subjectName = "Pro in antistite ferinos";
        let eventFired = false;
        window.addEventListener("wiki:signal-" + window.subjectsSignal, () => {
            eventFired = true;
        });

        window.subjects.fetchContent(subjectName);

        function assertion1() {
            return eventFired;
        }
        await waitFor(
            () => {
                if (!assertion1()) {
                    throw new Error("waiting");
                }
            },
            { container: document },
        );
        assert(assertion1());

        let success = false;
        window.subjects.pushContent(subjectName).then(() => {
            success = true;
        });

        function assertion2() {
            return success;
        }
        await waitFor(
            () => {
                if (!assertion2()) {
                    throw new Error("waiting");
                }
            },
            { container: document },
        );
        assert(assertion2());
    });
});
