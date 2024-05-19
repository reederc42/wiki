import { subject as subjectAPI } from "../api/mock/subject";

export const signal = "subjects";

const event = new Event("wiki:signal-" + signal, {
    bubbles: true,
});

export function Subject(content = "") {
    this.content = content;
    this.rendered = false;
    this.synced = false;
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
            store.set(e[0], new Subject());
        }
    }
    document.dispatchEvent(event);
}

function updateContent(subject, content) {
    if (!store.has(subject)) {
        let s = new Subject(content);
        s.synced = true;
        store.set(subject, s);
    } else {
        let s = store.get(subject);
        s.content = content;
        s.synced = true;
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
    fetchContent(name) {
        return subjectAPI.get(name).then((content) => {
            updateContent(name, content);
        });
    },

    // pushContent asynchronously saves the current content to backend, emitting
    // signal and updating store on success. New
    pushContent(name) {
        let s = store.get(name);
        if (s === undefined) {
            return new Promise((resolve, reject) => {
                reject(new Error("subject does not exist"));
            });
        }
        return subjectAPI.put(name, s.content).then(() => {
            s.synced = true;
        });
    },

    // list returns cached list of subject names
    list() {
        return Array.from(store.keys()).sort();
    },

    // get returns a reference to cached subject object
    get(name) {
        return store.get(name);
    },

    // create creates a new subject and returns a promise to create the subject
    // in the backend
    create(name, subject) {
        if (store.has(name)) {
            return new Promise((resolve, reject) => {
                reject(new Error("subject already exists"));
            });
        }
        return subjectAPI.post(name, subject.content).then(() => {
            subject.synced = true;
            store.set(name, subject);
        });
    },
};
