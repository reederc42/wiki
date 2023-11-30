/* eslint no-unused-vars: "off" */

export const user = {
    signIn(username, password, refresh) {
        return new Promise((resolve, reject) => {
            reject(new Error("unimplemented"));
        });
    },

    signOut() {
        throw new Error("unimplemented");
    },

    signUp(username, password) {
        return new Promise((resolve, reject) => {
            reject(new Error("unimplemented"));
        });
    },
};
