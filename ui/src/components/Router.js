import { component } from "reefjs";
import { router, navigate, signal as routerSignal } from "../store/router";

class Router extends HTMLElement {
    constructor() {
        super();
    }

    connectedCallback() {
        component(
            this,
            function () {
                console.log(`rendering router with ${router.value.path}`);
                return `
                ${
                    router.value.path == "/"
                        ? `
                    <p>Home page! Go to <a href="/wiki/abcd" onclick="navigate()">subject</a></p>
                `
                        : `
                    <wiki-subject></wiki-subject>
                `
                }
            `;
            },
            { events: { navigate }, signals: [routerSignal] },
        );
    }
}
customElements.define("wiki-router", Router);
