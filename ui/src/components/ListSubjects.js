import { component } from "reefjs";
import { subjects, signal as subjectsSignal } from "../store/subjects";
import { navigate } from "../store/router";

class ListSubjects extends HTMLElement {
    constructor() {
        super();
    }

    connectedCallback() {
        console.log("connected");
        subjects.updateList();
        this.component = component(
            this,
            function () {
                const subjectList = subjects.list();
                return subjectsTable(subjectList);
            },
            { events: { navigate }, signals: [subjectsSignal] },
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
