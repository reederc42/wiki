import { component } from "reefjs";
import { subjects, signal as subjectsSignal } from "../store/subjects";

class ListSubjects extends HTMLElement {
    constructor() {
        super();
    }

    connectedCallback() {
        subjects.updateList();
        component(
            this,
            function () {
                const subjectList = subjects.list();
                return `
                <table>
                    ${subjectList
                        .map((subject) => {
                            return `
                            <tr>
                                <td>
                                    <a href="/wiki/${subject}">${subject}</a>
                                </td>
                            </tr>
                        `;
                        })
                        .join("")}
                </table>
            `;
            },
            { signals: [subjectsSignal] },
        );
    }
}
customElements.define("wiki-list-subjects", ListSubjects);
