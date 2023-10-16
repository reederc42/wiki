import { component } from "reefjs";
import { router, navigate } from "../store/router";

class Router extends HTMLElement {
    constructor() {
        super();
    }

    connectedCallback() {
        component(
            this,
            function () {
                console.log(`rendering router with ${router.data.path}`);
                return `
                ${
                    router.data.path == "/"
                        ? `
                    <p>Home page! Go to <a href="/wiki/abcd" onclick="navigate()">subject</a></p>
                `
                        : `
                    <wiki-subject></wiki-subject>
                `
                }
            `;
            },
            { events: { navigate } },
        );
    }
}
customElements.define("wiki-router", Router);
