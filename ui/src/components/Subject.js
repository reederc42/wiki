import {store, component} from 'reefjs';
import {setView} from '../store/title';

const viewSubject = "view";
const editSubject = "edit";

class Subject extends HTMLElement {
    constructor() {
        super();
    }

    connectedCallback() {
        let subjectState = store(viewSubject);
        setView(subjectState.value);
        let events = {
            view() {
                subjectState.value = viewSubject;
                setView(viewSubject);
            },
            edit() {
                subjectState.value = editSubject;
                setView(editSubject);
            }
        }
        component(this, function() {
            return `
                <div>
                    <button onclick="view()">View</button>
                    <button onclick="edit()">Edit</button>
                </div>
                <div id="view" ${subjectState.value == viewSubject ? `` : `style="display: none"`}>
                    Viewing Subject
                </div>
                <div id="edit" ${subjectState.value == editSubject ? `` : `style="display: none"`}>
                    <wiki-edit-subject>
                        <div id="editor" style="width: 100ex;min-height: 82vh"></div>
                    </wiki-edit-subject>
                </div>
            `;
        }, {events});
    }

    disconnectedCallback() {
        setView("");
    }
}
customElements.define("wiki-subject", Subject);