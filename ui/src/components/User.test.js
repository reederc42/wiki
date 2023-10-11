import { describe, after, test } from "node:test";
import assert from "node:assert";
import { DOM } from "../test-helpers/dom.js";
import * as fs from "fs";

describe("User component", () => {
    test("signed out shows signed out component", () => {
        let dom = new DOM();
        let document = dom.window.document;
        dom.addScript("./src/components/User.js");

        let wikiUser = document.createElement("wiki-user");
        document.body.appendChild(wikiUser);

        assert(wikiUser.querySelector("wiki-signed-out-user"));
    });

    test("signed in shows signed in component", () => {
        let dom = new DOM();
        let document = dom.window.document;
        dom.addScript("./src/components/User.js");

        let wikiUser = document.createElement("wiki-user");
        document.body.appendChild(wikiUser);

        let username = wikiUser.querySelector("#username");
        username.value = "user1";

        let button = wikiUser.querySelector("button");
        button.click();

        assert(wikiUser.querySelector("wiki-signed-in-user"));
    });

    after(() => {
        fs.rmSync("testdata", {
            recursive: true,
            force: true
        });
    });
});
