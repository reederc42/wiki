import {component} from 'reefjs';
import {navigate} from '../store/router';

class App extends HTMLElement {
    constructor() {
        super();
    }

    connectedCallback() {
        let events = {navigate};
        component(this, () => {return `
            <h1><a href="/" onclick="navigate()">Wiki</a></h1>
            <wiki-user></wiki-user>
            <wiki-router></wiki-router>
        `}, {events});
    }
}
customElements.define("wiki-app", App);
