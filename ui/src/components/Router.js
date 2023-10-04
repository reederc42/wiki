import {component} from 'reefjs';
import {route, navigate} from '../store/router';

class Router extends HTMLElement {
    constructor() {
        super();
    }

    connectedCallback() {
        let events = {navigate};
        component(this, function() {
            return `
                ${route.value == "/" ? `
                    <p>Home page! Go to <a href="/abcd" onclick="navigate()">subject</a></p>
                ` : `
                    <wiki-subject></wiki-subject>
                `}
            `;
        }, {events});
    }
}
customElements.define("wiki-router", Router);
