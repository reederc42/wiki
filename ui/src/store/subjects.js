import { subject as subjectAPI } from "../api/mock/subject";

export const signal = "subjects";

const event = new Event("wiki:signal-" + signal, {
    bubbles: true,
});

function makeSubject(content, err) {
    let s = {
        content: "",
        rendered: false,
        synced: false,
        err: undefined,
    };
    if (content != "") {
        s.content = content;
        s.synced = true;
    }
    if (err !== undefined) {
        s.err = err;
    }
    return s;
}

const store = new Map();

function updateList(names) {
    let m = new Map(names.map((v) => [v]));
    // delete existing names that are invalid
    for (const e of store) {
        if (!m.has(e[0])) {
            store.delete(e[0]);
        }
    }
    // add new names
    for (const e of m) {
        if (!store.has(e[0])) {
            store.set(e[0], makeSubject("", undefined));
        }
    }
    document.dispatchEvent(event);
}

function updateContent(subject, content, err) {
    if (!store.has(subject)) {
        store.set(subject, makeSubject(content, err));
    } else {
        let s = store.get(subject);
        s.content = content;
        s.synced = true;
        s.err = err;
    }
    document.dispatchEvent(event);
}

export const subjects = {
    // updateList asynchronously updates the list of subjects, emitting signal
    // and updating store on success
    updateList() {
        return subjectAPI.list().then((v) => {
            updateList(v);
        });
    },

    // fetchContent asynchronously updates the content for a subject, emtting
    // signal and updating store on success
    fetchContent(subject) {
        return subjectAPI
            .get(subject)
            .then((content) => {
                updateContent(subject, content, undefined);
            })
            .catch((err) => {
                updateContent(subject, "", err);
            });
    },

    // pushContent asynchronously saves the current content to backend, emitting
    // signal and updating store on success
    pushContent(subject) {
        let s = store.get(subject);
        if (s === undefined) {
            return new Promise((resolve, reject) => {
                reject(new Error("subject does not exist"));
            });
        }
        if (s.err !== undefined) {
            return new Promise((resolve, reject) => {
                reject(s.err);
            });
        }
        return subjectAPI
            .put(subject, s.content)
            .then(() => {
                s.synced = true;
            })
            .catch((err) => {
                s.err = err;
            });
    },

    // list returns cached list of subject names
    list() {
        return Array.from(store.keys());
    },

    // get returns a reference to cached subject object
    get(subject) {
        return store.get(subject);
    },
};
