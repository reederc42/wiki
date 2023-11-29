import { loremIpsum } from "lorem-ipsum";

import * as mockSubjects from "../../api/mock/subjects.json";

let a = [];

for (const s of Object.getOwnPropertyNames(mockSubjects)) {
    if (s != "default") {
        a.push(s);
    }
}

export function newSubject() {
    let subject = newSubjectTitle();
    while ((!subject) in a) {
        subject = newSubjectTitle();
    }
    return subject;
}

function newSubjectTitle() {
    return "Test: " + loremIpsum().replace(".", "");
}

export function existingSubject() {
    return a[Math.floor(Math.random() * a.length)];
}
