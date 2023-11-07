import { render } from "reefjs";
import { setTitle } from "../store/title";
import { inject } from "../store/inject";
import { subjects as subjectStore } from "../store/subjects";

class Subject extends HTMLElement {
    constructor() {
        super();
    }

    connectedCallback() {
        let subjectProp = this.getAttribute("subj");
        this.subjectName = decodeURIComponent(subjectProp);

        // get current subject
        this.subject = subjectStore.get(this.subjectName);

        setTitle("view", this.subjectName);

        // if subject is undefined, fetch it and display updating
        if (this.subject === undefined || this.subject.content.length == 0) {
            this.showUpdating();
            subjectStore
                .fetchContent(this.subjectName)
                .then(() => {
                    this.subject = subjectStore.get(this.subjectName);
                    this.showSubject();
                })
                .catch((err) => {
                    this.showError(err);
                });
        } else {
            this.showSubject();
        }
    }

    showUpdating() {
        render(
            this,
            `
            <p>Fetching "${this.subjectName}"...</p>
        `,
        );
    }

    showSubject() {
        let viewTab, editTab, viewer, editor;
        let el = this;

        inject("viewer", this.subject);
        inject("editor", this.subject);

        // buttons start disabled until all elements are bound
        render(
            this,
            `
            <div>
                <button onclick="view()" disabled>View</button>
                <button onclick="edit()" disabled>Edit</button>
                <button onclick="save()" disabled>Save</button>
            </div>
            <div id="view">
                <wiki-view-subject id="viewer">
                </wiki-view-subject>
            </div>
            <div id="edit" style="display: none;">
                <wiki-edit-subject id="editor">
                </wiki-edit-subject>
            </div>
        `,
            {
                view: () => {
                    viewTab.style.display = "inline";
                    editTab.style.display = "none";

                    setTitle("view", el.subjectName);

                    if (!el.subject.rendered) {
                        el.subject.content = editor.getValue();
                        viewer.render();
                    }
                },
                edit: () => {
                    viewTab.style.display = "none";
                    editTab.style.display = "inline";

                    setTitle("edit", el.subjectName);
                },
                save: () => {
                    el.subject.content = editor.getValue();
                    el.saveButton.setAttribute("disabled", null);
                    subjectStore.pushContent(el.subjectName).catch((err) => {
                        console.error(err);
                        el.saveButton.removeAttribute("disabled");
                    });
                },
            },
        );

        viewTab = this.querySelector("#view");
        editTab = this.querySelector("#edit");
        viewer = viewTab.querySelector("#viewer");
        editor = editTab.querySelector("#editor");

        // enable view and edit buttons, bind save button
        const buttons = this.querySelectorAll("button");
        buttons[0].removeAttribute("disabled");
        buttons[1].removeAttribute("disabled");
        this.saveButton = buttons[2];

        document.addEventListener(
            "wiki:signal-subject-edited",
            this.enableSave(),
        );
    }

    showError(err) {
        render(
            this,
            `
            <p>Could not fetch "${this.subjectName}": ${err}</p>
        `,
        );
    }

    enableSave() {
        let el = this;
        return function () {
            if (!el.subject.synced) {
                el.saveButton.removeAttribute("disabled");
            }
        };
    }

    disconnectedCallback() {
        document.removeEventListener(
            "wiki:signal-subject-edited",
            this.enableSave(),
        );
        setTitle();
    }
}
customElements.define("wiki-subject", Subject);
