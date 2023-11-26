import { describe, beforeEach, test } from "node:test";
import assert from "node:assert";
import { DOM } from "../test-helpers/dom.js";
import { waitFor } from "../test-helpers/waitFor.js";

describe("user store", () => {
    let dom, window, document;

    beforeEach(() => {
        dom = new DOM();
        window = dom.window;
        document = dom.window.document;

        dom.addScript(`
            import { user, signal } from "./src/store/user";

            window.user = user;
            window.userSignal = signal;
        `);
    });

    test("sign out before signed in", async () => {
        let eventFired = false;
        window.addEventListener("reef:signal-" + window.userSignal, () => {
            eventFired = true;
        });
        window.user.signOut();

        await waitFor(() => eventFired, document);

        assert(window.user.username() == "");
    });

    ["in", "up"].forEach((method) => {
        test(`sign ${method} and out`, async () => {
            let eventFired = false;
            window.addEventListener("reef:signal-" + window.userSignal, () => {
                eventFired = true;
            });

            let user, pass;
            if (method == "in") {
                user = "bob";
                pass = "bobpass";
                window.user.signIn(user, pass);
            } else if (method == "up") {
                user = "newUser";
                pass = "goodpass";
                window.user.signUp(user, pass);
            }

            await waitFor(() => eventFired, document);

            assert(window.user.username() == user);
            assert(window.localStorage.getItem("user"));

            eventFired = false;
            window.user.signOut();

            await waitFor(() => eventFired, document);

            assert(window.user.username() == "");
            assert(!window.localStorage.getItem("user"));
        });
    });

    test("sign in with bad password fails", async () => {
        let err;
        window.user.signIn("bob", "not bob's pass").catch((e) => {
            err = e;
        });

        await waitFor(() => {
            return err !== undefined && err.message == "unauthorized";
        }, document);
    });

    test("sign up new user", async () => {
        let passed = false;
        window.user.signUp("newUser", "goodpass").then(() => {
            passed = true;
        });

        await waitFor(() => passed, document);
    });

    test("sign up with invalid password fails", async () => {
        let err;
        window.user.signUp("newUser", "badpass").catch((e) => {
            err = e;
        });

        await waitFor(() => {
            return err !== undefined && err.message == "unauthorized";
        }, document);
    });

    test("sign up existing user fails", async () => {
        let err;
        window.user.signUp("bob", "").catch((e) => {
            err = e;
        });

        await waitFor(() => {
            return err !== undefined && err.message == "unauthorized";
        }, document);
    });
});
