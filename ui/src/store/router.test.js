import assert from "node:assert";
import { describe, beforeEach, afterEach, test } from "node:test";

import { DOM } from "../test-helpers/dom.js";
import { waitFor } from "../test-helpers/waitFor.js";

describe("router store", () => {
    let dom, window, document, previousConsoleError;

    beforeEach(() => {
        previousConsoleError = console.error;

        dom = new DOM();
        window = dom.window;
        document = dom.window.document;

        dom.addScript(`
            import { router, navigate } from "./src/store/router";

            window.router = router;
            window.navigate = navigate;
        `);
    });

    afterEach(() => {
        console.error = previousConsoleError;
    });

    test("navigate to new place changes location", () => {
        window.router.navigate("/wiki/someSubject");

        assert(getPath(window.location) == "/wiki/someSubject");
    });

    test("navigate to same place does not change location", () => {
        let previousHistory = window.history.length;

        window.router.navigate("/");

        assert(window.history.length == previousHistory);
    });

    test("navigate to bad route changes location to root", () => {
        window.router.navigate("/badpath/");

        assert(getPath(window.location) == "/");
    });

    test("navigate handler checks for href", () => {
        const expectedMessage = "target is missing href";
        let actualMessage;
        window.console.error = function (message) {
            if (message == expectedMessage) {
                actualMessage = message;
            }
            console.error(message);
        };

        let div = document.createElement("div");
        div.onclick = window.navigate;
        div.click();

        assert(actualMessage == expectedMessage);
    });

    test("navigate handler navigates", () => {
        assert(getPath(window.location) == "/");

        let anchor = document.createElement("a");
        anchor.setAttribute("href", "/wiki/someSubject");
        anchor.onclick = window.navigate;

        anchor.click();

        assert(getPath(window.location) == "/wiki/someSubject");
    });

    test("setting route does not change history", () => {
        assert(getPath(window.location) == "/");

        let previousHistory = window.history.length;

        window.router.set("/wiki/someSubject");

        assert(getPath(window.location));
        assert(window.history.length == previousHistory);
    });

    test("history pop sets location", async () => {
        window.router.navigate("/wiki/someSubject");
        assert(window.router.value.path == "/wiki/someSubject");

        window.history.back();

        await waitFor(() => {
            return window.router.value.path == "/";
        }, document);
    });
});

function getPath(location) {
    return location.href.substring(location.origin.length);
}
