import { render } from "reefjs";
import { marked } from "marked";
import { extract } from "../store/inject";

class ViewSubject extends HTMLElement {
    constructor() {
        super();
    }

    connectedCallback() {
        this.subject = extract(this.id);
        this.render();
    }

    render() {
        render(this, marked.parse(this.subject.content));
        this.subject.rendered = true;
    }
}
customElements.define("wiki-view-subject", ViewSubject);
