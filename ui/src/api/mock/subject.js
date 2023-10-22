import * as mockSubjects from "./subjects.json";

const timeout = 400;

let m = new Map();

for (const s of Object.getOwnPropertyNames(mockSubjects)) {
    if (s != "default") {
        m.set(s, mockSubjects[s]);
    }
}

export const subject = {
    list() {
        return new Promise((resolve) => {
            setTimeout(() => {
                resolve(Array.from(m.keys()));
            }, timeout);
        });
    },

    getContent(subject) {
        return new Promise((resolve) => {
            setTimeout(() => {
                resolve(m.get(subject));
            }, timeout);
        });
    },
};