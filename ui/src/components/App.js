import { render } from "reefjs";
import { navigate } from "../store/router";

class App extends HTMLElement {
    constructor() {
        super();
    }

    connectedCallback() {
        render(
            this,
            `
            <h1>
                <a href="/" onclick="navigate()">Wiki</a>
            </h1>
            <wiki-user></wiki-user>
            <wiki-router></wiki-router>
        `,
            { navigate },
        );
    }

    disconnectedCallback() {
        this.component.stop();
    }
}
customElements.define("wiki-app", App);
