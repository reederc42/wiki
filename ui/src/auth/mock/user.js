import * as mockUsers from "./users.json";

const timeout = 400;
const userLifespan = 5 * 60 * 1000; // 5 minutes

let m = new Map();

for (const u of Object.getOwnPropertyNames(mockUsers)) {
    if (u != "default") {
        m.set(u, mockUsers[u]);
    }
}

(() => {
    console.log("getting current user");
    let username = localStorage.getItem("userName");
    if (username == null) {
        console.log("user was null");
        return;
    }

    console.log("user existed");
    let expiration = localStorage.getItem("userExpiration");
    let password = localStorage.getItem("userPass");
    console.log(username, password, expiration, Date.now() > new Date(expiration).getTime());
    if (new Date(localStorage.getItem("userExpiration")).getTime() - Date.now() > 0) {
        console.log("user sign in");
        user.signIn(username, localStorage.getItem("userPass"));
    }
})()

export const user = {
    signIn(username, password) {
        return new Promise((resolve, reject) => {
            setTimeout(() => {
                if (password == m.get(username)) {
                    localStorage.setItem("userName", username);
                    localStorage.setItem("userPass", password);
                    localStorage.setItem("userExpiration", Date.now() + userLifespan);
                    resolve(username);
                } else {
                    reject(new Error("unauthorized"));
                }
            }, timeout);
        });
    },

    signOut() {
        localStorage.clear();
    },

    signUp(username, password) {
        return new Promise((resolve, reject) => {
            setTimeout(() => {
                if (!m.has(username) && password != "badpass") {
                    m.set(username, password);
                    localStorage.setItem("userName", username);
                    localStorage.setItem("userPass", password);
                    localStorage.setItem("userExpiration", Date.now() + userLifespan);
                    resolve(username);
                } else {
                    reject(new Error("unauthorized"));
                }
            }, timeout);
        });
    },
};
