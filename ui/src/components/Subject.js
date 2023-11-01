import { signal, render } from "reefjs";
import { setTitle } from "../store/title";
import { subjects as subjectStore } from "../store/subjects";

class Subject extends HTMLElement {
    constructor() {
        super();
    }

    connectedCallback() {
        let viewTab, editTab, viewer;
        let subjectProp = this.getAttribute("subj");
        let subjectName = decodeURIComponent(subjectProp);

        let subject = signal(null, subjectProp);
        subjectStore.updateContent(subjectName).then(() => {
            subject.value = subjectStore.get(subjectName);
        });

        setTitle("view", subjectName);

        // buttons start disabled until all elements can be bound to a property
        render(
            this,
            `
            <div>
                <button onclick="view()" disabled>View</button>
                <button onclick="edit()" disabled>Edit</button>
            </div>
            <div id="view">
                <wiki-view-subject subj="${subjectProp}">
                </wiki-view-subject>
            </div>
            <div id="edit" style="display: none;">
                <wiki-edit-subject subj="${subjectProp}">
                </wiki-edit-subject>
            </div>
        `,
            {
                view: () => {
                    viewTab.style.display = "inline";
                    editTab.style.display = "none";

                    setTitle("view", subjectName);
                },
                edit: () => {
                    viewTab.style.display = "none";
                    editTab.style.display = "inline";

                    setTitle("edit", subjectName);
                },
            },
        );

        viewTab = this.querySelector("#view");
        editTab = this.querySelector("#edit");
        viewer = viewTab.querySelector("wiki-view-subject");

        for (const b of this.querySelectorAll("button")) {
            b.removeAttribute("disabled");
        }
    }

    disconnectedCallback() {
        setTitle();
    }
}
customElements.define("wiki-subject", Subject);
