import {store, component} from 'reefjs';

class Subject extends HTMLElement {
    constructor() {
        super();
    }

    connectedCallback() {
        let viewState = store("view");
        document.title = "wiki view";
        let events = {
            view() {
                viewState.value = "view";
                document.title = "wiki view";
            },
            edit() {
                viewState.value = "edit";
                document.title = "wiki edit";
            }
        }
        component(this, function() {
            return `
                <div>
                    <button onclick="view()">View</button>
                    <button onclick="edit()">Edit</button>
                </div>
                <div id="view" ${viewState.value == "view" ? `` : `style="display: none"`}>
                    Viewing Subject
                </div>
                <div id="edit" ${viewState.value == "edit" ? `` : `style="display: none"`}>
                    <wiki-edit-subject><div id="editor" style="width: 100ex;min-height: 90vh"></div></wiki-edit-subject>
                </div>
            `;
        }, {events});
    }

    disconnectedCallback() {
        document.title = "wiki";
    }
}
customElements.define("wiki-subject", Subject);
