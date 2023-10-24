import { store } from "reefjs";
import { subject as subjectAPI } from "../api/mock/subject";

export const signal = "subjects";

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
                    m.set(e[0]);
                }
            }
        },

        updateContent(m, subject, content) {
            m.set(subject, content);
        },
    },
    signal,
);

export const subjects = {
    // updateList asynchronously updates the list of subjects, emitting signal
    // and updating store on success
    updateList() {
        subjectAPI.list().then((v) => {
            subjectStore.updateList(v);
        });
    },

    // updateContent asynchrously updates the content for a subject, emtting
    // signal and updating store on success
    updateContent(subject) {
        subjectAPI
            .getContent(subject)
            .then((content) => {
                subjectStore.updateContent(subject, content);
            })
            .catch((reason) => {
                subjectStore.updateContent(subject, reason);
            });
    },

    // list returns cached list of subject names
    list() {
        return Array.from(subjectStore.value.keys());
    },

    // content returns cached subject content
    content(subject) {
        let content = subjectStore.value.get(subject);
        if (content === undefined) {
            return "";
        }
        return content;
    },
};
