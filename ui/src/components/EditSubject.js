import ace from "ace-builds/src-min-noconflict/ace";
// import "ace-builds/src-min-noconflict/theme-github";
// import "ace-builds/src-min-noconflict/mode-markdown";
import { subjects, signal as subjectsSignal } from "../store/subjects";

class EditSubject extends HTMLElement {
    constructor() {
        super();
    }

    connectedCallback() {
        this.subject = decodeURIComponent(this.getAttribute("subj"));
        this.editor = ace.edit(this.querySelector("#editor"));
        this.setContent();
        window.addEventListener("reef:signal-" + subjectsSignal, () => {
            this.setContent()
        });
    }

    setContent() {
        let content = subjects.content(this.subject);
        this.editor.setValue(content, 1);
    }
}
customElements.define("wiki-edit-subject", EditSubject);
