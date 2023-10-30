import { store } from "reefjs";
import { subject as subjectAPI } from "../api/mock/subject";

export const signal = "subjects";

function makeSubject(content) {
    let s = {
        content: "",
        rendered: false,
        synced: false,
    }
    if (content != "") {
        s.content = content
        s.synced = true
    }
    return s
}

const subjectStore = store(
    new Map(),
    {
        updateList(m, names) {
            let n = new Map(names.map((v) => [v]));
            // delete existing names that are invalid
            for (const e of m) {
                if (!n.has(e[0])) {
                    m.delete(e[0]);
                }
            }
            // add new names to m
            for (const e of n) {
                if (!m.has(e[0])) {
                    m.set(e[0], makeSubject(""));
                }
            }
        },

        updateContent(m, subject, content) {
            if (!m.has(subject)) {
                m.set(subject, makeSubject(content));
            } else {
                let s = m.get(subject);
                s.content = content;
                s.synced = true;
            }
        },
    },
    signal,
);

export const subjects = {
    // updateList asynchronously updates the list of subjects, emitting signal
    // and updating store on success
    updateList() {
        return subjectAPI.list().then((v) => {
            subjectStore.updateList(v);
        });
    },

    // updateContent asynchrously updates the content for a subject, emtting
    // signal and updating store on success
    updateContent(subject) {
        return subjectAPI.get(subject).then((content) => {
            subjectStore.updateContent(subject, content);
        });
    },

    // list returns cached list of subject names
    list() {
        return Array.from(subjectStore.value.keys());
    },

    // get returns a reference to cached subject object
    get(subject) {
        return subjectStore.value.get(subject);
    },
};
