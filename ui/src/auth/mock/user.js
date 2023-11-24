import * as mockUsers from "./users.json";

const timeout = 400;
const userLifespan = 5 * 60 * 1000; // 5 minutes

let m = new Map();

for (const u of Object.getOwnPropertyNames(mockUsers)) {
    if (u != "default") {
        m.set(u, mockUsers[u]);
    }
}

export const user = {
    signIn(username, password, refresh) {
        return new Promise((resolve, reject) => {
            setTimeout(() => {
                if (password == m.get(username)) {
                    resolve({
                        token: "",
                        refresh: refreshToken(
                            username,
                            password,
                            Date.now() + userLifespan,
                        ),
                    });
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
                    m.set(username, password);
                    resolve({
                        token: "",
                        refresh: refreshToken(
                            username,
                            password,
                            Date.now() + userLifespan,
                        ),
                    });
                } else {
                    reject(new Error("unauthorized"));
                }
            }, timeout);
        });
    },
};

export function refreshToken(username, password, expiration) {
    return btoa(
        JSON.stringify({
            username,
            password,
            expiration,
        }),
    );
}
