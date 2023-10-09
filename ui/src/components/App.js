import {component} from 'reefjs';
import {navigate} from '../store/router';

let defaults = {
    navigate: navigate,
    window: window
}

export function makeApp(config=defaults) {
    return class App extends config.window.HTMLElement {
        constructor() {
            super();
        }

        connectedCallback() {
            let events = {
                navigate: config.navigate
            }
            component(this, function() {
                return `
                    <h1><a href="/" onclick="navigate()">Wiki</a></h1>
                    <wiki-user></wiki-user>
                    <wiki-router></wiki-router>
                `;
            }, {events});
        }
    }
}
