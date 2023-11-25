import { render } from "reefjs";
import { setTitle } from "../store/title";
import { inject } from "../store/inject";
import {
    subjects as subjectStore,
    Subject as StoreSubject,
} from "../store/subjects";
import { router } from "../store/router";
import { user, signal as userSignal } from "../store/user";

const errTimeout = 3000;

class Subject extends HTMLElement {
    constructor() {
        super();
    }

    connectedCallback() {
        let subjectProp = this.getAttribute("subj");
        this.subjectName =
            subjectProp != null ? decodeURIComponent(subjectProp) : "";
        this.isNew = this.hasAttribute("new");

        if (this.subjectName != "") {
            this.subject = subjectStore.get(this.subjectName);
            if (
                this.subject === undefined ||
                this.subject.content.length == 0
            ) {
                this.showFetching();
                subjectStore
                    .fetchContent(this.subjectName)
                    .then(() => {
                        if (this.isNew) {
                            // existing subject has new attribute, redirect to existing
                            router.navigate("/wiki/" + subjectProp);
                            this.isNew = false;
                        }
                        this.subject = subjectStore.get(this.subjectName);
                        this.showSubject();
                    })
                    .catch((err) => {
                        if (err.message.includes("not found") && this.isNew) {
                            this.subject = new StoreSubject();
                            this.subject.content = "# " + this.subjectName;
                            this.showSubject();
                        } else {
                            this.showError(err);
                        }
                    });
            } else {
                if (this.isNew) {
                    // existing subject has new attribute, redirect to existing
                    router.navigate("/wiki/" + subjectProp);
                    this.isNew = false;
                }
                this.showSubject();
            }
        } else {
            this.subject = new StoreSubject();
            this.subject.synced = true;
            this.showSubject();
        }
    }

    showFetching() {
        render(
            this,
            `
            <p>Fetching "${this.subjectName}"...</p>
        `,
        );
    }

    showSubject() {
        let viewTab, editTab, viewer, editor, saveError;
        let el = this;

        inject("viewer", this.subject);
        inject("editor", this.subject);

        setTitle(this.isNew ? "edit" : "view", this.subjectName);

        // buttons start disabled until all elements are bound
        render(
            this,
            `
            <div>
                <button onclick="view()" disabled>View</button>
                <button onclick="edit()" disabled>Edit</button>
                <button onclick="save()" disabled>Save</button>
                <span id="save-error" style="color: red;display: none"></span>
            </div>
            <div id="view" style="display: ${this.isNew ? "none" : "inline"};">
                <wiki-view-subject id="viewer">
                </wiki-view-subject>
            </div>
            <div id="edit" style="display: ${!this.isNew ? "none" : "inline"};">
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
                    el.saveButton.setAttribute("disabled", true);
                    editor.disable();
                    if (el.isNew) {
                        el.subjectName = editor.getTitle();
                        let err = subjectStore.create(
                            el.subjectName,
                            el.subject,
                        );
                        if (err != null) {
                            el.handleSaveError(err);
                            return;
                        }
                        el.isNew = false;
                        el.removeAttribute("new");
                    }
                    subjectStore.pushContent(el.subjectName).catch((err) => {
                        el.handleSaveError(err);
                    }).finally(() => {
                        el.updateButtons();
                    });
                },
            },
        );

        viewTab = this.querySelector("#view");
        editTab = this.querySelector("#edit");
        viewer = viewTab.querySelector("#viewer");
        editor = editTab.querySelector("#editor");
        saveError = this.querySelector("#save-error");

        // enable view and edit buttons, bind save button
        const buttons = this.querySelectorAll("button");
        buttons[0].removeAttribute("disabled");
        this.editButton = buttons[1];
        this.saveButton = buttons[2];

        this.updateButtons = () => {
            if (user.username()) {
                el.editButton.removeAttribute("disabled");
                editor.enable();

                if (!el.subject.synced) {
                    el.saveButton.removeAttribute("disabled");
                }
            } else {
                el.editButton.setAttribute("disabled", true);
                editor.disable();
            }
        };

        this.handleSaveError = (err) => {
            console.error(err);
            saveError.textContent = "Error: " + err.message;
            saveError.style.display = "inline";
            setTimeout(() => {
                saveError.textContent = "";
                saveError.style.display = "none";
            }, errTimeout);
        };

        this.updateButtons();

        document.addEventListener(
            "wiki:signal-subject-edited",
            this.updateButtons,
        );

        document.addEventListener(
            "reef:signal-" + userSignal,
            this.updateButtons,
        );
    }

    showError(err) {
        render(
            this,
            `
            <p>Could not fetch or create "${this.subjectName}": ${err}</p>
        `,
        );
    }

    disconnectedCallback() {
        document.removeEventListener(
            "wiki:signal-subject-edited",
            this.updateButtons,
        );
        document.removeEventListener(
            "reef:signal-" + userSignal,
            this.updateButtons,
        );
        setTitle();
    }
}
customElements.define("wiki-subject", Subject);
