import { setter } from "reefjs";

export const user = setter(
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
);
