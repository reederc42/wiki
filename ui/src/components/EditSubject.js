import ace from "ace-builds/src-min-noconflict/ace";
import "ace-builds/src-min-noconflict/theme-github";
import "ace-builds/src-min-noconflict/mode-markdown";

class EditSubject extends HTMLElement {
    constructor() {
        super();
    }

    connectedCallback() {
        let editor = this.querySelector("#editor");
        ace.edit(editor);
    }
}
customElements.define("wiki-edit-subject", EditSubject);
