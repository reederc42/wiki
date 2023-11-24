import { store } from "reefjs";
import { user as auth } from "../auth/mock/user";

export const signal = "user";

const localStorageKey = "user";

const userStore = store(
    {
        username: "",
        token: "",
    },
    {
        signIn(user, username, token) {
            user.username = username;
            user.token = token;
        },

        signOut(user) {
            user.username = "";
            user.token = "";
        },
    },
    signal,
);

export const user = {
    signIn(username, password, refresh = "") {
        return auth.signIn(username, password, refresh).then((v) => {
            setPersistentUser(username, v.refresh);
            userStore.signIn(username, v.token);
        });
    },

    signOut() {
        clearPersistentUser();
        auth.signOut();
        userStore.signOut();
    },

    signUp(username, password) {
        return auth.signUp(username, password).then((v) => {
            setPersistentUser(username, v.refresh);
            userStore.signIn(username, v.token);
        });
    },

    username() {
        return userStore.value.username;
    },

    token() {
        return userStore.value.token;
    },
};

function setPersistentUser(username, refresh) {
    localStorage.setItem(
        localStorageKey,
        JSON.stringify({
            username,
            refresh,
        }),
    );
}

function getPersistentUser() {
    return JSON.parse(localStorage.getItem(localStorageKey));
}

function clearPersistentUser() {
    localStorage.removeItem(localStorageKey);
}

// Check for existing user when app starts
(() => {
    let u = getPersistentUser();
    if (u) {
        user.signIn(u.username, "", u.refresh);
    }
})();
