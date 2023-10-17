import { signal, component } from "reefjs";
import { setView } from "../store/title";

const viewView = "view";
const viewEdit = "edit";
const viewSignal = "view";

class Subject extends HTMLElement {
    constructor() {
        super();
    }

    connectedCallback() {
        let view = signal(viewView, viewSignal);
        setView(view.value);
        let events = {
            view() {
                view.value = viewView;
                setView(viewView);
            },
            edit() {
                view.value = viewEdit;
                setView(viewEdit);
            },
        };
        component(
            this,
            function () {
                return `
                <div>
                    <button onclick="view()">View</button>
                    <button onclick="edit()">Edit</button>
                </div>
                <div id="view" ${
                    view.value == viewView
                        ? ``
                        : `style="display: none"`
                }>
                    Viewing Subject
                </div>
                <div id="edit" ${
                    view.value == viewEdit
                        ? ``
                        : `style="display: none"`
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
        setView("");
    }
}
customElements.define("wiki-subject", Subject);
