import { describe, beforeEach, afterEach, test } from "node:test";
import assert from "node:assert";
import { DOM } from "../test-helpers/dom.js";
import fs from "fs";

describe("User component", () => {
    let dom;
    let window;
    let document;

    beforeEach(() => {
        dom = new DOM();
        window = dom.window;
        document = dom.window.document;

        dom.addScript(`
            import "../src/components/User";
            import { user } from "../src/store/user";

            window.user = user;
        `);
    });

    test("signed out shows signed out component", () => {
        let wikiUser = document.createElement("wiki-user");
        document.body.appendChild(wikiUser);

        assert(wikiUser.querySelector("wiki-signed-out-user"));
    });

    test("signed in shows signed in component", () => {
        window.user.signIn("user1");

        let wikiUser = document.createElement("wiki-user");
        document.body.appendChild(wikiUser);

        assert(wikiUser.querySelector("wiki-signed-in-user"));
    });

    test("sign in and out modifies component", () => {
        let wikiUser = document.createElement("wiki-user");
        document.body.appendChild(wikiUser);

        assert(wikiUser.querySelector("wiki-signed-out-user"));

        window.user.signIn("user1");

        assert(wikiUser.querySelector("wiki-signed-in-user"));

        window.user.signOut();

        assert(wikiUser.querySelector("wiki-signed-out-user"));
    });

    afterEach(() => {
        fs.rmSync("testdata", {
            recursive: true,
            force: true,
        });
    });
});
