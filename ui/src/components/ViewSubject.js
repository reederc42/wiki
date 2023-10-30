import { signal, component } from "reefjs";
import { marked } from "marked";
import { subjects, signal as subjectsSignal } from "../store/subjects";

const errorSignal = "view-subject-error";

class ViewSubject extends HTMLElement {
    constructor() {
        super();
    }

    connectedCallback() {
        const subject = decodeURIComponent(this.getAttribute("subj"));
        const err = signal(undefined, errorSignal);
        subjects.updateContent(subject).catch((error) => {
            err.value = error;
        });
        this.component = component(
            this,
            function () {
                if (err.value !== undefined) {
                    return `Error getting '${subject}': ${err.value.message}`;
                }
                return marked.parse(subjects.get(subject).content, {
                    async: false,
                });
            },
            { signals: [subjectsSignal, errorSignal] },
        );
    }

    disconnectedCallback() {
        this.component.stop();
    }
}
customElements.define("wiki-view-subject", ViewSubject);
