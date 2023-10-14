import { describe, beforeEach, afterEach, after, test } from "node:test";
import assert from "node:assert";
import fs from "fs";
import { DOM } from "../test-helpers/dom.js";

describe("router store", () => {
    let dom;
    let window;
    let document;
    let previousConsoleError;

    beforeEach(() => {
        previousConsoleError = console.error;

        dom = new DOM();
        window = dom.window;
        document = dom.window.document;

        dom.addScript(`
            import { route, navigate } from "../src/store/route";

            window.route = route;
            window.navigate = navigate;
        `);
    });

    afterEach(() => {
        console.error = previousConsoleError;
    });

    after(() => {
        fs.rmSync("testdata", {
            recursive: true,
            force: true,
        });
    });

    test("navigate to new place changes location", () => {
        window.route.navigate("/wiki/someSubject");

        assert(getPath(window.location) == "/wiki/someSubject");
    });

    test("navigate to same place does not change location", () => {
        let previousHistory = window.history.length;

        window.route.navigate("/");

        assert(window.history.length == previousHistory);
    });

    test("navigate to bad route changes location to root", () => {
        window.route.navigate("/badpath/");

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

        window.route.set("/wiki/someSubject");

        assert(getPath(window.location));
        assert(window.history.length == previousHistory);
    });
    // test("history pop sets location");
});

function getPath(location) {
    return location.href.substring(location.origin.length);
}
