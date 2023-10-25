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
                resolve(Array.from(m.keys()).sort());
            }, timeout);
        });
    },

    getContent(subject) {
        return new Promise((resolve, reject) => {
            setTimeout(() => {
                let content = m.get(subject);
                if (content === undefined) {
                    reject(new Error("not found"));
                } else {
                    resolve(content);
                }
            }, timeout);
        });
    },
};
