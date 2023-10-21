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
                    <wiki-list-subjects></wiki-list-subjects>
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
