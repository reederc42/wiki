import config from "../../config.json";
import mockUsers from "./users.json";

const timeout = 400;

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
                if (refresh != "") {
                    let u = JSON.parse(atob(refresh));
                    if (u.expiration < Date.now()) {
                        reject(new Error("unauthorized"));
                        return;
                    }
                    if (!m.has(u.username)) {
                        m.set(u.username, u.password);
                    }
                    password = u.password;
                }
                if (password == m.get(username)) {
                    resolve({
                        token: token(
                            username,
                            password,
                            Date.now() + config.apiExpiration,
                        ),
                        refresh: token(
                            username,
                            password,
                            Date.now() + config.userExpiration,
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
                        token: token(
                            username,
                            password,
                            Date.now() + config.apiExpiration,
                        ),
                        refresh: token(
                            username,
                            password,
                            Date.now() + config.userExpiration,
                        ),
                    });
                } else {
                    reject(new Error("unauthorized"));
                }
            }, timeout);
        });
    },
};

export function token(username, password, expiration) {
    return btoa(
        JSON.stringify({
            username,
            password,
            expiration,
        }),
    );
}
