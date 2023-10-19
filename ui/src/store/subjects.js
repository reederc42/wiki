import { store } from "reefjs";

export const signal = "subjects";

const subjectStore = store(new Map(), {
    updateList(m, names) {
        const n = new Map(names.map((v) => [v, null]));
        // delete existing names that are invalid
        for (const name of m) {
            if (!n.has(name)) {
                m.delete(name);
            }
        }
        // add new names to m
        for (const name of n) {
            if (!m.has(name)) {
                m.set(name);
            }
        }
    },
}, signal);

export const subjects = {
    // updateList asynchronously updates the list of subjects, emitting signal
    // and updating store on success
    updateList() {
        subjectStore.updateList(["one", "two", "three"]);
    },
    // updateContent asynchrously updates the content for a subject, emtting
    // signal and updating store on success
    updateContent(subject) {},
    forEach(fn) {

    },
    // content returns cached subject content
    content(subject) {},
}
