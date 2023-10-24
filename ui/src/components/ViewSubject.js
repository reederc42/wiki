import { component } from "reefjs";
import { marked } from "marked";
import { subjects, signal as subjectsSignal } from "../store/subjects";

class ViewSubject extends HTMLElement {
    constructor() {
        super();
    }

    connectedCallback() {
        const subject = decodeURIComponent(this.getAttribute("subj"));
        this.component = component(
            this,
            function () {
                return marked.parse(subjects.content(subject), {
                    async: false,
                });
            },
            { signals: [subjectsSignal] },
        );
    }

    disconnectedCallback() {
        this.component.stop();
    }
}
customElements.define("wiki-view-subject", ViewSubject);
