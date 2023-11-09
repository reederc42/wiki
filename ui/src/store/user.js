import { store } from "reefjs";
import { user as auth } from "../auth/mock/user";

export const signal = "user";

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
    signIn(username, password) {
        return auth.signIn(username, password).then((token) => {
            userStore.signIn(username, token);
        });
    },

    signOut() {
        auth.signOut();
        userStore.signOut();
    },

    signUp(username, password) {
        return auth.signUp(username, password).then((token) => {
            userStore.signIn(username, token);
        });
    },

    username() {
        return userStore.value.username;
    },

    token() {
        return userStore.value.token;
    },
};
