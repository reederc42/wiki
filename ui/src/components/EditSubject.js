import { render } from "reefjs";
import { extract } from "../store/inject";
import { user, signal as userSignal } from "../store/user";
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
        const editorEl = this.querySelector("#ace-editor");
        this.editor = ace.edit(editorEl);
        this.editor.setTheme("ace/theme/github");
        this.editor.getSession().setMode("ace/mode/markdown");

        editorEl
            .querySelector("textarea")
            .setAttribute("name", "wiki-subject-editor");

        this.subject = extract(this.id);
        this.editor.setValue(this.subject.content);

        let el = this;
        this.editor.on("change", () => {
            el.subject.rendered = false;
            el.subject.synced = false;
            document.dispatchEvent(event);
        });

        this.updateReadonly = () => {
            el.editor.setReadOnly(!user.username());
        };
        this.updateReadonly();

        document.addEventListener(
            "reef:signal-" + userSignal,
            this.updateReadonly,
        );
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

    disconnectedCallback() {
        document.removeEventListener(
            "reef:signal-" + userSignal,
            this.updateReadonly,
        );
    }
}
customElements.define("wiki-edit-subject", EditSubject);
