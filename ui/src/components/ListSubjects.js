import { signal, component } from "reefjs";
import { subjects, signal as subjectsSignal } from "../store/subjects";
import { navigate } from "../store/router";

const errorSignal = "list-subjects-error";

class ListSubjects extends HTMLElement {
    constructor() {
        super();
    }

    connectedCallback() {
        let err = signal(undefined, errorSignal);
        subjects.updateList().catch((error) => {
            err.value = error;
        });
        this.component = component(
            this,
            function () {
                if (err.value != undefined) {
                    return `Error listing subjects: ${err.value.message}`;
                }
                const subjectList = subjects.list();
                return subjectsTable(subjectList);
            },
            { events: { navigate }, signals: [subjectsSignal, errorSignal] },
        );
    }

    disconnectedCallback() {
        this.component.stop();
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
