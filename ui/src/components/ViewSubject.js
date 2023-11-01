import { render } from "reefjs";
import { marked } from "marked";
import { subjects } from "../store/subjects";

class ViewSubject extends HTMLElement {
    constructor() {
        super();
    }

    connectedCallback() {
        this.subjectProp = this.getAttribute("subj");
        let subjectName = decodeURIComponent(this.subjectProp);
        let root = this;

        this.render = function () {
            let subject = subjects.get(subjectName);
            if (subject === undefined) {
                render(root, `Updating ${subjectName}...`);
            } else if (subject.err !== undefined) {
                render(
                    root,
                    `Error updating ${subjectName}: ${subject.err.message}`,
                );
            } else {
                console.log("rendering subject");
                console.log(subject);
                render(root, marked.parse(subject.content, { async: false }));
                subject.rendered = true;
                console.log(
                    subjects.get(subjectName) === subjects.get(subjectName),
                );
            }
        };

        this.render();

        document.addEventListener(
            "reef:signal-" + this.subjectProp,
            this.render,
        );
    }

    disconnectedCallback() {
        document.removeEventListener(
            "reef:signal-" + this.subjectProp,
            this.render,
        );
    }
}
customElements.define("wiki-view-subject", ViewSubject);
