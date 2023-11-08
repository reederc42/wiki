import { render } from "reefjs";
import { extract } from "../store/inject";
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
        render(
            this,
            `
            <div id="ace-editor" style="width: 100ex; height: 82vh;">
            </div>
        `,
        );
        this.editor = ace.edit(this.querySelector("#ace-editor"));
        this.editor.setTheme("ace/theme/github");
        this.editor.getSession().setMode("ace/mode/markdown");

        this.subject = extract(this.id);
        this.editor.setValue(this.subject.content);

        let el = this;
        this.editor.on("change", () => {
            el.subject.rendered = false;
            el.subject.synced = false;
            document.dispatchEvent(event);
        });
    }

    getValue() {
        return this.editor.getValue();
    }

    getTitle() {
        return this.editor
            .getValue()
            .split("\n")[0]
            .replace(/^#?\s+/, "");
    }
}
customElements.define("wiki-edit-subject", EditSubject);
