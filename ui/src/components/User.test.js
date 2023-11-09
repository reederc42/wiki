import { describe, beforeEach, after, test } from "node:test";
import assert from "node:assert";
import { DOM } from "../test-helpers/dom.js";
import { waitFor } from "../test-helpers/waitFor.js";
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

        await waitFor(() => {
            return wikiUser.querySelector("wiki-signed-in-user");
        }, document);
    });

    test("signed up shows signed in component", async () => {
        window.user.signUp("testUser", "goodpass");

        let wikiUser = document.createElement("wiki-user");
        document.body.appendChild(wikiUser);

        await waitFor(() => {
            return wikiUser.querySelector("wiki-signed-in-user");
        }, document);
    });

    test("sign in and up with invalid password fails", async () => {
        let done = false;
        window.user.signIn("testUser", "badpass").catch(() => {
            done = true;
        });

        let wikiUser = document.createElement("wiki-user");
        document.body.appendChild(wikiUser);

        await waitFor(() => {
            return done;
        }, document);
        assert(!wikiUser.querySelector("wiki-signed-in-user"));

        done = false;
        window.user.signUp("testUser", "badpass").catch(() => {
            done = true;
        });
        await waitFor(() => {
            return done;
        }, document);

        assert(!wikiUser.querySelector("wiki-signed-in-user"));
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

        await waitFor(signedOutAssertion, document);

        window.user.signIn("user1", "passs");

        await waitFor(signedInAssertion, document);

        window.user.signOut();

        await waitFor(signedOutAssertion, document);
    });
});
