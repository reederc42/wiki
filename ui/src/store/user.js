import { store } from "reefjs";

export const signal = "user";

export const user = store(
    { username: "" },
    {
        signIn(user, username) {
            console.log(`signed in ${username}`);
            user.username = username;
        },
        signOut(user) {
            console.log(`signed out ${user.username}`);
            user.username = "";
        },
    },
    signal,
);
