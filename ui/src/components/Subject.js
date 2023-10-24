import { signal, component } from "reefjs";
import { setTitle } from "../store/title";
import { subjects } from "../store/subjects";

const viewView = "view";
const viewEdit = "edit";
const viewSignal = "view";

class Subject extends HTMLElement {
    constructor() {
        super();
    }

    connectedCallback() {
        let subject = decodeURIComponent(this.getAttribute("subj"));
        subjects.updateContent(subject);
        let view = signal(viewView, viewSignal);
        setTitle(viewView, subject);
        let events = {
            view() {
                view.value = viewView;
                setTitle(viewView, subject);
            },
            edit() {
                view.value = viewEdit;
                setTitle(viewEdit, subject);
            },
        };
        this.component = component(
            this,
            function () {
                return `
                <div>
                    <button onclick="view()">View</button>
                    <button onclick="edit()">Edit</button>
                </div>
                <div id="view" ${
                    view.value == viewView ? `` : `style="display: none"`
                }>
                    <wiki-view-subject subj=${encodeURIComponent(subject)}>
                    </wiki-view-subject>
                </div>
                <div id="edit" ${
                    view.value == viewEdit ? `` : `style="display: none"`
                }>
                    <wiki-edit-subject>
                        <div id="editor" style="width: 100ex;min-height: 82vh"></div>
                    </wiki-edit-subject>
                </div>
            `;
            },
            { events, signals: [viewSignal] },
        );
    }

    disconnectedCallback() {
        setTitle();
        this.component.stop();
    }
}
customElements.define("wiki-subject", Subject);
