import { describe, beforeEach, after, test } from "node:test";
import assert from "node:assert";
import fs from "fs";
import { waitFor } from "../test-helpers/waitFor.js";
import { DOM } from "../test-helpers/dom.js";

describe("Subject component", () => {
    let dom, window, document;

    beforeEach(() => {
        dom = new DOM();
        window = dom.window;
        document = dom.window.document;

        dom.addScript(`
            import { signal as routerSignal } from "../src/store/router";
            import {
                subjects,
                signal as subjectsSignal
            } from "../src/store/subjects";
            import { user } from "../src/store/user";
            import "../src/components/Subject";

            window.routerSignal = routerSignal;
            window.subjects = subjects;
            window.subjectsSignal = subjectsSignal;
            window.user = user;
        `);
    });

    after(async () => {
        await fs.rm("testdata", {
            recursive: true,
            force: true,
        });
    });

    test("subject inits to view and switches to edit and back to view", async () => {
        window.customElements.define(
            "wiki-edit-subject",
            class MockEditSubject extends window.HTMLElement {
                constructor() {
                    super();
                }

                getValue() {
                    return "";
                }
            },
        );

        window.customElements.define(
            "wiki-view-subject",
            class MockViewSubject extends window.HTMLElement {
                constructor() {
                    super();
                }

                render() {}
            },
        );

        let subjectName = "Pro in antistite ferinos";
        let eventFired = false;
        window.addEventListener("wiki:signal-" + window.subjectsSignal, () => {
            eventFired = true;
        });

        let wikiSubject = document.createElement("wiki-subject");
        wikiSubject.setAttribute("subj", encodeURIComponent(subjectName));
        document.body.appendChild(wikiSubject);

        await waitFor(() => {
            return eventFired;
        }, document);

        let buttons = wikiSubject.querySelectorAll("button");
        let viewButton = buttons[0];
        let editButton = buttons[1];
        editButton.removeAttribute("disabled");

        let viewTab = wikiSubject.querySelector("#view");
        let editTab = wikiSubject.querySelector("#edit");

        function editDisplayedAssertion() {
            return (
                editTab.style.display == "inline" &&
                viewTab.style.display == "none"
            );
        }

        function viewDisplayedAssertion() {
            return (
                editTab.style.display == "none" &&
                viewTab.style.display == "inline"
            );
        }

        assert(viewDisplayedAssertion());

        editButton.click();

        assert(editDisplayedAssertion());

        viewButton.click();

        assert(viewDisplayedAssertion());
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

        await waitFor(() => {
            return eventFired;
        }, document);

        assert(document.title.includes(subjectName));
    });

    test("content not found shows error", async () => {
        let subjectName = "not a real subject";

        let wikiSubject = document.createElement("wiki-subject");
        wikiSubject.setAttribute("subj", encodeURIComponent(subjectName));
        document.body.appendChild(wikiSubject);

        await waitFor(() => {
            let p = wikiSubject.querySelector("p");
            return p && p.textContent.includes("not found");
        }, document);
    });

    test("shows fetching before content available", async () => {
        let subjectName = "Pro in antistite ferinos";

        let wikiSubject = document.createElement("wiki-subject");
        wikiSubject.setAttribute("subj", encodeURIComponent(subjectName));
        document.body.appendChild(wikiSubject);

        assert(wikiSubject.textContent.includes("Fetching"));

        await waitFor(() => {
            return !wikiSubject.querySelector("p");
        }, document);
    });

    test("new subject shows editor", () => {
        let wikiSubject = document.createElement("wiki-subject");
        wikiSubject.setAttribute("new", null);
        document.body.appendChild(wikiSubject);

        assert(wikiSubject.querySelector("#edit").style.display == "inline");
    });

    test("saving new subject adds to store", () => {
        let subjectName = "brand new subject";
        window.customElements.define(
            "wiki-edit-subject",
            class MockEditSubject extends window.HTMLElement {
                constructor() {
                    super();
                }

                getValue() {
                    return "# " + subjectName;
                }

                getTitle() {
                    return subjectName;
                }
            },
        );

        let wikiSubject = document.createElement("wiki-subject");
        wikiSubject.setAttribute("new", null);
        document.body.appendChild(wikiSubject);

        let saveButton = wikiSubject.querySelectorAll("button")[2];
        saveButton.removeAttribute("disabled");
        saveButton.click();

        let wikiEditSubject = document.querySelector("wiki-edit-subject");
        assert(
            window.subjects.get(subjectName).content ==
                wikiEditSubject.getValue(),
        );
    });

    test("new named subject shows editor and title", async () => {
        let subjectName = "a new subject";
        let wikiSubject = document.createElement("wiki-subject");
        wikiSubject.setAttribute("new", null);
        wikiSubject.setAttribute("subj", encodeURIComponent(subjectName));
        document.body.appendChild(wikiSubject);

        await waitFor(() => {
            return wikiSubject.querySelector("#edit");
        }, document);

        assert(wikiSubject.querySelector("#edit").style.display == "inline");
        assert(document.title.includes(subjectName));
    });

    test("creating new named subject that exists redirects to existing subject", async () => {
        let subjectName = "Pro in antistite ferinos";
        let wikiSubject = document.createElement("wiki-subject");
        wikiSubject.setAttribute("new", null);
        wikiSubject.setAttribute("subj", encodeURIComponent(subjectName));
        document.body.appendChild(wikiSubject);

        let subjectEventFired = false;
        window.addEventListener("wiki:signal-" + window.subjectsSignal, () => {
            subjectEventFired = true;
        });

        let routerEventFired = false;
        window.addEventListener("reef:signal-" + window.routerSignal, () => {
            routerEventFired = true;
        });
        await waitFor(() => {
            return subjectEventFired;
        }, document);

        await waitFor(() => {
            return routerEventFired;
        }, document);

        assert(
            window.location.href.substring(window.location.origin.length) ==
                "/wiki/" + encodeURIComponent(subjectName),
        );
    });
});
