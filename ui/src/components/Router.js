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
                return `
                ${
                    route.data == "/"
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
