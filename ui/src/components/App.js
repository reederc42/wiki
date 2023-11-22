import { render } from "reefjs";
import { router, navigate } from "../store/router";

class App extends HTMLElement {
    constructor() {
        super();
    }

    connectedCallback() {
        router.navigate(location.href.substring(location.origin.length));
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
}
customElements.define("wiki-app", App);
