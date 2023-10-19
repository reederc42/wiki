const timeout = 400;

export const subject = {
    list() {
        return new Promise((resolve) => {
            setTimeout(() => {
                resolve(["one", "two", "three"]);
            }, timeout);
        });
    },

    getContent(subject) {
        return new Promise((resolve) => {
            setTimeout(() => {
                resolve("some content");
            }, timeout);
        });
    }
}
