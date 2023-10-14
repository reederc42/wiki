import { component } from "reefjs";
import { navigate } from "../store/route";

class App extends HTMLElement {
    constructor() {
        super();
    }

    connectedCallback() {
        component(
            this,
            function () {
                return `
                <h1>
                    <a href="/" onclick="navigate()">Wiki</a>
                </h1>
                <wiki-user></wiki-user>
                <wiki-router></wiki-router>
            `;
            },
            { events: { navigate } },
        );
    }
}
customElements.define("wiki-app", App);
