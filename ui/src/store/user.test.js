import { describe, beforeEach, after, test } from "node:test";
import assert from "node:assert";
import fs from "fs";
import { DOM } from "../test-helpers/dom.js";
import { waitFor } from "@testing-library/dom";

describe("user store", () => {
    let dom, window, document;

    beforeEach(() => {
        dom = new DOM();
        window = dom.window;
        document = dom.window.document;

        dom.addScript(`
            import { user, signal } from "../src/store/user";

            window.user = user;
            window.userSignal = signal;
        `);
    });

    after(async () => {
        await fs.rm("testdata", {
            recursive: true,
            force: true,
        });
    });

    test("sign out before signed in", async () => {
        let eventFired = false;
        window.addEventListener("reef:signal-" + window.userSignal, () => {
            eventFired = true;
        });
        window.user.signOut();

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

        assert(window.user.username() == "");
    });

    test("sign in and out", async () => {
        let eventFired = false;
        window.addEventListener("reef:signal-" + window.userSignal, () => {
            eventFired = true;
        });
        function assertion() {
            return eventFired;
        }

        window.user.signIn("bob", "bobpass");

        await waitFor(
            () => {
                if (!assertion()) {
                    throw new Error("waiting");
                }
            },
            { container: document },
        );
        assert(assertion());

        assert(window.user.username() == "bob");

        eventFired = false;
        window.user.signOut();

        await waitFor(
            () => {
                if (!assertion()) {
                    throw new Error("waiting");
                }
            },
            { container: document },
        );
        assert(assertion());

        assert(window.user.username() == "");
    });

    test("sign in with bad password fails", async () => {
        let err;
        window.user.signIn("bob", "not bob's pass").catch((e) => {
            err = e;
        });

        function assertion() {
            return err !== undefined && err.message == "unauthorized";
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

    test("sign up new user", async () => {
        let passed = false;
        window.user.signUp("newUser", "goodpass").then(() => {
            passed = true;
        });

        function assertion() {
            return passed;
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

    test("sign up with invalid password fails", async () => {
        let err;
        window.user.signUp("newUser", "badpass").catch((e) => {
            err = e;
        });

        function assertion() {
            return err !== undefined && err.message == "unauthorized";
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

    test("sign up existing user fails", async () => {
        let err;
        window.user.signUp("bob", "").catch((e) => {
            err = e;
        });

        function assertion() {
            return err !== undefined && err.message == "unauthorized";
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
});
