import {component} from 'reefjs';

class WikiSubject extends HTMLElement {
    constructor() {
        super();
    }

    connectedCallback() {
        component(this, function() {
            return `
                <div>
                    <button>View</button>
                    <button>Edit</button>
                </div>
            `;
        });
    }
}
customElements.define("wiki-subject", WikiSubject);
