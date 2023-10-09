import {component} from 'reefjs';
import {route as defaultRoute, navigate as defaultNavigate} from '../store/router';

export function makeRouter(elementName="wiki-router", route=defaultRoute, navigate=defaultNavigate) {
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
}
