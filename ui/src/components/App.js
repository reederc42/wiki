import { component } from "reefjs";
import { navigate } from "../store/router";

class App extends HTMLElement {
    constructor() {
        super();
    }

    connectedCallback() {
        this.component = component(
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

    disconnectedCallback() {
        this.component.stop();
    }
}
customElements.define("wiki-app", App);
