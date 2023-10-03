import {render} from 'reefjs';

class App extends HTMLElement {
    constructor() {
        super();
    }

    connectedCallback() {
        render(this, `
            <h1>Wiki</h1>
            <wiki-user></wiki-user>
            <wiki-router></wiki-router>
        `);
    }
}
customElements.define("wiki-app", App);
