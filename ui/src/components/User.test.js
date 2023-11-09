import { describe, beforeEach, after, test } from "node:test";
import assert from "node:assert";
import { DOM } from "../test-helpers/dom.js";
import { waitFor } from "@testing-library/dom";
import fs from "fs";

describe("User component", () => {
    let dom, window, document;

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

    after(async () => {
        await fs.rm("testdata", {
            recursive: true,
            force: true,
        });
    });

    test("signed out shows signed out component", () => {
        let wikiUser = document.createElement("wiki-user");
        document.body.appendChild(wikiUser);

        assert(wikiUser.querySelector("wiki-signed-out-user"));
    });

    test("signed in shows signed in component", async () => {
        window.user.signIn("alice", "somepass");

        let wikiUser = document.createElement("wiki-user");
        document.body.appendChild(wikiUser);

        function assertion() {
            return wikiUser.querySelector("wiki-signed-in-user");
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

    test("sign in and out modifies component", async () => {
        let wikiUser = document.createElement("wiki-user");
        document.body.appendChild(wikiUser);

        function signedOutAssertion() {
            return wikiUser.querySelector("wiki-signed-out-user");
        }
        function signedInAssertion() {
            return wikiUser.querySelector("wiki-signed-in-user");
        }

        await waitFor(
            () => {
                if (!signedOutAssertion()) {
                    throw new Error("waiting");
                }
            },
            { container: document },
        );
        assert(signedOutAssertion());

        window.user.signIn("user1", "passs");

        await waitFor(
            () => {
                if (!signedInAssertion()) {
                    throw new Error("waiting");
                }
            },
            { container: document },
        );
        assert(signedInAssertion());

        window.user.signOut();

        await waitFor(
            () => {
                if (!signedOutAssertion()) {
                    throw new Error("waiting");
                }
            },
            { container: document },
        );
        assert(signedOutAssertion());
    });
});
