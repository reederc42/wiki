import { component } from "reefjs";
import {
    router,
    getSubject,
    navigate,
    signal as routerSignal,
} from "../store/router";

class Router extends HTMLElement {
    constructor() {
        super();
    }

    connectedCallback() {
        this.component = component(
            this,
            function () {
                console.log(`rendering router with ${router.value.path}`);
                return `
                ${
                    router.value.path == "/"
                        ? `
                    <wiki-list-subjects></wiki-list-subjects>
                `
                        : `
                    <wiki-subject subj="${getSubject()}"></wiki-subject>
                `
                }
            `;
            },
            { events: { navigate }, signals: [routerSignal] },
        );
    }

    disconnectedCallback() {
        this.component.stop();
    }
}
customElements.define("wiki-router", Router);
