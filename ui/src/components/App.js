import {render} from 'reefjs';

class App extends HTMLElement {
    constructor() {
        super();
    }

    connectedCallback() {
        render(this, `
            <h1>Wiki</h1>
            <wiki-user />
        `)
    }
}
customElements.define("wiki-app", App);
