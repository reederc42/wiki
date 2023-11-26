import { describe, beforeEach, test } from "node:test";
import assert from "node:assert";
import { waitFor } from "../test-helpers/waitFor.js";
import { DOM } from "../test-helpers/dom.js";

describe("subjects store", () => {
    let dom, window, document;

    beforeEach(() => {
        dom = new DOM();
        window = dom.window;
        document = dom.window.document;

        dom.addScript(`
            import { subjects, signal, Subject } from "./src/store/subjects";
            import { user } from "./src/store/user";

            window.subjects = subjects;
            window.subjectsSignal = signal;
            window.Subject = Subject;

            window.user = user;
        `);
    });

    test("update list emits event", async () => {
        let eventFired = false;
        window.addEventListener("wiki:signal-" + window.subjectsSignal, () => {
            eventFired = true;
        });

        window.subjects.updateList();

        await waitFor(() => eventFired, document);
    });

    test("list after update returns array", async () => {
        assert(window.subjects.list().length == 0);

        window.subjects.updateList();

        await waitFor(() => window.subjects.list().length > 0, document);
    });

    test("update content emits event", async () => {
        let eventFired = false;
        window.addEventListener("wiki:signal-" + window.subjectsSignal, () => {
            eventFired = true;
        });

        window.subjects.fetchContent("Pro in antistite ferinos");

        await waitFor(() => eventFired, document);
    });

    test("content after update returns non-empty string", async () => {
        let subjectName = "Pro in antistite ferinos";
        assert(window.subjects.get(subjectName) === undefined);

        window.subjects.fetchContent(subjectName);

        await waitFor(() => window.subjects.get(subjectName).content.length > 0, document);
    });

    test("not found content rejects promise", async () => {
        let subjectName = "not a subject";

        let err = new Error();
        window.subjects.fetchContent(subjectName).catch((e) => {
            err = e;
        });

        await waitFor(() => err.message.includes("not found"), document);
    });

    test("get returns mutable reference", async () => {
        let subjectName = "Pro in antistite ferinos";

        window.subjects.fetchContent(subjectName);

        await waitFor(() => window.subjects.get(subjectName).content.length > 0, document);

        assert(
            window.subjects.get(subjectName) ===
                window.subjects.get(subjectName),
        );

        let subject = window.subjects.get(subjectName);
        subject.content = "some test value";

        assert(subject.content == window.subjects.get(subjectName).content);
    });

    test("pushing nonexistent subject fails", async () => {
        let signedIn = false;
        window.user.signIn("bob", "bobpass").then(() => {
            signedIn = true;
        })
        await waitFor(() => signedIn, document);

        let err;
        window.subjects.pushContent("some subject").catch((e) => {
            err = e;
        });

        await waitFor(() => err !== undefined, document);
    });

    test("pushing new content sets synced", async () => {
        let subjectName = "Pro in antistite ferinos";
        let eventFired = false;
        window.addEventListener("wiki:signal-" + window.subjectsSignal, () => {
            eventFired = true;
        });

        window.subjects.fetchContent(subjectName);

        await waitFor(() => eventFired, document);

        let signedIn = false;
        window.user.signIn("bob", "bobpass").then(() => {
            signedIn = true;
        })
        await waitFor(() => signedIn, document);

        let success = false;
        window.subjects.pushContent(subjectName).then(() => {
            success = true;
        });

        await waitFor(() => success, document);
    });

    test("creating subject that already exists fails", async () => {
        let subjectName = "Pro in antistite ferinos";

        let eventFired = false;
        window.addEventListener("wiki:signal-" + window.subjectsSignal, () => {
            eventFired = true;
        });

        window.subjects.updateList();

        await waitFor(() => eventFired, document);

        assert(
            window.subjects
                .create(subjectName, new window.Subject())
                .message.includes("already exists"),
        );
    });

    test("creating new subject allows retrieval", async () => {
        let subjectName = "brand new subject";
        let newSubject = new window.Subject();

        let err = window.subjects.create(subjectName, newSubject);
        assert(err == null);

        assert(window.subjects.get(subjectName) === newSubject);
    });
});
