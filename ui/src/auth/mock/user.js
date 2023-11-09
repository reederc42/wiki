import * as mockUsers from "./users.json";

const timeout = 400;

let m = new Map();

for (const u of Object.getOwnPropertyNames(mockUsers)) {
    if (u != "default") {
        m.set(u, mockUsers[u]);
    }
}

export const user = {
    signIn(username, password) {
        return new Promise((resolve, reject) => {
            setTimeout(() => {
                if (password == m.get(username)) {
                    resolve(username);
                } else {
                    reject(new Error("unauthorized"));
                }
            }, timeout);
        });
    },

    signOut() {},

    signUp(username, password) {
        return new Promise((resolve, reject) => {
            setTimeout(() => {
                if (!m.has(username) && password != "badpass") {
                    resolve(username);
                } else {
                    reject(new Error("unauthorized"));
                }
            }, timeout);
        });
    },
};
