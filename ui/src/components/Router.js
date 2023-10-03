import {component} from 'reefjs';
import {route, navigate} from '../store/router';

class WikiRouter extends HTMLElement {
    constructor() {
        super();
    }

    connectedCallback() {
        let events = {navigate};
        component(this, function() {
            return `
                ${route.value == "/" ? `
                    <p>Home page! Go to <a to-link="/abcd" onclick="navigate()">subject</a></p>
                ` : `
                    <wiki-subject></wiki-subject>
                `}
            `;
        }, {events});
    }
}
customElements.define("wiki-router", WikiRouter);
