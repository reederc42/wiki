import { component } from "reefjs";
import {
    router,
    getSubject,
    isNew,
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
                let template;
                if (router.value.path == "/") {
                    template = "<wiki-list-subjects></wiki-list-subjects>";
                } else if (isNew()) {
                    template = "<wiki-subject new></wiki-subject>";
                } else {
                    template = `<wiki-subject subj="${getSubject()}"></wiki-subject>`;
                }
                return template;
            },
            { events: { navigate }, signals: [routerSignal] },
        );
    }

    disconnectedCallback() {
        this.component.stop();
    }
}
customElements.define("wiki-router", Router);
