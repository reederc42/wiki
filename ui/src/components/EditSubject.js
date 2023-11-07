import { render } from "reefjs";
import { subjects } from "../store/subjects";
import ace from "ace-builds/src-min-noconflict/ace";
import "ace-builds/src-min-noconflict/theme-github";
import "ace-builds/src-min-noconflict/mode-markdown";

const signal = "subject-edited";
const event = new Event("wiki:signal-" + signal, {
    bubbles: true,
});

class EditSubject extends HTMLElement {
    constructor() {
        super();
    }

    connectedCallback() {
        let subject;
        this.subjectProp = this.getAttribute("subj");
        let subjectName = decodeURIComponent(this.subjectProp);
        render(
            this,
            `
            <div id="editor" style="width: 100ex; height: 82vh;">
            </div>
        `,
        );
        let editor = ace.edit(this.querySelector("#editor"));
        this.editor = editor;
        editor.getSession().setMode("ace/mode/markdown");
        editor.setTheme("ace/theme/github");

        this.updateContent = function () {
            subject = subjects.get(subjectName);
            if (subject !== undefined && subject.err === undefined) {
                editor.setValue(subject.content);
            }
        };

        this.updateContent();

        editor.on("change", () => {
            if (subject !== undefined) {
                subject.rendered = false;
                subject.synced = false;
            }
            document.dispatchEvent(event);
        });

        document.addEventListener(
            "reef:signal-" + this.subjectProp,
            this.updateContent,
        );
    }

    disconnectedCallback() {
        document.removeEventListener(
            "reef:signal-" + this.subjectProp,
            this.updateContent,
        );
    }

    getValue() {
        return this.editor.getValue();
    }
}
customElements.define("wiki-edit-subject", EditSubject);
