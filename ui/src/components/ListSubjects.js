import { signal, render } from "reefjs";

import { navigate } from "../store/router";
import { subjects, signal as subjectsSignal } from "../store/subjects";

const errorSignal = "list-subjects-error";

class ListSubjects extends HTMLElement {
    constructor() {
        super();
    }

    connectedCallback() {
        let root = this;
        let err = signal(undefined, errorSignal);
        subjects.updateList().catch((error) => {
            err.value = error;
        });

        this.render = function () {
            function template() {
                if (err.value !== undefined) {
                    return `Error listing subjects: ${err.value.message}`;
                }
                const subjectList = subjects.list();
                return (
                    `<div><a href="/wiki-new" onclick="navigate()">new</a></div><div>` +
                    subjectsTable(subjectList) +
                    "</div>"
                );
            }
            render(root, template(), { navigate });
        };

        this.render();

        document.addEventListener("wiki:signal-" + subjectsSignal, this.render);
    }

    disconnectedCallback() {
        document.removeEventListener(
            "wiki:signal-" + subjectsSignal,
            this.render,
        );
    }
}
customElements.define("wiki-list-subjects", ListSubjects);

const columns = 3;

function subjectsTable(subjectList) {
    let table = `<table style="width: 100ex">`;
    let rowCount = Math.ceil(subjectList.length / columns);
    for (let r = 0; r < rowCount; r++) {
        table += subjectsRow(subjectList, r, rowCount);
    }
    return table + "</table>";
}

function subjectsRow(subjectList, row, rowCount) {
    let rowString = "<tr>";
    for (let c = 0; c < columns; c++) {
        let i = row + c * rowCount;
        rowString += `<td>${
            i < subjectList.length
                ? `<a href="/wiki/${encodeURIComponent(
                      subjectList[i],
                  )}" onclick="navigate()">${subjectList[i]}</a>`
                : ""
        }</td>`;
    }
    return rowString + "</td>";
}
