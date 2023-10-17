import { describe, beforeEach, after, test } from "node:test";
import assert from "node:assert";
import fs from "fs";
import { DOM } from "../test-helpers/dom.js";

describe("user store", () => {
    let dom;
    let window;

    beforeEach(() => {
        dom = new DOM();
        window = dom.window;

        dom.addScript(`
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

    test("sign out before signed in", () => {
        window.user.signOut();

        assert(window.user.value.username == "");
    });

    test("sign in and out", () => {
        window.user.signIn("testuser");

        assert(window.user.value.username == "testuser");

        window.user.signOut();

        assert(window.user.value.username == "");
    });
});
