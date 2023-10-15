import { component } from "reefjs";
import { route, navigate } from "../store/route";

class Router extends HTMLElement {
    constructor() {
        super();
    }

    connectedCallback() {
        component(
            this,
            function () {
                console.log(`rendering router with ${route.data.path}`);
                return `
                ${
                    route.data.path == "/"
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
